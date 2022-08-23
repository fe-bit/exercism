fn series(s: &str, n: usize) -> Vec<&str> {
    if n > s.len() {
        return vec![];
    }
    let mut result = vec![];
    for i in 0..s.len() - n + 1 {
        let substring = &s[i..i + n];
        result.push(substring);
    }
    result
}

fn main() {
    assert_eq!(series("49142", 3 as usize), vec!["491", "914", "142"]);
}
