use std::io;
use std::io::Write;
use crate::user_handling::*;

pub fn run() {
    loop {
        print!("KevShell$> ");
        io::stdout().flush().unwrap();

        let user_in = input();
        let command = match user_in {
            Ok(command) => command,
            Err(e) => {
                eprintln!("Could not get user input. Something went wrong: {e:?}");
                std::process::exit(1);
            }
        };

        let mut parser = process_input(&command);
        let tokens = match parser.lex_output() {
            Some(token_list) => token_list,
            None => {
                eprintln!("No tokens found. Aborting");
                std::process::exit(1);
            }
        };

        process_output(tokens)

    }
}