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


const DEFAULT_ARGS: [&str;4] = ["a", "aa", "s", "n"];

fn dfm() {
    println!(
        "\n╔════════════════════════════════════════════════════╗\n\
     ║     Available Symbols for Password Generation      ║\n\
     ╠════════════════════════════════════════════════════╣\n\
     ║ a   or lowercase   → abcdefghijklmnopqrstuvwxyz    ║\n\
     ║ aa  or uppercase   → ABCDEFGHIJKLMNOPQRSTUVWXYZ    ║\n\
     ║ n   or number      → 0123456789:                   ║\n\
     ║ s   or symbols     → #$%^*_-=+~;[<(:&|@:)>];~.?!   ║\n\
     ║ r   or arrows      → →←↑↓↔↕↩↪                      ║\n\
     ║ m   or math        → ∑∆∞∫∏√≠≈±∂                    ║\n\
     ║ mm  or mood        → 😁😇🙂🙃🥳🤠😎                ║\n\
     ╠════════════════════════════════════════════════════╣\n\
     ║ Examples:                                          ║\n\
     ║ - Lowercase and uppercase letters:    a aa         ║\n\
     ║ - Numbers and special symbols:        n s          ║\n\
     ║ - Math and mood symbols:              m mm         ║\n\
     ╠════════════════════════════════════════════════════╣\n\
     ║ You can combine these options to create complex    ║\n\
     ║ passwords!                                         ║\n\
     ╚════════════════════════════════════════════════════╝\n"
    );
}

fn create_ch_pool(args: &[&str]) -> String {
    let mut list = args.iter().map(|&x| match x {
        "a" | "lower" => LOW_CASE,
        "aa" | "upper" => UP_CASE,
        "n" | "nums" => NUMBERS,
        "s" | "symbols" => SYMBOLS,
        "r" | "arrows" => ARROWS,
        "m" | "math" => MATH_SYMBOLS,
        "mm" | "mood" => MOOD_SYMBOLS,
        custom => custom,
    }).collect::<Vec<&str>>();

    list.shuffle(&mut thread_rng());
    list.concat()
}

fn create_password(pool: &str, n: usize) -> String {
    let mut rng = thread_rng();
    repeat_with(|| pool.chars().choose(&mut rng).unwrap())
        .take(n).collect()
}

fn main() {
    //dfm();
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
    let pool = create_ch_pool(if args.is_empty() { &DEFAULT_ARGS } else { args });
    let password = create_password(&pool, lens);

    println!("Yours generated password[{lens}]: {password}");
    // println!("Unique pool: {}", pool);
}
