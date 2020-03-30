extern crate rand;
use rand::seq::SliceRandom;
use rand::Rng;
use std::io::stdin;
use std::io::Write;

fn main() {
    generate_password()
}

fn generate_password() {
    let pwd_length: u8 = input_pwd_length();
    let mut chosen_characters = String::new();
    input_special_characters(&mut chosen_characters);
    input_digits(&mut chosen_characters);
    input_lowercase_characters(&mut chosen_characters);
    input_uppercase_characters(&mut chosen_characters);
    if chosen_characters.len() == 0 {
        println!("No options chosen");
        return;
    };
    let password: String = (0..pwd_length)
        .map(|_| {
            let idx = random_index(&chosen_characters);
            chosen_characters.as_bytes()[idx] as char
        })
        .collect();
    print!("Here is your password: {}", password);
    std::io::stdout().flush().unwrap();
}

fn input_pwd_length() -> u8 {
    // Choose password length
    print!("Choose password length: ");
    std::io::stdout().flush().unwrap();
    let mut input_length_pwd = String::new();
    let _ = stdin().read_line(&mut input_length_pwd).unwrap();
    let value: u8 = input_length_pwd
        .trim()
        .parse()
        .expect("You didn't type digits");
    match value {
        8..=54 => (),
        0..=7 => panic!(
            "Password length needs to be higher or equal to 8 characters, length entered: {}",
            value
        ),
        _ => panic!(
            "Password length needs to be lower or equal to 54 characters, length entered {}",
            value
        ),
    };
    value
}

fn input_special_characters(cc: &mut String) {
    const SPECIAL_CHARACTERS: &str = "~!@#$%^&*()-_+={}[]:;<>,.?";
    // Choose whether include symbols or not
    print!("Do you want to include symbols(y/n): ");
    std::io::stdout().flush().unwrap();
    let mut input_include_symbols = String::new();
    let _ = stdin().read_line(&mut input_include_symbols).unwrap();
    match input_include_symbols.trim() {
        "y" | "yes" => cc.push_str(SPECIAL_CHARACTERS),
        "n" | "no" => (),
        n @ _ => panic!("You didn't type correctly, typed: {}", n),
    };
}

fn input_digits(cc: &mut String) {
    const DIGITS: &str = "0123456789";
    // Choose whether includes numbers or not
    print!("Do you want to include number(y/n): ");
    std::io::stdout().flush().unwrap();
    let mut input_include_numbers = String::new();
    let _ = stdin().read_line(&mut input_include_numbers).unwrap();
    match input_include_numbers.trim() {
        "y" | "yes" => cc.push_str(DIGITS),
        "n" | "no" => (),
        n @ _ => panic!("You didn't type correctly, typed: {}", n),
    };
}

fn input_lowercase_characters(cc: &mut String) {
    const LOWERCASE_CHARACTERS: &str = "abcdefghijklmnopqrstuvwxyz";
    // Choose whether lowercase characters or not
    print!("Do you want to include lowercase characters(y/n): ");
    std::io::stdout().flush().unwrap();
    let mut input_include_lowercase_characters: String = String::new();
    let _ = stdin()
        .read_line(&mut input_include_lowercase_characters)
        .unwrap();
    match input_include_lowercase_characters.trim() {
        "y" | "yes" => cc.push_str(LOWERCASE_CHARACTERS),
        "n" | "no" => (),
        n @ _ => panic!("You didn't type correctly, typed: {}", n),
    };
}

fn input_uppercase_characters(cc: &mut String) {
    const UPPERCASE_CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    // Choose whether uppercase characters or not
    print!("Do you want to include uppercase characters(y/n): ");
    std::io::stdout().flush().unwrap();
    let mut input_include_uppercase_characters: String = String::new();
    let _ = stdin()
        .read_line(&mut input_include_uppercase_characters)
        .unwrap();
    match input_include_uppercase_characters.trim() {
        "y" | "yes" => cc.push_str(UPPERCASE_CHARACTERS),
        "n" | "no" => (),
        n @ _ => panic!("You didn't type correctly, typed: {}", n),
    };
}

fn random_index(cc: &str) -> usize {
    let mut rng = rand::thread_rng();
    let mut bytes = cc.to_owned().into_bytes();
    bytes.shuffle(&mut rng);
    let idx = rng.gen_range(0, bytes.len());
    idx
}
