//! Module with helper structs with methods to work with common sequence types.

mod nucleotide;
mod quality;

use std::ops::{Deref, DerefMut};

use bstr::{BStr, BString};

pub use nucleotide::NucleotideView;
pub use quality::{PhredQualityEncoding, QualityView};

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Sequence(BString);

/// Sequence creation functions
impl Sequence {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(BString::new(bytes))
    }
}

/// Sequence utility methods
impl Sequence {
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
        Self::new(value.into())
    }
}

impl From<String> for Sequence {
    fn from(value: String) -> Self {
        Self::new(value.into())
    }
}

impl From<BString> for Sequence {
    fn from(b: BString) -> Self {
        Self(b)
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
