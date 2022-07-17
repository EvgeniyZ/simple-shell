use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::process::Command;

fn main() {
    let mut input = String::new();

    stdout().write("Console started, press command:\n".as_bytes());
    stdin().read_line(&mut input).unwrap();

    let command = input.trim();

    Command::new(command)
        .spawn()
        .unwrap();
}