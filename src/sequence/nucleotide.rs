//! Nucleotide sequence module

use crate::alphabet::dna::dna_iupac;
use crate::alphabet::rna::rna_iupac;

use super::{Sequence, SequenceViewError};

/// A struct representing a view to a DNA-IUPAC or RNA-IUPAC sequence.
/// Contains useful methods to calculate the GC contents and symbol counts.
#[derive(Debug)]
pub struct NucleotideView<'s> {
    inner: &'s Sequence,
}

// NucleotideView creation functions
impl<'s> NucleotideView<'s> {
    /// Checks for the sequence to be a valid DNA-IUPAC of RNA-IUPAC sequence.
    pub fn try_new(sequence: &'s Sequence) -> Result<Self, SequenceViewError> {
        if dna_iupac().validate_sequence(&sequence) || rna_iupac().validate_sequence(&sequence) {
            Ok(Self { inner: &sequence })
        } else {
            Err(SequenceViewError::InvalidSequence(
                "DNA-IUPAC/RNA-IUPAC".to_owned(),
            ))
        }
    }

    /// Creates the view without checking for a valid DNA IUPAC of RNA IUPAC sequence.
    pub fn new_unchecked(sequence: &'s Sequence) -> Self {
        Self { inner: &sequence }
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

    /// Returns the DNA complement of the nucleotide view sequence.
    pub fn dna_complement(&self) -> Sequence {
        const COMPLEMENT_LUT: [u8; 256] = {
            let mut lut = [0; 256];

            lut[b'A' as usize] = b'T';
            lut[b'a' as usize] = b't';
            lut[b'C' as usize] = b'G';
            lut[b'c' as usize] = b'g';
            lut[b'G' as usize] = b'C';
            lut[b'g' as usize] = b'c';
            lut[b'T' as usize] = b'A';
            lut[b't' as usize] = b'a';
            lut[b'U' as usize] = b'A';
            lut[b'u' as usize] = b'a';

            lut
        };

        let mut res = Sequence::new(Vec::with_capacity(self.inner.capacity()));

        for &u in self.inner.iter() {
            res.push(COMPLEMENT_LUT[u as usize]);
        }

        res
    }

    /// Returns the RNA complement of the nucleotide view sequence.
    pub fn rna_complement(&self) -> Sequence {
        const COMPLEMENT_LUT: [u8; 256] = {
            let mut lut = [0; 256];

            lut[b'A' as usize] = b'U';
            lut[b'a' as usize] = b'u';
            lut[b'C' as usize] = b'G';
            lut[b'c' as usize] = b'g';
            lut[b'G' as usize] = b'C';
            lut[b'g' as usize] = b'c';
            lut[b'T' as usize] = b'A';
            lut[b't' as usize] = b'a';
            lut[b'U' as usize] = b'A';
            lut[b'u' as usize] = b'a';

            lut
        };

        let mut res = Sequence::new(Vec::with_capacity(self.inner.capacity()));

        for &u in self.inner.iter() {
            res.push(COMPLEMENT_LUT[u as usize]);
        }

        res
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
        assert_eq!(nucleotide_view.dna_complement(), "TGCA");
        assert_eq!(nucleotide_view.rna_complement(), "UGCA");
    }
}
