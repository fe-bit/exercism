fn beer_song(maximum: i32) {
    if maximum < 1 {
        return;
    }
    for i in (0..maximum + 1).rev() {
        match i {
            0 => println!(
                "No more bottles of beer on the wall, no more bottles of beer.
                    Go to the store and buy some more, 99 bottles of beer on the wall."
            ),
            1 => println!(
                "1 bottle of beer on the wall, 1 bottle of beer.
                Take it down and pass it around, no more bottles of beer on the wall.",
            ),
            _ => println!(
                "{} bottles of beer on the wall, {} bottles of beer.
                Take one down and pass it around, {} bottle of beer on the wall.",
                i,
                i,
                i - 1
            ),
        }
    }
}

fn main() {
    beer_song(5);
}
