use std::fs;

struct Password {
    password: String,
    min: usize,
    max: usize,
    char: char,
}

fn parse_password(line: &str) -> Password {
    let parts: Vec<&str> = line.split(&['-', ' ', ':'][..]).collect();
    Password {
        min: parts[0].parse().unwrap(),
        max: parts[1].parse().unwrap(),
        char: parts[2].chars().next().unwrap(),
        // parts[3] is empty
        password: parts[4].to_string(),
    }
}

fn valid_password(pw: &Password) -> bool {
    let count = pw.password.chars().filter(|&c| c == pw.char).count();
    count >= pw.min && count <= pw.max
}

fn valid_password2(pw: &Password) -> bool {
    (pw.password.chars().nth(pw.min - 1).unwrap() == pw.char) ^ (pw.password.chars().nth(pw.max - 1).unwrap() == pw.char)
}

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");

    let valid_passwords = data.lines()
        .map(parse_password)
        .filter(valid_password)
        .count();
    println!("Valid passwords according to old interpretation: {}", valid_passwords);

    let valid_passwords = data.lines()
        .map(parse_password)
        .filter(valid_password2)
        .count();
    println!("Valid passwords according to new interpretation: {}", valid_passwords);
}
