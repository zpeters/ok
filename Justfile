default: format clippy test build
publish: format clippy-die test build-release copy-release

alias b := build
alias pub := publish
alias r := build-release

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