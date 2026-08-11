use thiserror::Error;

use crate::alphabet::rna::RNA_IUPAC;

use crate::sequence::{Sequence, build_complement_lut};

/// Error type for operations over NucleotideView
#[derive(Error, Debug)]
pub enum RnaViewError {
    #[error("the supplied sequence is not valid for the RNA-IUPAC alphabet")]
    InvalidSequence,
}

pub struct RnaView<'s> {
    inner: &'s Sequence,
}

// RnaView creation functions
impl<'s> RnaView<'s> {
    /// Checks for the sequence to be a valid RNA-IUPAC sequence.
    pub fn try_new(sequence: &'s Sequence) -> Result<Self, RnaViewError> {
        if RNA_IUPAC.validate_sequence(sequence) {
            Ok(Self { inner: sequence })
        } else {
            Err(RnaViewError::InvalidSequence)
        }
    }

    /// Creates the view without checking for a valid RNA-IUPAC sequence.
    pub fn new_unchecked(sequence: &'s Sequence) -> Self {
        Self { inner: sequence }
    }
}

// RnaView utility methods
impl<'s> RnaView<'s> {
    /// Returns the complement of the RNA sequence view.
    pub fn complement(&self) -> Sequence {
        const RNA_COMPLEMENT_LUT: [u8; 256] =
            build_complement_lut(b"ACGURYSWKMBDHV", b"UGCAYRSWMKVHDB");

        Sequence::from(
            self.inner
                .iter()
                .map(|&b| RNA_COMPLEMENT_LUT[b as usize])
                .collect::<Vec<u8>>(),
        )
    }

    /// Returns the transcribed RNA from this DNA sequence.
    pub fn reverse_transcript(&self) -> Sequence {
        const RNA_REVERSE_TRANSCRIPTION_LUT: [u8; 256] =
            build_complement_lut(b"ACGURYSWKMBDHV", b"TGCAYRSWMKVHDB");

        Sequence::from(
            self.inner
                .iter()
                .map(|&b| RNA_REVERSE_TRANSCRIPTION_LUT[b as usize])
                .collect::<Vec<u8>>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequence::nucleotide::NucleotideView;

    #[test]
    fn test_rna_view() {
        let sequence = Sequence::from("ACGU");

        let nucleotide_view =
            NucleotideView::try_new(&sequence).expect("sequence is valid for RNA alphabet");

        // NucleotideView casting to RnaView
        let rna_view = nucleotide_view.as_rna().expect("it is a RNA sequence");
        assert_eq!(rna_view.complement(), "UGCA");
        assert_eq!(rna_view.reverse_transcript(), "TGCA");

        // Not a valid RNA sequence
        assert!(RnaView::try_new(&Sequence::from("ACGT")).is_err());
    }
}
