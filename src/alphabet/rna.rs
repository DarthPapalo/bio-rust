use std::sync::LazyLock;

use super::Alphabet;

pub static RNA: LazyLock<Alphabet> = LazyLock::new(|| Alphabet::new("ACGU.-"));
pub static RNA_IUPAC: LazyLock<Alphabet> = LazyLock::new(|| Alphabet::new("ACGURYSWKMBDHVN.-"));
