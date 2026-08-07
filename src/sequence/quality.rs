use super::{Sequence, SequenceViewError};

/// Phred quality encodings enum.
#[derive(Debug)]
pub enum PhredQualityEncoding {
    Phred33,
    Phred64,
    Other(u8),
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
    ) -> Result<Self, SequenceViewError> {
        if sequence
            .iter()
            .all(|&b| b >= encoding.ascii_offset() && b <= 126)
        {
            Ok(Self {
                inner: &sequence,
                encoding,
            })
        } else {
            Err(SequenceViewError::InvalidSequence(format!(
                "Phred{} Quality",
                encoding.ascii_offset().to_string()
            )))
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
    }
}
