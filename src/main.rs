use rand::{Rng,seq::SliceRandom, thread_rng};
use std::iter::repeat_with;
use std::io::{stdout, stdin, Write};

const LOW_CASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UP_CASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const SYMBOLS: &str = "#$%^*_-=+~;[<{(:&|@:)}>];~.?!";
const NUMBERS: &str = "0123456789";

const DEFAULT_ARGS: [&str;4] = ["a", "aa", "s", "n"];

fn generate_line(args: &[&str]) -> String {
    let mut rng = thread_rng();
    let mut list = args.iter().map(|&x| match x {
        "a" => LOW_CASE,
        "aa" => UP_CASE,
        "s" => SYMBOLS,
        "n" => NUMBERS,
        custom => custom,
    }).collect::<Vec<&str>>();

    list.shuffle(&mut rng);
    list.concat()
}

fn generate_pass(line: &str, max: usize) -> String {
    let mut rng = thread_rng();
    repeat_with(|| { line.chars().nth(rng.gen_range(0..line.len())).unwrap() })
        .take(max)
        .collect()
}

fn main() {
    let mut input: String = String::new();
    print!(">> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();

    let input_items = input.split_whitespace().collect::<Vec<&str>>();

    if input_items.len().eq(&0usize) {
        eprintln!("Empty input!");
        return;
    }
    // first arg always number
    let password_len = input_items[0].parse::<usize>()
        .expect("invalid value for password length!");

    let args = &input_items[1..];
    let line = match &args.len() {
        0 => generate_line(&DEFAULT_ARGS), // default
        _ => generate_line(&args), // custom [ .., .., ... ]
    };
    let password = generate_pass(&line, password_len);

    println!("Yours generated password[{password_len}]: {password}");
    // println!("Unique generated line: {}", new_line);

}
