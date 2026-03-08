# catsay

This repository follows chapter 2 of Shing Lyu's book Practical Rust Projects Second Edition.

## Usage

```bash
cargo run -- --help
cargo run -- "Hello I'm a cat"
# Reading a cat image from a file
cargo run -- "Hello" -f catfile.txt
# A dead cat
cargo run -- -d
# Piping text to the program
echo -n "Hello World" | cargo run -- --stdin
# Printing to STDERR
cargo run "woof" 1> stdout.txt 2> stderr.txt
```