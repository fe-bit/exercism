const ALLERGIES: [(i32, &'static str); 8] = [
    (128, "Cats"),
    (64, "Pollen"),
    (32, "Chocolate"),
    (16, "Tomatoes"),
    (8, "Strawberries"),
    (4, "Shellfish"),
    (2, "Peanuts"),
    (1, "Eggs"),
];

fn allergies(n: i32) -> Vec<String> {
    let mut result = vec![];
    let mut allergie_score = n.clone();
    for (score, allergy) in ALLERGIES {
        if allergie_score >= score {
            allergie_score -= score;
            result.push(String::from(allergy));
        }
        if allergie_score == 0 {
            break;
        }
    }
    if allergie_score == 0 {
        return result;
    } else {
        println!("Should never happen!");
        return result;
    }
}

fn main() {
    assert_eq!(allergies(2), vec![String::from("Peanuts")]);
    assert_eq!(
        allergies(34),
        vec![String::from("Chocolate"), String::from("Peanuts")]
    );
}
