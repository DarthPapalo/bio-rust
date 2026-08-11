use std::sync::LazyLock;

use super::Alphabet;

pub static AMINOACID: LazyLock<Alphabet> = LazyLock::new(|| Alphabet::new("ACDEFGHIKLMNPQRSTVWY*"));
