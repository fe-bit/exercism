use std::collections::{HashMap, HashSet};

fn nucletoid_count(s: &str) -> Result<HashMap<char, i32>, &'static str> {
    let mut cache = HashMap::new();
    let valid_characters = HashSet::from(['A', 'C', 'G', 'T']);
    for c in s.chars() {
        if !valid_characters.contains(&c) {
            return Err("Not a valid DNA Character");
        }
        *cache.entry(c).or_insert(0) += 1;
    }
    Ok(cache)
}

fn main() {
    assert_eq!(
        nucletoid_count("GATTACA"),
        Ok(HashMap::from([('A', 3), ('C', 1), ('G', 1), ('T', 2),]))
    );
    assert_eq!(nucletoid_count("INVALID"), Err("Not a valid DNA Character"));
}
