use time::ext::NumericalDuration;
use time::PrimitiveDateTime as DateTime;

pub fn after(start: DateTime) -> DateTime {
    start + 1e9.seconds()
}
