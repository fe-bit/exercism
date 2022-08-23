fn collatz_conjecture(n: i32) -> i32 {
    collatz_conjecture_rec(n, 0)
}

fn collatz_conjecture_rec(n: i32, count: i32) -> i32 {
    if n == 1 {
        return count;
    }
    let new_count = count + 1;
    match n % 2_i32 {
        0_i32 => return collatz_conjecture_rec(n / 2, new_count),
        1_i32 => return collatz_conjecture_rec((n * 3) + 1, new_count),
        _ => return 0,
    }
}

fn main() {
    assert_eq!(collatz_conjecture(12), 9);
}
