fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.step_by(2)
}

struct Position(i16, i16);

impl Position {
    fn manhattan(&self) -> i16 {
        let origin = Position(0, 0);
        let x = self.0 - origin.0;
        let y = self.1 - origin.1;
        x.abs() + y.abs()
    }
}

fn main() {
    assert_eq!(divmod(10, 3), (3, 1));
    let mut even_ints = evens(0_u8..);
    assert_eq!(even_ints.next(), Some(0));
    assert_eq!(even_ints.next(), Some(2));
    assert_eq!(even_ints.next(), Some(4));
    assert_eq!(even_ints.next(), Some(6));

    let mut evens_from_odds = evens(1_i16..);
    assert_eq!(evens_from_odds.next(), Some(1));
    assert_eq!(evens_from_odds.next(), Some(3));
    assert_eq!(evens_from_odds.next(), Some(5));
    assert_eq!(evens_from_odds.next(), Some(7));

    assert_eq!(Position(3, 4).manhattan(), 7);
}
