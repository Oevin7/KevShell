use crate::command_lex::command_lex::*;
use crate::commands::{Command, Utils};
use crate::utils::*;
use std::io;

///Gets input from the user
pub fn input() -> Result<String, io::Error> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    Ok(input)
}

///Creates the parser to split the input into tokens, that can then turn into a command
pub fn process_input(commands : &str) -> CommandLexer {
    let mut parser = CommandLexer::new(commands);
    parser.get_output();

    parser

}
///Creates a new command from the user input, and then returns that command to be executed later.
pub fn process_output(tokens : Vec<Tokens>) -> Command {

    let mut command = Command::new();

    for token in 0..tokens.len() {
        match tokens[token].clone() {
            Tokens::Command(comm) => {
                command.command = Some(comm);
            }
            Tokens::Flag(flag) => {
                command.flags.push(flag);
            }
            //Can be a file path or other type of argument.
            Tokens::FilePath(path) => {
                command.arguments.push(path);
            }
            Tokens::Wildcard(extension) => {
                command.arguments.push(extension)
            }
            Tokens::AppendOut => command.append_output = true,
            Tokens::Background => command.background = true,
            Tokens::Pipe => {
                let token_list = tokens[token + 1..tokens.len()].to_vec();
                let next_command = Box::new(process_output(token_list));
                command.pipe_to = Some(next_command);
                break;
            }
            _ => eprintln!("Yet to add")
        }
    }
    command

}

///This executes the command returned from the process_output function.
pub fn execute_commands(command: Command) {
    let mut current_command = command.clone();
    match current_command.command {
        Some(Utils::Print) => {
            for arg in command.arguments {
                print(arg);
            }
        }
        Some(Utils::List) => list(current_command.arguments, current_command.flags),
        Some(Utils::Exit) => std::process::exit(0),
        _ => eprintln!("Not implemented yet")
    }

    current_command = match current_command.pipe_to {
        Some(next_command) => *next_command,
        None => {
            return;
        }
    };

    execute_commands(current_command);

}