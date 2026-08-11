//! Sequence alphabets module

pub mod aminoacid;
pub mod dna;
pub mod rna;

use bstr::BString;
use thiserror::Error;

/// An alphabet made from a set of symbols, it generates a look up table on creation.
#[derive(Debug, PartialEq, Clone, Hash)]
pub struct Alphabet {
    symbols: BString,
    lut: [bool; 256],
}

// Alphabet creation methods
impl Alphabet {
    /// Creates a new alphabet from a set of symbols. It is case insensitive, meaning that
    /// including `A` also includes `a` as a valid symbol. Use `Alphabet::new_cs()` to
    /// create case sensitive alphabet.
    pub fn new<S>(characters: S) -> Self
    where
        S: Into<Vec<u8>>,
    {
        let (dedup_characters, lut) = Alphabet::make_lut(characters, false);
        Self {
            symbols: BString::from(dedup_characters),
            lut,
        }
    }

    /// Creates a new alphabet from a set of symbols. It is case sensitive, meaning that
    /// including `A` doesn't include `a` as a valid symbol.
    pub fn new_cs<S>(characters: S) -> Self
    where
        S: Into<Vec<u8>>,
    {
        let (dedup_characters, lut) = Alphabet::make_lut(characters, true);
        Self {
            symbols: BString::from(dedup_characters),
            lut,
        }
    }

    fn make_lut<C>(characters: C, cs: bool) -> (Vec<u8>, [bool; 256])
    where
        C: Into<Vec<u8>>,
    {
        let mut characters = characters.into();
        characters.sort_unstable();
        characters.dedup();

        let mut lut = [false; 256];
        for &b in &characters {
            if cs {
                lut[b as usize] = true;
            } else {
                lut[b.to_ascii_lowercase() as usize] = true;
                lut[b.to_ascii_uppercase() as usize] = true;
            }
        }

        (characters, lut)
    }

    pub fn validate_sequence<S>(&self, sequence: S) -> bool
    where
        S: AsRef<[u8]>,
    {
        sequence.as_ref().iter().all(|&b| self.lut[b as usize])
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AlphabetType {
    Exact(String),
    Ambiguous(Vec<String>),
    Unknown,
}

// Alphabet classifier error
#[derive(Error, Debug)]
pub enum ClassifierError {
    #[error("Maximum number of alphabets in the classifier exceeded")]
    CapacityExceeded,

    #[error("alphabet with name '{0}' already exists")]
    DuplicateName(String),
}

pub struct Classifier<U> {
    lut: [U; 256],
    names: Vec<String>,
}

impl Classifier<u8> {
    pub fn new() -> Self {
        Self {
            lut: [0u8; 256],
            names: Vec::new(),
        }
    }

    pub fn add_alphabet<N>(&mut self, name: N, alphabet: &Alphabet) -> Result<(), ClassifierError>
    where
        N: Into<String>,
    {
        let name = name.into();
        if self.names.iter().any(|n| n == &name) {
            return Err(ClassifierError::DuplicateName(name));
        }

        let bitflag_idx = self.names.len();
        if bitflag_idx >= u8::BITS as usize {
            return Err(ClassifierError::CapacityExceeded);
        }

        self.names.push(name);
        let mask = 1 << bitflag_idx;

        for &byte in alphabet.symbols.iter() {
            self.lut[byte.to_ascii_uppercase() as usize] |= mask;
            self.lut[byte.to_ascii_lowercase() as usize] |= mask;
        }

        Ok(())
    }

    pub fn classify<S>(&self, sequence: S) -> AlphabetType
    where
        S: AsRef<[u8]>,
    {
        let sequence = sequence.as_ref();

        if self.names.is_empty() || sequence.is_empty() {
            return AlphabetType::Unknown;
        }

        let mut active_mask: u8 = (1 << self.names.len()) - 1;

        for &byte in sequence {
            active_mask &= self.lut[byte as usize];

            if active_mask == 0 {
                return AlphabetType::Unknown;
            }
        }

        self.resolve_mask(active_mask)
    }

    fn resolve_mask(&self, mask: u8) -> AlphabetType {
        let num_matches = mask.count_ones();

        if num_matches == 1 {
            let bit_idx = mask.trailing_zeros() as usize;
            AlphabetType::Exact(self.names[bit_idx].clone())
        } else {
            let mut ambiguous = Vec::with_capacity(num_matches as usize);
            for i in 0..self.names.len() {
                if (mask & (1 << i)) != 0 {
                    ambiguous.push(self.names[i].clone());
                }
            }
            AlphabetType::Ambiguous(ambiguous)
        }
    }
}

impl Default for Classifier<u8> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alphabet() {
        let dna_alphabet = Alphabet::new(b"ACGT");
        let rna_alphabet = Alphabet::new(b"ACGU");

        assert!(dna_alphabet.validate_sequence("ACGTGTACGTA"));
        assert!(!dna_alphabet.validate_sequence("ACGUUCGA"));

        assert!(rna_alphabet.validate_sequence("ACGUUCGA"));
        assert!(!rna_alphabet.validate_sequence("ACGTGTACGTA"));
    }

    #[test]
    fn test_classifier() {
        let mut dna_rna_classifier = Classifier::new();

        let dna_alphabet: Alphabet = Alphabet::new(b"ACGT");
        let rna_alphabet = Alphabet::new(b"ACGU");

        dna_rna_classifier
            .add_alphabet("DNA", &dna_alphabet)
            .expect("We are not exceeding the bitmask size");
        dna_rna_classifier
            .add_alphabet("RNA", &rna_alphabet)
            .expect("We are not exceeding the bitmask size");

        assert_eq!(
            dna_rna_classifier.classify("ACGT"),
            AlphabetType::Exact("DNA".into())
        );
        assert_eq!(
            dna_rna_classifier.classify("ACGU"),
            AlphabetType::Exact("RNA".into())
        );

        assert_eq!(
            dna_rna_classifier.classify("ACG"),
            AlphabetType::Ambiguous(vec!["DNA".into(), "RNA".into()])
        );
    }
}
