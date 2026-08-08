use std::fmt::Display;

use thiserror::Error;

use super::Sequence;

/// Error type for operations over QualityView
#[derive(Error, Debug)]
pub enum QualityViewError {
    #[error("invalid sequence for {0} encoding")]
    InvalidSequence(PhredQualityEncoding),

    #[error("invalid {0} encoding change for sequence")]
    InvalidEncodingChange(PhredQualityEncoding),
}

/// Phred quality encodings enum.
#[derive(Debug)]
pub enum PhredQualityEncoding {
    Phred33,
    Phred64,
    Other(u8),
}

impl Display for PhredQualityEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PhredQualityEncoding::Phred33 => write!(f, "Phred+33"),
            PhredQualityEncoding::Phred64 => write!(f, "Phred+64"),
            PhredQualityEncoding::Other(x) => write!(f, "Phred+{x}"),
        }
    }
}

impl PhredQualityEncoding {
    fn ascii_offset(&self) -> u8 {
        match *self {
            PhredQualityEncoding::Phred33 => 33,
            PhredQualityEncoding::Phred64 => 64,
            PhredQualityEncoding::Other(x) => x,
        }
    }
}

/// A struct representing a view to a Phred Quality sequence.
/// Contains a method to calculate the average quality.
#[derive(Debug)]
pub struct QualityView<'s> {
    inner: &'s Sequence,
    encoding: PhredQualityEncoding,
}

// QualityView creation functions
impl<'s> QualityView<'s> {
    /// Checks for the sequence to be a valid Phred Quality sequence.
    pub fn try_new(
        sequence: &'s Sequence,
        encoding: PhredQualityEncoding,
    ) -> Result<Self, QualityViewError> {
        if sequence
            .iter()
            .all(|&b| b >= encoding.ascii_offset() && b <= 126)
        {
            Ok(Self {
                inner: &sequence,
                encoding,
            })
        } else {
            Err(QualityViewError::InvalidSequence(encoding))
        }
    }

    /// Creates the view without checking for a valid Phred Quality sequence.
    pub fn new_unchecked(sequence: &'s Sequence, encoding: PhredQualityEncoding) -> Self {
        Self {
            inner: &sequence,
            encoding,
        }
    }
}

// QualityView utility methods
impl<'s> QualityView<'s> {
    /// Returns the average phred score value of the view.
    pub fn average_quality(&self) -> f32 {
        self.inner
            .iter()
            .map(|&b| (b - self.encoding.ascii_offset()) as usize)
            .sum::<usize>() as f32
            / self.inner.len() as f32
    }

    /// Returns the phred quality value of the symbol at the given position (0 indexed).
    pub fn position_quality(&self, index: usize) -> u8 {
        self.inner[index] - self.encoding.ascii_offset()
    }

    /// Returns a new quality Sequence in the given encoding.
    pub fn into_other_encoding(
        &self,
        other_encoding: PhredQualityEncoding,
    ) -> Result<Sequence, QualityViewError> {
        let mut res = Sequence::new(Vec::with_capacity(self.inner.len()));

        for b in self.inner.iter() {
            let new_b = (b - self.encoding.ascii_offset()) + other_encoding.ascii_offset();

            if !(new_b >= b'!' && new_b <= b'~') {
                return Err(QualityViewError::InvalidEncodingChange(other_encoding));
            }

            res.push(new_b);
        }

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_view() {
        let sequence = Sequence::from("JJJIIIJJJ");

        let quality_view = QualityView::try_new(&sequence, PhredQualityEncoding::Phred33)
            .expect("sequence is valid for the Phred33 Quality alphabet");

        assert!((quality_view.average_quality() - 40.666668).abs() < f32::EPSILON);
        assert_eq!(quality_view.position_quality(0), 41);
        assert_eq!(quality_view.position_quality(3), 40);
        assert_eq!(
            quality_view
                .into_other_encoding(PhredQualityEncoding::Phred64)
                .unwrap(),
            Sequence::from("iiihhhiii")
        );

        let invalid_for_phred122 = Sequence::from("#$$%&");

        let quality_view2 =
            QualityView::try_new(&invalid_for_phred122, PhredQualityEncoding::Phred33)
                .expect("sequence is valid for the Phred33 Quality alphabet");

        assert_eq!(
            quality_view2
                .into_other_encoding(PhredQualityEncoding::Phred64)
                .unwrap(),
            Sequence::from("BCCDE")
        );
        assert!(
            quality_view2
                .into_other_encoding(PhredQualityEncoding::Other(122))
                .is_err()
        );
    }
}
