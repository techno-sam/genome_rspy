/*
Genome RSPY
Copyright (C) 2024  Sam Wagenaar

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use super::*;

fn sorted<T: Ord>(v: Vec<T>) -> Vec<T> {
    let mut v = v;
    v.sort();
    return v;
}

#[test]
fn fasta_parsing() {
    let fasta = ">MyTitle\nACTGTTTACGACTCTATCAGAGAGGGATTAC\nCATCTACTTCTAA\n>OtherLabel\nATCTCTACATGAGGAGGATTACATAGAGGAGATGATGTCCG";

    let parsed = sorted(Fasta::parse_fasta(fasta).expect("Parsing succeeded").0);
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed, sorted(vec![
        ("MyTitle".to_owned(), "ACTGTTTACGACTCTATCAGAGAGGGATTACCATCTACTTCTAA".to_owned()),
        ("OtherLabel".to_owned(), "ATCTCTACATGAGGAGGATTACATAGAGGAGATGATGTCCG".to_owned())
    ]));
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
    assert_eq!(sorted(fasta.0), sorted(reparsed.0));
}
