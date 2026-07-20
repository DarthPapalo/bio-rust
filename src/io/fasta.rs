//! IO module for FASTA format files

use std::io;

use thiserror::Error;

const FASTA_RECORD_START: u8 = b'>';

/// A FASTA format record
/// Includes an ID and a sequence
#[derive(Debug, Default, Clone)]
pub struct Record {
    pub id: String,
    pub sequence: String,
}

impl Record {
    fn reset(&mut self) {
        self.id.clear();
        self.sequence.clear();
    }
}

// FASTA reader error
#[derive(Error, Debug)]
pub enum ReaderError {
    #[error("IO error in FASTA file")]
    IO(#[from] io::Error),

    #[error("expected valid FASTA record header")]
    ExpectedHeader,

    #[error("expected valid FASTA sequence")]
    ExpectedSequence,
}

enum ReaderState {
    Header,
    Sequence,
}

/// FASTA reader
pub struct Reader<R> {
    inner: R,
    line_buf: String,
    record_buf: Record,
}

impl<R> Reader<R>
where
    R: io::BufRead,
{
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            line_buf: String::new(),
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
                    .read_line(&mut self.line_buf)
                    .map_err(ReaderError::IO)?;

                if bytes_read == 0 {
                    if !self.record_buf.id.is_empty() {
                        if self.record_buf.sequence.is_empty() {
                            return Err(ReaderError::ExpectedSequence);
                        }
                        return Ok(Some(&self.record_buf));
                    }
                    return Ok(None); // EOF
                }

                self.line_buf.truncate(self.line_buf.trim_end().len());
            }

            if self.line_buf.is_empty() {
                continue; // Ignore empty lines
            }

            match state {
                ReaderState::Header => {
                    if self.line_buf.as_bytes().first() == Some(&FASTA_RECORD_START) {
                        self.record_buf.id.push_str(&self.line_buf);
                        self.line_buf.clear();

                        state = ReaderState::Sequence;
                    } else {
                        return Err(ReaderError::ExpectedHeader);
                    }
                }
                ReaderState::Sequence => {
                    if self.line_buf.as_bytes().first() == Some(&FASTA_RECORD_START) {
                        if self.record_buf.sequence.is_empty() {
                            return Err(ReaderError::ExpectedSequence);
                        }

                        break;
                    } else {
                        self.record_buf.sequence.push_str(&self.line_buf);
                        self.line_buf.clear();
                    }
                }
            }
        }

        Ok(Some(&self.record_buf))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::assert_matches;
    use std::io::BufReader;

    #[test]
    fn test_next_record() {
        let buf_reader = BufReader::new(
            b"\n\
            >id1\n\
            tgcatgca\n\
            >id2\n\
            TGCATGCA\n\
            "
            .as_slice(),
        );

        let mut fasta_reader = Reader::new(buf_reader);

        let record = fasta_reader.next_record().unwrap().unwrap();
        assert_eq!(record.id, ">id1");
        assert_eq!(record.sequence, "tgcatgca");

        let record = fasta_reader.next_record().unwrap().unwrap();
        assert_eq!(record.id, ">id2");
        assert_eq!(record.sequence, "TGCATGCA");
    }

    #[test]
    fn test_read_records() {
        let buf_reader = BufReader::new(
            b"\n\
            >id1\n\
            ACGTACGT\n\
            tgcatgca\n\
            >id2\n\
            TGCATGCA\n\
            \n\
            \n\
            >id3\n\
            UTGAUTGA\n\
            "
            .as_slice(),
        );

        let mut fasta_reader = Reader::new(buf_reader);
        let records = fasta_reader.read_records().unwrap();

        assert_eq!(records[0].id, ">id1");
        assert_eq!(records[0].sequence, "ACGTACGTtgcatgca");

        assert_eq!(records[1].id, ">id2");
        assert_eq!(records[1].sequence, "TGCATGCA");

        assert_eq!(records[2].id, ">id3");
        assert_eq!(records[2].sequence, "UTGAUTGA");
    }

    #[test]
    fn test_header_error() {
        let buf_reader = BufReader::new(
            b"\n\
            xid1\n\
            ACGTACGT\n\
            tgcatgca\
            "
            .as_slice(),
        );

        let mut fasta_reader = Reader::new(buf_reader);
        let records = fasta_reader.read_records();
        assert_matches!(records, Err(ReaderError::ExpectedHeader));
    }
}
