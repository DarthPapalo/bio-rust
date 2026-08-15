use thiserror::Error;

use crate::alphabet::dna::DNA_IUPAC;

use crate::sequence::{NucleotideView, Sequence, build_complement_lut};

/// Error type for operations over NucleotideView
#[derive(Error, Debug)]
pub enum DnaViewError {
    #[error("the supplied sequence is not valid for the DNA-IUPAC alphabet")]
    InvalidSequence,
}

pub struct DnaView<'s> {
    inner: &'s Sequence,
    pub nucleotide_view: NucleotideView<'s>,
}

// DnaView creation functions
impl<'s> DnaView<'s> {
    /// Checks for the sequence to be a valid DNA-IUPAC sequence.
    pub fn try_new(sequence: &'s Sequence) -> Result<Self, DnaViewError> {
        if DNA_IUPAC.validate_sequence(sequence) {
            Ok(Self {
                inner: sequence,
                nucleotide_view: NucleotideView::new_unchecked(sequence),
            })
        } else {
            Err(DnaViewError::InvalidSequence)
        }
    }

    /// Creates the view without checking for a valid DNA-IUPAC sequence.
    pub fn new_unchecked(sequence: &'s Sequence) -> Self {
        Self {
            inner: sequence,
            nucleotide_view: NucleotideView::new_unchecked(sequence),
        }
    }
}

// DnaView utility methods
impl<'s> DnaView<'s> {
    /// Returns the complement of the DNA sequence view.
    pub fn complement(&self) -> Sequence {
        const DNA_COMPLEMENT_LUT: [u8; 256] =
            build_complement_lut(b"ACGTRYSWKMBDHV", b"TGCAYRSWMKVHDB");

        Sequence::from(
            self.inner
                .iter()
                .map(|&b| DNA_COMPLEMENT_LUT[b as usize])
                .collect::<Vec<u8>>(),
        )
    }

    /// Returns the transcribed RNA from this DNA sequence.
    pub fn transcript(&self) -> Sequence {
        const DNA_TRANSCRIPTION_LUT: [u8; 256] =
            build_complement_lut(b"ACGTRYSWKMBDHV", b"UGCAYRSWMKVHDB");

        Sequence::from(
            self.inner
                .iter()
                .map(|&b| DNA_TRANSCRIPTION_LUT[b as usize])
                .collect::<Vec<u8>>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequence::nucleotide::NucleotideView;

    #[test]
    fn test_nucleotide_view() {
        let sequence = Sequence::from("ACGT");

        let nucleotide_view =
            NucleotideView::try_new(&sequence).expect("sequence is valid for DNA alphabet");

        // NucleotideView casting to DnaView
        let dna_view = nucleotide_view.as_dna().expect("it is a DNA sequence");
        assert_eq!(dna_view.complement(), "TGCA");
        assert_eq!(dna_view.transcript(), "UGCA");

        // Not a valid DNA sequence
        assert!(DnaView::try_new(&Sequence::from("ACGU")).is_err());
    }

    #[test]
    fn test_nucleotide_view_access() {
        let seq = Sequence::from("ACGTGTCANNN");

        let dna_view = DnaView::try_new(&seq).expect("its a valid DNA-IUPAC sequence");

        assert_eq!(dna_view.complement(), "TGCACAGTNNN");

        // A DnaView is also always a valid NucleotideView
        // We can access it from DnaView
        assert_eq!(dna_view.nucleotide_view.gc_count(), 4);
    }
}
