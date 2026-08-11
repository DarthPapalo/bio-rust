use std::sync::LazyLock;

use super::Alphabet;

pub static DNA: LazyLock<Alphabet> = LazyLock::new(|| Alphabet::new("ACGT.-"));
pub static DNA_IUPAC: LazyLock<Alphabet> = LazyLock::new(|| Alphabet::new("ACGTRYSWKMBDHVN.-"));
