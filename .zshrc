# Defining Local Aliases
alias build="rustfmt src/*.rs; echo 'Compiling for macOS...'; cargo build --release && echo 'Compiling for Linux...'; cargo build --release --target=x86_64-unknown-linux-gnu";
alias run="rustfmt src/*.rs; cargo run"