fn isbn_is_valid(isbn: &str) -> bool {
    isbn_is_valid_rec(isbn, 10, 0)
}

fn isbn_is_valid_rec(isbn: &str, factor: i32, cache: i32) -> bool {
    if let Some(c) = isbn.chars().nth(0) {
        match c {
            '-' => return isbn_is_valid_rec(&isbn[1..], factor, cache),
            _ => {
                let c = c.to_string().parse::<i32>().unwrap();
                return isbn_is_valid_rec(&isbn[1..], factor - 1, cache + factor * c);
            }
        }
    } else {
        return isbn.len() == 0 && factor == 0 && cache % 11 == 0;
    }
}

fn main() {
    assert!(isbn_is_valid("3-598-21508-8"));
    assert!(!isbn_is_valid("3-598-21508-2"));
}
