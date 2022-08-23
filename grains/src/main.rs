use std::time::Instant;

fn grain(square: u32) -> u64 {
    if square > 64 || square < 1 {
        panic!("Square must be between 1 and 64");
    }

    1 << (square - 1)
}

fn total() -> u64 {
    let now = Instant::now();
    //let result = (1..65).map(grain).sum();
    //let result = 2_u128.pow(64) - 1;
    let result = (1_u128 << 64_u128) - 1;
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    result as u64
    //
}

fn main() {
    assert_eq!(grain(1), 1);
    assert_eq!(grain(2), 2);
    assert_eq!(grain(3), 4);
    assert_eq!(grain(4), 8);

    assert_eq!(total(), 18_446_744_073_709_551_615);
}
