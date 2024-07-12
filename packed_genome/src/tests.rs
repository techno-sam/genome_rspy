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
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

fn random_nucleotide(rng: &mut impl Rng) -> char {
    return ['A', 'C', 'T', 'G'][rng.gen_range(0..4)];
}

fn random_sequence(rng: &mut impl Rng, len: usize) -> String {
    return (0..len).into_iter().map(|_| random_nucleotide(rng)).collect();
}

#[test]
fn round_trip_simple() {
    let mut rng = StdRng::seed_from_u64(0);
    let seq = random_sequence(&mut rng, 62);

    println!("Sequence: {seq}");

    let packed = SimplePackedSequence::new(&seq);

    assert_eq!(seq, packed.str());
}

#[test]
fn round_trip_encode_simple() {
    let mut rng = StdRng::seed_from_u64(0);
    let seq = random_sequence(&mut rng, 62);

    println!("Sequence: {seq}");

    let packed = SimplePackedSequence::new(&seq);

    let serialized = packed.serialize_and_compress()
        .expect("Serialization works");

    let deserialized = SimplePackedSequence::decompress_and_deserialize(&serialized)
        .expect("Deserialization works");

    assert_eq!(packed.str(), deserialized.str());
}

#[test]
fn round_trip_pre_varied() {
    let mut rng = StdRng::seed_from_u64(0);
    let seq = random_sequence(&mut rng, 62);

    println!("Sequence: {seq}");

    let packed = PreVariedPackedSequence::new(&seq);

    assert_eq!(seq, packed.str());
}

#[test]
fn round_trip_encode_pre_varied() {
    let mut rng = StdRng::seed_from_u64(0);
    let seq = random_sequence(&mut rng, 62);

    println!("Sequence: {seq}");

    let packed = PreVariedPackedSequence::new(&seq);

    let serialized = packed.serialize_and_compress()
        .expect("Serialization works");

    let deserialized = PreVariedPackedSequence::decompress_and_deserialize(&serialized)
        .expect("Deserialization works");

    assert_eq!(packed.str(), deserialized.str());
}

#[test]
fn round_trip_indexed() {
    let mut rng = StdRng::seed_from_u64(0);
    let seq = random_sequence(&mut rng, 62);

    println!("Sequence: {seq}");

    let packed = indexed_packed_sequence!(&seq, 4);

    assert_eq!(seq, packed.str());
}

#[test]
fn round_trip_encode_indexed() {
    let mut rng = StdRng::seed_from_u64(0);
    let seq = random_sequence(&mut rng, 62);

    println!("Sequence: {seq}");

    let packed = IndexedPackedSequence::<u8, 4>::new(&seq);

    let serialized = packed.serialize_and_compress()
        .expect("Serialization works");

    let deserialized = IndexedPackedSequence::<u8, 4>::decompress_and_deserialize(&serialized)
        .expect("Deserialization works");

    assert_eq!(packed.str(), deserialized.str());
}

#[test]
#[should_panic(expected = "Check failed for IndexedPackedSequence, expected chunk length 3, got 4")]
fn round_trip_encode_indexed_mismatched_length() {
    let mut rng = StdRng::seed_from_u64(0);
    let seq = random_sequence(&mut rng, 62);

    println!("Sequence: {seq}");

    let packed = IndexedPackedSequence::<u8, 4>::new(&seq);

    let serialized = packed.serialize_and_compress()
        .expect("Serialization works");

    let _deserialized = IndexedPackedSequence::<u8, 3>::decompress_and_deserialize(&serialized)
        .expect("Deserialization (shouldn't) work");
}

#[test]
#[should_panic(expected = "Io(Kind(UnexpectedEof))")]
fn round_trip_encode_indexed_mismatched_type() {
    let mut rng = StdRng::seed_from_u64(0);
    let seq = random_sequence(&mut rng, 62);

    println!("Sequence: {seq}");

    let packed = IndexedPackedSequence::<u16, 5>::new(&seq);

    let serialized = packed.serialize_and_compress()
        .expect("Serialization works");

    let _deserialized = IndexedPackedSequence::<u8, 3>::decompress_and_deserialize(&serialized)
        .expect("Deserialization (shouldn't) work");
}

#[test]
fn index_sanity_check() {
    let mut rng = StdRng::seed_from_u64(0);
    let seq = random_sequence(&mut rng, 62);

    println!("Sequence: {seq}");

    let packed = indexed_packed_sequence!(&seq, 3);
    assert_eq!(seq, packed.str());

    packed.debug_index();

    let test_value = "AAT";
    let key = packed.str_to_key(test_value);

    println!("Key: {key}");

    let index = packed.index.get(&key);

    assert_eq!(index, Some(vec![6, 13, 18, 30].as_ref()));
}

/*
Search test sequence:
GATCGGAATGTGAAATTGAATGGACTCCGCAATCCTAACATAGGGGCTACCGGTACCCCGGTGTAGCCCTTGCGTTTCAA
GACAAACGACACTTATGTTATTAAACTTAAAAGTGAGGCTATCACATCTCTCCTTCGACCAGTTTCAAGCACCAGCCGTT
AGCACTTAGAGCGAGTGCACAAATGTGGTGTACTTAATCCACAACGGCACCAGACTGGTGTCCAGACTCGAATCAGAAAC
TAGTGTAGTACGGATAGGCTACCTAGCCGGAAGTCTGGTGTCTCGCATTTGTTGGGATCAGGTGCTATCGTTTAAAACTA
GTGGGAGCGGTCGGCTGCACATGTACAAATTTCTCGGTATCTAGAACCAAAACAAGCGTACCGAGGAATGTTGCCCCGTA
CCAGAGTGCAGGATGTGAAATCGACCAGCTTTGATCATTGCAACCCCGAGCCGTGGTCTACCTCTACGAGAGTTGGTCAC
AACAAGTTCGTGTATGCTCCCGGCCGCCTATGAAAGTCATGCGTCGCTCGACCTACTTGGATATAGCGGGCCAGTACTGA
GTTCAACTGCTGGTAGCTACAAGTCGTCGGCCAATCGCCCTTTTAAAGAGTTTCAAGCTATGGGGTATCGCGTCGAAAGC
CTGAGGGTTGGCGTGGGGTAGTGCGAGCTCGAAACGGGAGCTTTTCTGTGGGTAAGCCCCCGATGTCAAGTGCCTGACTG
TCGTGCCCACGGTGAGGGTTTTACATAGACTGGCTCGAGGACCGGTCCTGGCCGGTGTCGGCTGACATCTCACACAAAAG
TACGCCCGTCTAATCGATAAAAAGTTACAGTGATGTCAACCCGCCGAGCTCGGGGAGTTCCGGTTTGTGATCAGTATTTT
AGAACAGCTACTTAATGGCGAGATCCGGAGCGAAACAGAGAGACTGCGATTACGATAACCGACCACGTTTTGGATCGAGA
TGGCAAATGCAGAGGGACTAATAACAAAACATCTTTATAT
*/

#[test]
fn search_test() {
    let mut rng = StdRng::seed_from_u64(0);
    let seq = random_sequence(&mut rng, 1000);

    println!("Sequence: {seq}");

    let haystack = indexed_packed_sequence!(&seq, 5);

    assert_eq!(true, haystack.contains_str("GATCG"), "Search at start");
    assert_eq!(true, haystack.contains_str("GA"), "Search shorter than chunk size");
    assert_eq!(true, haystack.contains_str("ATCGGAATGTGAAAT"), "Search (unaligned) beyond start");
    assert_eq!(false, haystack.contains_str("TAAAGTGTAAGGCTA"), "Non existent segment");

    let pos_test = &seq[31..96];
    assert_eq!(Some(31), haystack.find_str(pos_test), "Find index");
    assert_eq!(None, haystack.find_bounded_str(pos_test, Some(33), None), "Start cutoff");
    assert_eq!(None, haystack.find_bounded_str(pos_test, None, Some(95)), "End cutoff");
    assert_eq!(Some(31), haystack.find_bounded_str(pos_test, None, Some(96)), "End cutoff not too harsh");
}

#[test]
fn bulk_search_test() {
    let mut rng = StdRng::seed_from_u64(0);
    let seq = random_sequence(&mut rng, 100_000);

    println!("Sequence: {seq}");

    let haystack = indexed_packed_sequence!(&seq, 8);

    let needle = &seq[2..2+31];
    assert_eq!(31, needle.len(), "sanity check");
    assert_eq!(Some(vec![2]), unsafe {haystack.find_all_31mer(&SimplePackedSequence::new(needle))});

    let needle = "ACTGAGTTAGCTCTAGCATGGTTAGTACTAC";
    assert_eq!(31, needle.len(), "sanity check");
    assert_eq!(Some(vec![]), unsafe {haystack.find_all_31mer(&SimplePackedSequence::new(needle))});
}
