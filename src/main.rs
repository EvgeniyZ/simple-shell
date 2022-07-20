use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::process::Command;
use std::process::Child;
use std::str::SplitWhitespace;
use std::io::Error;

fn main() {
    loop {
        print!("> ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let parts = input.trim()
            .split_whitespace();

        let child = exec_command(parts);

        match child {
            Ok(mut child) => { child.wait(); },
            Err(e) => eprintln!("{}", e),
        };
    }    
}

fn exec_command(mut parts: SplitWhitespace) -> Result<Child, Error> {
    let command = parts.next().unwrap();
    let args = parts;

    return Command::new(command)
                .args(args)
                .spawn();
}