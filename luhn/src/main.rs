fn luhn(credit_card: &str) -> bool {
    let mut result = 0;
    for i in 0..credit_card.len() {
        let d = credit_card.chars().nth(i).unwrap();
        let d = d.to_string().parse::<u64>().unwrap();
        match (i % 2 == 0, d > 5) {
            (false, _) => result += d,
            (true, true) => result += d * 2 - 9,
            (true, false) => result += d * 2,
        };
    }
    println!("result is {}", result);
    result % 10 == 0
}

fn main() {
    assert!(luhn("4539319503436467"));
    assert!(!luhn("8273123273520569"));
}
