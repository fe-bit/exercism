fn is_armstrong_number(number: i32) -> bool {
    let mut result = 0;
    let digits = number
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let len_digits = digits.len() as u32;
    for digit in digits {
        result += digit.pow(len_digits);
    }
    result == number.try_into().unwrap()
}

fn main() {
    assert!(is_armstrong_number(9));
    assert!(is_armstrong_number(153));
    assert!(!is_armstrong_number(154));
}
