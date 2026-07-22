//! Module with helper structs with methods to work with common sequence types.

mod nucleotide;
pub use nucleotide::NucleotideView;

use bstr::BString;
use thiserror::Error;

pub type Sequence = BString;

#[derive(Debug, Error)]
pub enum SequenceViewError {
    #[error("the supplied sequence is not valid for the {0} alphabet")]
    InvalidSequence(&'static str),
}
