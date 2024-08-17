# lists recipes
default:
    @just --list

build:
    cargo build

run:
    cargo run

install:
    cargo install --path .
