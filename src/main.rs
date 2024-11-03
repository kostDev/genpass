use rand::{seq::{SliceRandom, IteratorRandom}, thread_rng};
use std::iter::repeat_with;
use std::io::{stdout, stdin, Write};

const LOW_CASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UP_CASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";
const SYMBOLS: &str = "#$%^*_-=+~;[<{(:&|@:)}>];~.?!";
const ARROWS: &str = "→←↑↓↔↕↩↪";
const MATH_SYMBOLS: &str = "∑∆∞∫∏√≠≈±∂";
const MOOD_SYMBOLS: &str = "😁😇🙂🙃🥳🤠😎";
// const DOMI_SYMBOLS: &str = "🁣🁫🁳🁻🂃🂋🂓";


const DEFAULT_ARGS: [&str;4] = ["a", "aa", "s", "n"];

fn generate_line(args: &[&str]) -> String {
    let mut rng = thread_rng();
    let mut list = args.iter().map(|&x| match x {
        "a" | "lower" => LOW_CASE,
        "aa" | "upper" => UP_CASE,
        "n" | "number" => NUMBERS,
        "s" | "symbols" => SYMBOLS,
        "r" | "arrows" => ARROWS,
        "m" | "math" => MATH_SYMBOLS,
        "mm" | "mood" => MOOD_SYMBOLS,
        // "dm" | "domi" => DOMI_SYMBOLS,
        custom => custom,
    }).collect::<Vec<&str>>();

    list.shuffle(&mut rng);
    list.concat()
}

fn generate_pass(line: &str, max: usize) -> String {
    let mut rng = thread_rng();
    repeat_with(|| line.chars().choose(&mut rng).unwrap())
        .take(max)
        .collect()
}

fn main() {
    let mut input: String = String::new();
    print!(">> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();

    let input_items = input.split_whitespace().collect::<Vec<&str>>();

    if input_items.is_empty() {
        eprintln!("Empty input! Please provide a password length or/and optional character sets.");
        return;
    }
    // first arg always number
    let password_len = input_items[0].parse::<usize>()
        .expect("invalid value for password length!");

    let args = &input_items[1..];
    let line = generate_line(if args.is_empty() { &DEFAULT_ARGS } else { args });
    let password = generate_pass(&line, password_len);

    println!("Yours generated password[{password_len}]: {password}");
    // println!("Unique generated line: {}", new_line);
}
