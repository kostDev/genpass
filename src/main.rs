use rand::{seq::{SliceRandom, IteratorRandom}, thread_rng};
use std::io::{stdout, stdin, Write};
use std::iter::repeat_with;
use genpass::{ consts, cli::handle_cli };

fn create_ch_pool(args: &[&str]) -> String {
    let mut pool = args.iter().map(|&x| match x {
        "a" | "lower" => consts::LOW_CASE,
        "aa" | "upper" => consts::UP_CASE,
        "n" | "nums" => consts::NUMBERS,
        "s" | "symbols" => consts::SYMBOLS,
        "r" | "arrows" => consts::ARROWS,
        "m" | "math" => consts::MATH_SYMBOLS,
        "mm" | "mood" => consts::MOOD_SYMBOLS,
        custom => custom,
    }).collect::<Vec<&str>>();

    pool.shuffle(&mut thread_rng());
    pool.concat()
}

fn create_password(pool: &str, n: usize) -> String {
    let mut rng = thread_rng();
    repeat_with(|| pool.chars().choose(&mut rng).unwrap())
        .take(n).collect()
}

fn main() {
    handle_cli();

    let mut input: String = String::new();
    print!(">> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();

    let tokens = input.split_whitespace().collect::<Vec<&str>>();

    if tokens.is_empty() {
        eprintln!("Empty input! Please provide a password length or/and optional character set!");
        return;
    }
    // first arg always number as password length
    let lens = tokens[0].parse::<usize>().expect("Invalid value for password length!");
    let args = &tokens[1..];
    let pool = create_ch_pool(if args.is_empty() { &consts::DEFAULT_ARGS } else { args });
    let password = create_password(&pool, lens);

    println!("Yours generated password[{lens}]: {password}");
    // println!("Unique pool: {}", pool);
}
