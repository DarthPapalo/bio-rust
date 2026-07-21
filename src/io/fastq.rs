//! IO module for FASTQ format files

use std::io;

use bstr::{BString, ByteVec};
use thiserror::Error;

const FASTQ_RECORD_START: u8 = b'@';

/// A FASTQ format record
/// Includes an ID, sequence and quality
#[derive(Debug, Default, Clone)]
pub struct Record {
    pub id: BString,
    pub sequence: BString,
    pub quality: BString,
}

impl Record {
    fn reset(&mut self) {
        self.id.clear();
        self.sequence.clear();
        self.quality.clear();
    }
}

// FASTQ reader error
#[derive(Error, Debug)]
pub enum ReaderError {
    #[error("IO error in FASTQ file")]
    IO(#[from] io::Error),

    #[error("expected valid FASTQ record header")]
    ExpectedHeader,

    #[error("expected valid FASTQ sequence")]
    ExpectedSequence,

    #[error("expected valid FASTQ sequence-quality separator")]
    ExpectedSeparator,

    #[error("expected valid FASTQ quality")]
    ExpectedQuality,
}

enum ReaderState {
    Header,
    Sequence,
    Separator,
    Quality,
}

/// FASTQ reader
pub struct Reader<R> {
    inner: R,
    line_buf: BString,
    record_buf: Record,
}

impl<R> Reader<R>
where
    R: io::BufRead,
{
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            line_buf: BString::default(),
            record_buf: Record::default(),
        }
    }

    pub fn read_records(&mut self) -> Result<Vec<Record>, ReaderError> {
        let mut records = Vec::new();

        while let Some(record) = self.next_record()? {
            records.push(record.clone());
        }

        Ok(records)
    }

    pub fn next_record(&mut self) -> Result<Option<&Record>, ReaderError> {
        self.record_buf.reset();

        let mut state = ReaderState::Header;

        loop {
            if self.line_buf.is_empty() {
                // Read next line and trim the end
                let bytes_read = self
                    .inner
                    .read_until(b'\n', &mut self.line_buf)
                    .map_err(ReaderError::IO)?;

                if bytes_read == 0 {
                    if !self.record_buf.id.is_empty() {
                        if self.record_buf.sequence.is_empty() {
                            return Err(ReaderError::ExpectedSequence);
                        } else if self.record_buf.quality.is_empty() {
                            return Err(ReaderError::ExpectedQuality);
                        }
                        return Ok(Some(&self.record_buf));
                    }
                    return Ok(None); // EOF
                }

                // Remove newline byte
                if self.line_buf.ends_with(b"\n") {
                    self.line_buf.pop();
                }
            }

            match state {
                ReaderState::Header => {
                    if self.line_buf.is_empty() {
                        continue; // Ignore empty lines between headers
                    }

                    if self.line_buf.first() != Some(&FASTQ_RECORD_START) {
                        return Err(ReaderError::ExpectedHeader);
                    }

                    self.record_buf.id.push_str(&self.line_buf);
                    self.line_buf.clear();

                    state = ReaderState::Sequence;
                }
                ReaderState::Sequence => {
                    if self.line_buf.is_empty() {
                        return Err(ReaderError::ExpectedSequence);
                    }

                    self.record_buf.sequence.push_str(&self.line_buf);
                    self.line_buf.clear();

                    state = ReaderState::Separator;
                }
                ReaderState::Separator => {
                    if self.line_buf.is_empty() {
                        return Err(ReaderError::ExpectedSeparator);
                    }

                    self.line_buf.clear();

                    state = ReaderState::Quality;
                }
                ReaderState::Quality => {
                    if self.line_buf.is_empty() {
                        return Err(ReaderError::ExpectedQuality);
                    }

                    self.record_buf.quality.push_str(&self.line_buf);
                    self.line_buf.clear();

                    return Ok(Some(&self.record_buf));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::assert_matches;
    use std::io::BufReader;

    #[test]
    fn test_read_records() {
        let buf_reader = BufReader::new(
            b"\n\
            @id1\n\
            ACGTACGT\n\
            +\n\
            IIIIHHHH\n\
            @id2\n\
            TGCATGCA\n\
            +\n\
            HHHHIIII\n\
            \n\
            \n\
            @id3\n\
            UTGAUTGA\n\
            +\n\
            KKKKKKKK\
            "
            .as_slice(),
        );

        let mut fastq_reader = Reader::new(buf_reader);
        let records = fastq_reader.read_records().unwrap();
        assert_eq!(records[0].id, "@id1");
        assert_eq!(records[0].sequence, "ACGTACGT");
        assert_eq!(records[0].quality, "IIIIHHHH");

        assert_eq!(records[1].id, "@id2");
        assert_eq!(records[1].sequence, "TGCATGCA");
        assert_eq!(records[1].quality, "HHHHIIII");

        assert_eq!(records[2].id, "@id3");
        assert_eq!(records[2].sequence, "UTGAUTGA");
        assert_eq!(records[2].quality, "KKKKKKKK");
    }

    #[test]
    fn test_header_error() {
        let buf_reader = BufReader::new(
            b"\n\
            @id1\n\
            ACGTACGT\n\
            +\n\
            IIIIHHHH\n\
            >id2\n\
            TGCATGCA\n\
            +\n\
            HHHHIIII\
            "
            .as_slice(),
        );

        let mut fastq_reader = Reader::new(buf_reader);
        let records = fastq_reader.read_records();
        assert_matches!(records, Err(ReaderError::ExpectedHeader));
    }
}
