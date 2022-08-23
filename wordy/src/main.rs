#[derive(PartialEq, Debug)]
enum Operator {
    Minus,
    Plus,
    Multiply,
    Division,
    None,
}

fn wordy(call: &str) -> Result<i32, &'static str> {
    if call.starts_with("What is") && call.ends_with("?") {
        let new_call = call[7..call.len() - 1 as usize]
            .replace("minus", "-")
            .replace("plus", "+")
            .replace("multiplied by", "*")
            .replace("divided by", "/");
        let new_call: Vec<&str> = new_call.split_whitespace().collect();
        let starting_number: i32 = new_call[0].parse::<i32>().expect("Should be a number");
        wordy_recursively(new_call[1..].to_vec(), starting_number, Operator::None)
    } else {
        Err("Does  not start with What is")
    }
}

fn wordy_recursively(call: Vec<&str>, cache: i32, operator: Operator) -> Result<i32, &'static str> {
    println!(
        "call: {:?}, cache: {}, operator: {:?}",
        call, cache, operator
    );
    if call.len() == 0 {
        if operator == Operator::None {
            return Ok(cache);
        } else {
            return Err("There is still an operator with no following number");
        }
    }
    if operator != Operator::None {
        let n = call[0].parse::<i32>().expect("Should be a number");
        match operator {
            Operator::Plus => wordy_recursively(call[1..].to_vec(), n + cache, Operator::None),
            Operator::Minus => wordy_recursively(call[1..].to_vec(), cache - n, Operator::None),
            Operator::Multiply => wordy_recursively(call[1..].to_vec(), cache * n, Operator::None),
            Operator::Division => wordy_recursively(call[1..].to_vec(), cache / n, Operator::None),
            _ => Err("Not valid operation"),
        }
    } else {
        match call[0] {
            "-" => wordy_recursively(call[1..].to_vec(), cache, Operator::Minus),
            "+" => wordy_recursively(call[1..].to_vec(), cache, Operator::Plus),
            "*" => wordy_recursively(call[1..].to_vec(), cache, Operator::Multiply),
            "/" => wordy_recursively(call[1..].to_vec(), cache, Operator::Division),
            _ => Err("Not a valid operator"),
        }
    }
}

fn main() {
    assert_eq!(wordy("What is 7 minus 5?"), Ok(2));
    assert_eq!(wordy("What is 6 multiplied by 4?"), Ok(24));
    assert_eq!(wordy("What is 25 divided by 5?"), Ok(5));
    assert_eq!(wordy("What is 5 plus 13 plus 6?"), Ok(24));
    assert_eq!(wordy("What is 3 plus 2 multiplied by 3?"), Ok(15));
}
