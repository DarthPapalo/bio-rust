use bio::sequence::nucleotide::dna::DnaView;
use bio::sequence::nucleotide::rna::{RnaView, RnaViewError};
use bio::sequence::*;

#[test]
fn test_sequences() {
    // Easy to create through From implementations for common sequence types (&str, String, BString, BStr)
    let sequence = Sequence::from("ACGTGANGTYYTGCNA");

    // Using methods for any nucleotide sequence
    let nucleotide_view =
        NucleotideView::try_new(&sequence).expect("it is a valid nucleotide sequence");

    assert_eq!(nucleotide_view.gc_count(), 6);

    // Use DNA sequence specific methods through validation
    let dna_view = DnaView::try_new(&sequence).expect("it is a valid DNA-IUPAC sequence");

    assert_eq!(dna_view.complement(), "TGCACTNCARRACGNT");

    // Or casting from the NucleotideView, also with validation
    let dna_view = nucleotide_view
        .as_dna()
        .expect("it is a valid DNA-IUPAC sequence");

    assert_eq!(dna_view.transcript(), "UGCACUNCARRACGNU");

    // Try to use RNA methods on an invalid RNA sequence
    let rna_view = RnaView::try_new(&sequence);

    // Will result in an error that the user can handle
    assert!(matches!(rna_view, Err(RnaViewError::InvalidSequence)));
}
