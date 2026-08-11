//! Nucleotide sequences module

mod dna;
mod rna;

use thiserror::Error;

use crate::alphabet::dna::DNA_IUPAC;
use crate::alphabet::rna::RNA_IUPAC;

use dna::{DnaView, DnaViewError};
use rna::{RnaView, RnaViewError};

use super::Sequence;

/// Error type for operations over NucleotideView
#[derive(Error, Debug)]
pub enum NucleotideViewError {
    #[error("the supplied sequence is not valid for the DNA-IUPAC/RNA-IUPAC alphabet")]
    InvalidSequence,
}

/// A struct representing a view to a DNA-IUPAC or RNA-IUPAC sequence.
/// Contains useful methods to calculate the GC contents and symbol counts.
#[derive(Debug)]
pub struct NucleotideView<'s> {
    inner: &'s Sequence,
}

// NucleotideView creation functions
impl<'s> NucleotideView<'s> {
    /// Checks for the sequence to be a valid DNA-IUPAC of RNA-IUPAC sequence.
    pub fn try_new(sequence: &'s Sequence) -> Result<Self, NucleotideViewError> {
        if DNA_IUPAC.validate_sequence(sequence) || RNA_IUPAC.validate_sequence(sequence) {
            Ok(Self { inner: sequence })
        } else {
            Err(NucleotideViewError::InvalidSequence)
        }
    }

    /// Creates the view without checking for a valid DNA IUPAC of RNA IUPAC sequence.
    pub fn new_unchecked(sequence: &'s Sequence) -> Self {
        Self { inner: sequence }
    }
}

// NucleotideView utility methods
impl<'s> NucleotideView<'s> {
    /// Returns the GC percentage of the view.
    pub fn gc_percentage<S>(&self, gap_symbols: S) -> f32
    where
        S: AsRef<[u8]>,
    {
        let gap_symbols = gap_symbols.as_ref();

        let mut gap_lut = [false; 256];
        for &g in gap_symbols {
            gap_lut[g as usize] = true;
        }

        let mut gc_total = 0;
        let mut gap_total = 0;

        for &b in self.inner.iter() {
            if matches!(b, b'G' | b'C' | b'g' | b'c') {
                gc_total += 1;
            } else if gap_lut[b as usize] {
                gap_total += 1;
            }
        }

        let nucleotide_len = self.inner.len() - gap_total;

        if nucleotide_len == 0 {
            0f32
        } else {
            ((gc_total as f64 * 100f64) / nucleotide_len as f64) as f32
        }
    }

    /// Returns the count of G and C bases inside the view. Case **insensitive**.
    #[inline]
    pub fn gc_count(&self) -> usize {
        self.inner
            .iter()
            .filter(|&&b| matches!(b, b'G' | b'C' | b'g' | b'c'))
            .count()
    }

    /// Counts the occurrences of a symbol inside the view. Case **sensitive**.
    #[inline]
    pub fn symbols_count<S>(&self, symbols: S) -> usize
    where
        S: AsRef<[u8]>,
    {
        let symbols = symbols.as_ref();

        let mut lut = [false; 256];
        for &g in symbols {
            lut[g as usize] = true;
        }

        self.inner.iter().filter(|&&b| lut[b as usize]).count()
    }
}

// NucleotideView casting methods
impl<'s> NucleotideView<'s> {
    /// Returns the `NucleotideView` as a `DnaView` if the sequence is valid for the DNA-IUPAC alphabet.
    pub fn as_dna(self) -> Result<DnaView<'s>, DnaViewError> {
        DnaView::try_new(self.inner)
    }

    /// Returns the `NucleotideView` as a `RnaView` if the sequence is valid for the RNA-IUPAC alphabet.
    pub fn as_rna(self) -> Result<RnaView<'s>, RnaViewError> {
        RnaView::try_new(self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nucleotide_view() {
        let sequence = Sequence::from("ACGT");

        let nucleotide_view =
            NucleotideView::try_new(&sequence).expect("sequence is valid for DNA alphabet");

        assert!((nucleotide_view.gc_percentage("") - 50f32).abs() < f32::EPSILON);

        assert_eq!(nucleotide_view.gc_count(), 2);

        assert_eq!(nucleotide_view.symbols_count(b"A"), 1);
    }
}
