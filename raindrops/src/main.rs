const SOUNDS: [(i32, &'static str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

fn raindrops(n: i32) -> String {
    // let mut result = String::new();
    // if n % 3 == 0 {
    //     result.push_str("Pling");
    // }
    // if n % 5 == 0 {
    //     result.push_str("Plang");
    // }
    // if n % 7 == 0 {
    //     result.push_str("Plong");
    // }

    // if result.len() == 0 {
    //     return n.to_string();
    // } else {
    //     return result;
    // }

    let result = SOUNDS
        .iter()
        .filter(|(number, _)| n % number == 0)
        .map(|&(_, sound)| sound)
        .collect::<String>();
    match result.len() {
        0 => n.to_string(),
        _ => result,
    }
}

fn main() {
    assert_eq!(raindrops(28), String::from("Plong"));
    assert_eq!(raindrops(30), String::from("PlingPlang"));
    assert_eq!(raindrops(34), String::from("34"));
}
