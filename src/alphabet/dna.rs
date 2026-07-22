use super::Alphabet;

pub fn dna() -> Alphabet {
    Alphabet::new("ACGT.-")
}

pub fn dna_iupac() -> Alphabet {
    Alphabet::new("ACGTRYSWKMBDHVN.-")
}
