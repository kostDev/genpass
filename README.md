# genpass [![Rust](https://github.com/kostDev/genpass/actions/workflows/rust.yml/badge.svg)](https://github.com/kostDev/genpass/actions/workflows/rust.yml)

`genpass` — is a simple utility to generate random passwords based on a user-specified length. The program is built on Rust and provides a quick and convenient way to generate passwords from random characters.

## Implementation

- Random password generation from Latin characters (upper and lower case), numbers and special characters.
- A simple command to create a password of `any` length.
- Suitable for quick use in the command line.
- Use custom text patterns

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) ~> 1.82.0.

## Installation

1. Clone the repository or download the source code:

   ```bash
   git clone https://github.com/kostDev/genpass.git
   cd genpass
   run --package genpass --bin genpass --quiet
    ```

2. Build the project in release mode:

    ```bash
    cargo build --release
    ```

3. Make the utility available globally (optional):

    ```bash 
   sudo cp target/release/genpass /usr/local/bin/
    ```
   
## Launching

```bash
genpass
```
examples:
```text
example 1: default 'a' 'n' 's'

➜ genpass
>> 24 a n s
Yours generated password[24]: 5=~v;4cv1f;1.8nfto+.+;5@
------------------------------------------------------------
example 2: 'custom user pattern'

➜ genpass
>> 24 @#$%^.
Yours generated password[24]: %@#@@#@.$%@.$$.#^%@@@.@.
------------------------------------------------------------
example 3: combining 

>> 24 a n @_@_@:
Yours generated password[24]: @t_3d@27lo8gx1c1b2v@@o4d
```

where:
- a: lowercase `[a-z]`
- aa: uppercase `[A-Z]`
- s: symbols: `[#$%^*_-=+~;[<{(:&|@:)}>];~.?!]`
- n: numbers `[0-9]`