//! Module with helper structs with methods to work with common sequence types.

mod nucleotide;
mod quality;

use std::ops::{Deref, DerefMut};

pub use nucleotide::NucleotideView;
pub use quality::{PhredQualityEncoding, QualityView};

use bstr::{BStr, BString};

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub struct Sequence(BString);

/// Sequence creation functions
impl Sequence {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(BString::new(bytes))
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
