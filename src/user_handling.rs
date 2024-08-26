use std::io;
use crate::command_lex::command_lex::*;
use crate::commands::Command;

pub fn input() -> Result<String, io::Error> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    Ok(input)
}

pub fn process_output(tokens : Vec<Tokens>) {



}

pub fn process_input(commands : &str) -> CommandLexer {
    let mut parser = CommandLexer::new(commands);
    parser.get_output();

    parser

}