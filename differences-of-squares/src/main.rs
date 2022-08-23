fn diff_of_squares(n: i32) -> i32 {
    let result_sum_first = (n * (n + 1) / 2).pow(2);
    let result_square_first = n * (n + 1) * (2 * n + 1) / 6;

    (result_sum_first - result_square_first).abs()
}

fn main() {
    assert_eq!(diff_of_squares(10), 2640);
}
