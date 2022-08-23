fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn main() {
    assert!(!is_leap_year(1997));
    assert!(is_leap_year(1996));
    assert!(!is_leap_year(1900));
    assert!(is_leap_year(2000));
}
