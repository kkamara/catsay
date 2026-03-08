# catsay

This repository follows chapter 2 of Shing Lyu's book Practical Rust Projects Second Edition.

## Usage

```bash
cargo run -- --help
cargo run -- "Hello I'm a cat"
cargo run -- "Hello" -f catfile.txt
cargo run -- -d
```

## Printing to STDERR

```bash
cargo run "woof" 1> stdout.txt 2> stderr.txt
```