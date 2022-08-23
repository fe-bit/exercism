use std::collections::HashSet;

fn sum_of_multiples(number: i32, multiples: Vec<i32>) -> i32 {
    let mut result = HashSet::new();
    for n in 1..number {
        for multiple in &multiples {
            if n % multiple == 0 {
                result.insert(n);
            }
        }
    }
    result.iter().sum()
}

fn sum_of_multiples_start(number: i32, multiples: Vec<i32>) -> i32 {
    let mut result: HashSet<i32> = HashSet::new();
    let mut multi = multiples.clone();
    sum_of_multiples_rec(number, multi, 1, result)
}

fn sum_of_multiples_rec(
    number: i32,
    mut multiples: Vec<i32>,
    iteration: i32,
    mut cache: HashSet<i32>,
) -> i32 {
    let mut to_drop = vec![];
    for i in 0..multiples.len() {
        let d = multiples[i] * iteration;
        if d >= number {
            to_drop.push(i);
            continue;
        } else {
            cache.insert(d);
        }
    }

    for i in &to_drop {
        multiples.remove(*i as usize);
    }

    if multiples.len() > 0 {
        let next_iter = iteration + 1;
        return sum_of_multiples_rec(number, multiples, next_iter, cache);
    } else {
        return cache.iter().sum();
    }
}

fn main() {
    assert_eq!(sum_of_multiples_start(20, vec![3, 5]), 78);
}
