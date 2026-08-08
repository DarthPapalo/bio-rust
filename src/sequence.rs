//! Module with helper structs with methods to work with common sequence types.

mod nucleotide;
mod quality;

pub use nucleotide::NucleotideView;
pub use quality::{PhredQualityEncoding, QualityView};

use bstr::BString;

pub type Sequence = BString;
