fn acronym(s: &str) -> String {
    s.split_whitespace()
        .map(|x| x.chars().nth(0).unwrap())
        .collect::<String>()
}

fn main() {
    assert_eq!(acronym("Portable Network Graphics"), String::from("PNG"));
    assert_eq!(
        acronym("Moving Average Line of Time"),
        String::from("MALoT")
    );
}
