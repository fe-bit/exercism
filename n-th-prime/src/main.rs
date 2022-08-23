fn get_nth_prime(n: i32) -> u32 {
    (2..)
        .filter(|x| is_prime(*x))
        .nth((n - 1) as usize)
        .unwrap()
}

fn is_prime(number: u32) -> bool {
    !(2..(number as f32).sqrt() as u32 + 1).any(|x| number % x == 0)
}

fn main() {
    assert_eq!(get_nth_prime(6), 13)
}
