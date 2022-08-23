fn say_until_100(n: i32) -> String {
    match n {
        0 => "".to_owned(),
        1 => "one".to_owned(),
        2 => "two".to_owned(),
        3 => "three".to_owned(),
        4 => "four".to_owned(),
        5 => "five".to_owned(),
        6 => "six".to_owned(),
        7 => "seven".to_owned(),
        8 => "eight".to_owned(),
        9 => "nine".to_owned(),
        10 => "ten".to_owned(),
        11 => "eleven".to_owned(),
        12 => "twelve".to_owned(),
        13 => "thirteen".to_owned(),
        14 => "fourteen".to_owned(),
        15 => "fifteen".to_owned(),
        16 => "sixteen".to_owned(),
        17 => "seventeen".to_owned(),
        18 => "eighteen".to_owned(),
        19 => "nineteen".to_owned(),
        20 => "twenty".to_owned(),
        30 => "thirty".to_owned(),
        40 => "fourty".to_owned(),
        50 => "fifty".to_owned(),
        60 => "sixty".to_owned(),
        70 => "seventy".to_owned(),
        80 => "eighty".to_owned(),
        90 => "ninety".to_owned(),
        _ => {
            if n < 100 {
                say_mid_size(n)
            } else {
                say_hundred_size(n)
            }
        }
    }
}

fn say_mid_size(n: i32) -> String {
    let last = &n.to_string()[1..2].parse::<i32>().unwrap();
    let first = &n.to_string()[0..1].parse::<i32>().unwrap();
    let mut result = say_until_100(first * 10) + "-" + &say_until_100(*last);
    result = result.trim_end_matches('-').to_string();
    result
}

fn say_hundred_size(n: i32) -> String {
    let last = &n.to_string()[2..3];
    let middle = &n.to_string()[1..2].to_owned();
    let first = *&n.to_string()[0..1].parse::<i32>().unwrap();
    let mut result = say_until_100(first)
        + " hundred "
        + &say_mid_size((middle.to_owned() + last).parse::<i32>().unwrap());
    result = result.trim_end_matches('-').to_string();

    result
}

fn break_number(number: i32) -> Vec<i32> {
    // let mut v = vec![];
    // for (i, c) in number.to_string().chars().rev().enumerate() {
    //     let n = c.to_digit(10).unwrap();
    //     println!("n is {}", n);
    // }
    let n = number.to_string();
    if n.len() <= 3 {
        vec![n[0..].parse::<i32>().unwrap()]
    } else if n.len() <= 6 {
        vec![
            n[0..n.len() - 3 as usize].parse::<i32>().unwrap(),
            n[n.len() - 3..].parse::<i32>().unwrap(),
        ]
    } else if n.len() <= 9 {
        vec![
            n[0..n.len() - 6 as usize].parse::<i32>().unwrap(),
            n[n.len() - 6..n.len() - 3].parse::<i32>().unwrap(),
            n[n.len() - 3..].parse::<i32>().unwrap(),
        ]
    } else if n.len() <= 12 {
        vec![
            n[0..n.len() - 9 as usize].parse::<i32>().unwrap(),
            n[n.len() - 9..n.len() - 6].parse::<i32>().unwrap(),
            n[n.len() - 6..n.len() - 3].parse::<i32>().unwrap(),
            n[n.len() - 3..].parse::<i32>().unwrap(),
        ]
    } else {
        vec![]
    }
}

fn say_with_expos(number: i32) -> String {
    let parts = break_number(number);
    let mut current = parts.len();
    let mut saying = String::new();
    for part in parts {
        saying.push_str(&part.to_string());
        match current {
            4 => saying.push_str(" billion "),
            3 => saying.push_str(" million "),
            2 => saying.push_str(" thousand "),
            1 => saying.push_str(""),
            _ => {}
        };
        current -= 1;
    }
    saying
}

fn say(number: i32) -> String {
    let mut saying = String::new();
    let number_str = number.to_string();
    let number = number.abs();
    if number == 0 {
        return "zero".to_string();
    } else {
        if number_str.starts_with("-") {
            saying.push_str("minus ");
        }

        let parts = break_number(number);
        let mut current = parts.len();
        for part in &parts {
            saying.push_str(&say_until_100(*part));
            match current {
                4 => saying.push_str(" billion "),
                3 => saying.push_str(" million "),
                2 => saying.push_str(" thousand "),
                1 => saying.push_str(""),
                _ => {}
            };
            current -= 1;
        }
    }

    saying = saying.trim().to_string();
    saying = saying.replace("  ", " ");
    if saying.ends_with(" and") {
        saying = saying[..saying.len() - 3].to_string();
        saying = saying.trim().to_string();
    }
    println!("saying is {}", &saying);
    saying
}
fn main() {
    assert_eq!(&say(-1), "minus one");
    assert_eq!(&say(0), "zero");
    assert_eq!(&say(14), "fourteen");
    assert_eq!(&say(50), "fifty");
    assert_eq!(&say(98), "ninety-eight");
    assert_eq!(&say(100), "one hundred");
    assert_eq!(break_number(1), vec![1]);
    assert_eq!(break_number(12), vec![12]);
    assert_eq!(break_number(123), vec![123]);
    assert_eq!(break_number(1234), vec![1, 234]);
    assert_eq!(break_number(12345), vec![12, 345]);
    assert_eq!(break_number(123456), vec![123, 456]);
    assert_eq!(break_number(1234567890), vec![1, 234, 567, 890]);

    assert_eq!(
        &say_with_expos(1234567890),
        "1 billion 234 million 567 thousand 890"
    );

    assert_eq!(&say(1000), "one thousand");
    assert_eq!(&say(1002), "one thousand two");
    assert_eq!(&say(1234567890), "one billion two hundred thirty-four million five hundred sixty-seven thousand eight hundred ninety");
}
