fn get_factors(number: u32) -> Vec<u32> {
    let result: Vec<u32> = (1..number)
        .collect::<Vec<u32>>()
        .into_iter()
        .filter(|s| number % s == 0)
        .collect();
    result
}

fn get_aliquot_sum(number: u32) -> u32 {
    let factors = get_factors(number);
    factors.iter().sum()
}

#[derive(PartialEq, Debug)]
enum Category {
    Perfect,
    Abundant,
    Deficient,
}

fn classify_number(number: u32) -> Category {
    let sum = get_aliquot_sum(number);
    if sum == number {
        Category::Perfect
    } else if sum > number {
        Category::Abundant
    } else {
        Category::Deficient
    }
}

fn main() {
    assert_eq!(classify_number(6), Category::Perfect);
    assert_eq!(classify_number(28), Category::Perfect);
    assert_eq!(classify_number(12), Category::Abundant);
    assert_eq!(classify_number(24), Category::Abundant);
    assert_eq!(classify_number(8), Category::Deficient);
    assert_eq!(classify_number(5), Category::Deficient);

    let number = 6;
    let result: Vec<u32> = (1..number)
        .collect::<Vec<u32>>()
        .into_iter()
        .filter(|s| number % s == 0)
        .collect();
    assert_eq!(result, vec![1, 2, 3]);
}
