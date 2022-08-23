use std::collections::HashSet;

fn isogram(s: &str) -> bool {
    let mut cache = HashSet::new();
    for c in s.chars() {
        if !cache.insert(c) {
            return false;
        }
    }
    true
}

fn main() {
    assert!(isogram("lumberjacks"));
}
