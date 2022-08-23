fn prime_factors(number: i32) -> Vec<i32> {
    let mut divisors: Vec<i32> = vec![];
    let mut divisor = 2;
    let mut number = number;
    while number > 0 && divisor <= number {
        if number % divisor == 0 {
            divisors.push(divisor);
            number /= divisor;
        } else {
            divisor += 1;
        }
    }
    divisors
}

fn main() {
    assert_eq!(prime_factors(60), vec![2, 2, 3, 5]);
}
