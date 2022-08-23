use std::iter::zip;

fn hamming(dna1: &str, dna2: &str) -> Option<i32> {
    if dna1.len() != dna2.len() {
        return None;
    }

    let dna1_chars = dna1.chars();
    let dna2_chars = dna2.chars();
    let iter = zip(dna1_chars, dna2_chars);
    let mut differences = 0;
    for (dna1_c, dna2_c) in iter {
        if dna1_c != dna2_c {
            differences += 1;
        }
    }
    Some(differences)
}

fn main() {
    assert_eq!(hamming("GAGCCTACTAACGGGAT", "CATCGTAATGACGGCCT"), Some(7));
}
