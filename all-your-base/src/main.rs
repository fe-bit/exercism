fn all_your_base(n: i32, base: i32) -> String {
    let mut result = vec![];
    let n_string = n.to_string();
    let max_base = n_string.len() as i32 - 1;

    for (i, c) in n_string.chars().enumerate() {
        let s = format!("({} * {}^{})", c, base, max_base - i as i32);
        result.push(s);
    }
    result.join(" + ")
}

fn main() {
    assert_eq!(
        all_your_base(42, 10),
        String::from("(4 * 10^1) + (2 * 10^0)")
    );
    assert_eq!(
        all_your_base(101010, 2),
        String::from("(1 * 2^5) + (0 * 2^4) + (1 * 2^3) + (0 * 2^2) + (1 * 2^1) + (0 * 2^0)")
    );
}
