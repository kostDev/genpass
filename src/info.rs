
pub fn display_help() {
    println!("═════════════════════════════════════════════════════════════");
    println!("                      Password Generator Help               ");
    println!("═════════════════════════════════════════════════════════════");
    println!(" Use the following options to customize your password:      ");
    println!(" ┌───────────────┬───────────────────────────────────────┐");
    println!(" │ Option        │ Description                           │");
    println!(" ├───────────────┼───────────────────────────────────────┤");
    println!(" │ a             │ Lowercase letters (a-z)               │");
    println!(" │ aa            │ Uppercase letters (A-Z)               │");
    println!(" │ n             │ Numbers [0-9]                         │");
    println!(" │ s             │ Symbols (e.g., @, #, %, &, etc.)      │");
    println!(" │ r             │ Arrows (→, ←, ↑, ↓, etc.)             │");
    println!(" │ m             │ Math symbols (e.g., ∑, ∆, ∞)          │");
    println!(" │ mm            │ Mood symbols (e.g., 😁, 🙂, 🙃)       │");
    println!(" │ custom        │ Custom string of characters           │");
    println!(" └───────────────┴───────────────────────────────────────┘");
    println!(" Example usage:");
    println!("  genpass 16 a n s               -> Generates a 16-char password with lowercase, numbers, and symbols.");
    println!("  genpass 12 a aa n              -> Generates a 12-char password with lowercase, uppercase, and numbers.");
    println!("  genpass 20 m mm                -> Generates a 20-char password with math and mood symbols.");
    println!("  genpass 8 ^@_@^               -> Generates an 8-char password from your custom character set.");
    println!("═════════════════════════════════════════════════════════════");
}
