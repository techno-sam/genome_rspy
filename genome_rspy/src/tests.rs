use super::*;

#[test]
fn fasta_parsing() {
    let fasta = ">MyTitle\nACTGTTTACGACTCTATCAGAGAGGGATTAC\nCATCTACTTCTAA\n>OtherLabel\nATCTCTACATGAGGAGGATTACATAGAGGAGATGATGTCCG";

    let parsed = Fasta::parse_fasta(fasta).expect("Parsing succeeded");
    assert_eq!(parsed.0.len(), 2);
    assert_eq!(parsed.0, vec![
        ("MyTitle".to_owned(), "ACTGTTTACGACTCTATCAGAGAGGGATTACCATCTACTTCTAA".to_owned()),
        ("OtherLabel".to_owned(), "ATCTCTACATGAGGAGGATTACATAGAGGAGATGATGTCCG".to_owned())
    ]);
}

#[test]
fn fasta_parsing_invalid() {
    let fasta = ">MyTitle\nACATACAC\n>>";

    Fasta::parse_fasta(fasta).expect_err("Parsing failed");
}

#[test]
fn fasta_writing() {
    let fasta = Fasta(vec![
        ("SomeTitle".to_owned(), "ACTATCTATCATGGATATGTATTACCTCTTATCTATGTAGTATTCTACTATCATTACGAT".to_owned()),
        ("SomeOtherTitle".to_owned(), "ACTATCGATCGTAGGGAAGGTTGTATATCGATGAAAC".to_owned())
    ]);

    let written = fasta.write_fasta();

    assert_eq!(&written, ">SomeTitle
ACTATCTATCATGGATATGTATTACCTCTTATCTATGTAGTATTCTACTATCATTACGAT
>SomeOtherTitle
ACTATCGATCGTAGGGAAGGTTGTATATCGATGAAAC
");

    let reparsed = Fasta::parse_fasta(&written).expect("Parsing succeeded");
    assert_eq!(fasta, reparsed);
}
