//! Sequence alphabets module

pub mod classifier;

pub mod aminoacid;
pub mod dna;
pub mod rna;

use bstr::BString;

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
}
