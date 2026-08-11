use bio;

#[test]
fn test_sequence_classification() {
    use std::collections::HashSet;

    use bio::alphabet;
    use bio::alphabet::classifier;
    use bio::sequence;

    let mut classifier = classifier::Classifier::new();
    classifier
        .add_alphabet("DNA-IUPAC", &alphabet::dna::DNA_IUPAC)
        .unwrap();
    classifier
        .add_alphabet("RNA-IUPAC", &alphabet::rna::RNA_IUPAC)
        .unwrap();

    let dna_seq = sequence::Sequence::from("ACGTGTA");
    let rna_seq = sequence::Sequence::from("ACGUGUA");
    let ambiguous_seq = sequence::Sequence::from("ACGNNNAGC");

    assert_eq!(
        classifier.classify(dna_seq),
        classifier::AlphabetType::Exact("DNA-IUPAC".into())
    );

    assert_eq!(
        classifier.classify(rna_seq),
        classifier::AlphabetType::Exact("RNA-IUPAC".into())
    );

    assert_eq!(
        classifier.classify(ambiguous_seq),
        classifier::AlphabetType::Ambiguous(HashSet::from([
            "RNA-IUPAC".into(),
            "DNA-IUPAC".into()
        ]))
    );

    assert_eq!(
        classifier.classify("ZZZZZ"),
        classifier::AlphabetType::Unknown
    );
}
