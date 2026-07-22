//! Nucleotide sequence module

use crate::alphabet::dna::dna_iupac;
use crate::alphabet::rna::rna_iupac;

use super::{Sequence, SequenceViewError};

#[derive(Debug)]
pub struct NucleotideView<'s> {
    inner: &'s Sequence,
}

/// A struct representing a view to a DNA-IUPAC or RNA-IUPAC sequence.
/// Contains useful methods to calculate the GC contents and symbol counts.
impl<'s> NucleotideView<'s> {
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

    #[inline]
    pub fn gc_count(&self) -> usize {
        self.inner
            .iter()
            .filter(|&&b| matches!(b, b'G' | b'C' | b'g' | b'c'))
            .count()
    }

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

impl<'s> NucleotideView<'s> {
    /// Checks for the sequence to be a valid DNA-IUPAC of RNA-IUPAC sequence
    pub fn try_new(sequence: &'s Sequence) -> Result<Self, SequenceViewError> {
        if dna_iupac().validate_sequence(&sequence) || rna_iupac().validate_sequence(&sequence) {
            Ok(Self { inner: &sequence })
        } else {
            Err(SequenceViewError::InvalidSequence("DNA-IUPAC/RNA-IUPAC"))
        }
    }

    /// Creates the view without checking for a valid DNA IUPAC of RNA IUPAC sequence
    pub fn new_unchecked(sequence: &'s Sequence) -> Self {
        Self { inner: &sequence }
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
