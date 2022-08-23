fn bob(sentence: &str) -> String {
    if sentence.to_uppercase().as_str() == sentence {
        if sentence.chars().last().unwrap() == '?' {
            return String::from("Calm down, I know what I'm doing!");
        }
        return String::from("Whoa, chill out!");
    } else if sentence.chars().last().unwrap() == '?' {
        return String::from("Sure.");
    } else {
        return String::from("Fine. Be that way!");
    }
}

fn main() {
    assert_eq!(bob("YELLING"), String::from("Whoa, chill out!"));
    assert_eq!(
        bob("YELLING?"),
        String::from("Calm down, I know what I'm doing!")
    );
    assert_eq!(bob("How are you?"), String::from("Sure."));
    assert_eq!(bob("I don't know."), String::from("Fine. Be that way!"));
}
