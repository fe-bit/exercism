use unicode_segmentation::UnicodeSegmentation;

fn reverse_string(mystring: &str) -> String {
    mystring.graphemes(true).rev().collect()
}

fn main() {
    println!("Cool is now {}", reverse_string("Cool"));
}
