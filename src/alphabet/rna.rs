use super::Alphabet;

pub fn rna() -> Alphabet {
    Alphabet::new("ACGU.-")
}

pub fn rna_iupac() -> Alphabet {
    Alphabet::new("ACGURYSWKMBDHVN.-")
}
