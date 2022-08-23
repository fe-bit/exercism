use std::any::type_name;

fn anagram<'a>(source: &'a str, given: Vec<&str>) -> Vec<&'a str> {
    let original_chars = source.chars();
    let mut source_sorted: Vec<char> = original_chars.collect();

    source_sorted.sort_unstable();
    let mut result: Vec<&str> = Vec::new();
    for item in &mut given {
        if item != &source {
            let mut given_sorted: Vec<char> = item.chars().collect();
            given_sorted.sort_unstable();
            print_type_of(&item);
            if given_sorted == source_sorted {
                result.append(item);
            }
        }
    }
    result
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
fn main() {
    let source = "Hello";
    let given = vec!["olleh", "Liheo"];
    println!("result is {:?}", anagram(source, given));
}
