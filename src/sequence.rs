//! Module with helper structs with methods to work with common sequence types.

mod nucleotide;

use std::ops::{Deref, DerefMut};

use bstr::{BStr, BString};

pub use nucleotide::NucleotideView;
use thiserror::Error;

const fn build_complement_lut<const X: usize>(
    sequence: &[u8; X],
    complement: &[u8; X],
) -> [u8; 256] {
    let mut lut = [0; 256];

    // 1. Initialize with identity mapping
    let mut i = 0;
    while i < 256 {
        lut[i] = i as u8;
        i += 1;
    }

    // 2. Map the specific complement pairs
    let mut j = 0;
    while j < X {
        let base = sequence[j];
        let comp = complement[j];

        lut[base as usize] = comp;

        j += 1;
    }

    lut
}

#[derive(Debug, Error)]
pub enum SequenceOperationError {
    #[error("can't calculate distance between two sequences with different lengths")]
    DifferentLengths,
}

/// A struct representing a biological sequence.
/// It has utility methods to work with K-mers.
#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Sequence(BString);

// Sequence creation functions
impl Sequence {
    /// Creates an empty sequence without allocating any memory.
    pub fn new() -> Self {
        Self(BString::new(Vec::new()))
    }

    /// Creates an empty sequence with at least the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(BString::new(Vec::with_capacity(capacity)))
    }
}

// Sequence utility methods
impl Sequence {
    /// Returns the Hamming distance between this sequence and another one.
    ///
    /// Returns a `SequenceOperationError::DifferentLengths` error if the sequences have different lengths.
    pub fn hamming_distance(&self, other: &Sequence) -> Result<usize, SequenceOperationError> {
        if self.len() != other.len() {
            return Err(SequenceOperationError::DifferentLengths);
        }

        Ok(self
            .iter()
            .zip(other.iter())
            .filter(|&(&b, &v)| b != v)
            .count())
    }

    /// Returns the p-distance between this sequence and another one.
    /// ///
    /// Returns a `SequenceOperationError::DifferentLengths` error if the sequences have different lengths.
    pub fn p_distance(&self, other: &Sequence) -> Result<f32, SequenceOperationError> {
        if self.len() != other.len() {
            return Err(SequenceOperationError::DifferentLengths);
        }

        Ok((self
            .iter()
            .zip(other.iter())
            .filter(|&(&b, &v)| b != v)
            .count() as f64
            / self.len() as f64) as f32)
    }

    /// Returns a `slice::Windows` iterator over the sequence kmers of size `k`.
    pub fn kmers(&self, k: usize) -> impl Iterator<Item = &BStr> {
        self.0.windows(k).map(BStr::new)
    }

    /// Returns the number of appearances of `kmer` inside the sequence.
    pub fn count_kmer<K>(&self, kmer: K) -> usize
    where
        K: AsRef<[u8]>,
    {
        let kmer = kmer.as_ref();
        let kmers_iter = self.kmers(kmer.len());

        kmers_iter.filter(|&k| k == kmer).count()
    }
}

impl Deref for Sequence {
    type Target = BString;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Sequence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PartialEq<&str> for Sequence {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<str> for Sequence {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<Sequence> for &str {
    fn eq(&self, other: &Sequence) -> bool {
        *self == other.0
    }
}

impl<'s> From<&'s str> for Sequence {
    fn from(value: &'s str) -> Self {
        Self(value.into())
    }
}

impl From<String> for Sequence {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl From<BString> for Sequence {
    fn from(value: BString) -> Self {
        Self(value)
    }
}

impl From<Vec<u8>> for Sequence {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}

impl AsRef<[u8]> for Sequence {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<BStr> for Sequence {
    fn as_ref(&self) -> &BStr {
        self.0.as_ref()
    }
}

impl From<Sequence> for BString {
    fn from(s: Sequence) -> Self {
        s.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let first_seq = Sequence::from("ACGTGTCAN");
        let second_seq = Sequence::from("ACGNGNCAN");

        assert_eq!(
            first_seq
                .hamming_distance(&second_seq)
                .expect("both sequences have the same length"),
            2
        );
        assert_eq!(
            first_seq.p_distance(&second_seq).unwrap(),
            (2f64 / first_seq.len() as f64) as f32
        );
    }

    #[test]
    fn test_kmers() {
        let kmers_vec: Vec<BString> = Sequence::from("CGTGAGTC")
            .kmers(3)
            .map(ToOwned::to_owned)
            .collect();

        assert_eq!(kmers_vec, vec!["CGT", "GTG", "TGA", "GAG", "AGT", "GTC"]);
    }

    #[test]
    fn test_count_kmer() {
        let seq = Sequence::from("AGTCTAGGATAGTTCGAG");

        assert_eq!(seq.count_kmer("AG"), 4);
        assert_eq!(seq.count_kmer("GT"), 2);

        assert_eq!(seq.count_kmer("TAG"), 2);
    }
}
