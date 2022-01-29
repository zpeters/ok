default: format clippy test build
release: format clippy-die test build-release copy-release

alias b := build
alias r := release

build:
    cargo build
build-release:
    cargo build --release
clippy:
    cargo clippy
clippy-die:
    cargo clippy -- -D warnings
copy-release:
    cp ./target/release/ok ~/bin
doc:
    cargo doc 
format:
    cargo fmt
test:
    cargo test