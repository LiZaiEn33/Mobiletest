# Mobiletest

A simple Rust test environment for coding in GitHub Codespaces on iPad.

## Getting Started

This repository contains a basic Rust project that you can use to test and learn Rust programming in GitHub Codespaces.

### Prerequisites

Rust is already installed in this Codespace environment. You can verify by running:
```bash
rustc --version
cargo --version
```

### Building the Project

To build the project, run:
```bash
cargo build
```

### Running the Program

To run the program, use:
```bash
cargo run
```

### Running Tests

To run the tests, use:
```bash
cargo test
```

### Project Structure

- `src/main.rs` - The main source file containing example code and tests
- `Cargo.toml` - The project configuration file
- `target/` - Build artifacts (excluded from git)

## Testing Your Code

Feel free to modify `src/main.rs` to test your own Rust code. The file includes:
- A basic `main()` function
- Example functions (`add_numbers`, `greet`)
- Unit tests to demonstrate testing in Rust

## Quick Commands

- `cargo build` - Compile the project
- `cargo run` - Compile and run the project
- `cargo test` - Run all tests
- `cargo check` - Quick compile check without producing executables
- `cargo fmt` - Format your code according to Rust style guidelines
- `cargo clippy` - Run linter for additional code quality checks

## Learning Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)

Happy coding! 🦀