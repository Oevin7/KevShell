pub mod command_lex {
    use crate::commands::Utils::{Concatenate, Exit, Find, Grep, List, MakeDirectory, Print, Touch};
    use crate::commands::Utils;

    /// These are the tokens that the input is broken down into
    #[derive(Debug, Clone)]
    pub enum Tokens {
        Command(Utils),
        FilePath(String),
        Wildcard(String),
        Quote(String),
        Variable(String),
        RedirectIn,
        RedirectOut,
        AppendOut,
        Semicolon,
        NewLine,
        Pipe,
        Background,
        EndOfFile,
        LogicalAnd,
        LogicalOr,
        Flag(String),
        Error(String),
    }

    /// The struct that contains the user input that needs to be lexed.
    /// It also contains the output which was broken down into tokens.
    pub struct CommandLexer<'a> {
        pub input : &'a str,
        pub output : Option<&'a str>,
    }

    ///Contains the functions for the lexer. A lexer needs to be created with input.
    impl<'a> CommandLexer<'a> {
        pub fn new(input : &'a str) -> CommandLexer<'a> {
            CommandLexer {
                input,
                output : None
            }
        }

        pub fn get_output(&mut self) {
            self.output = Some(self.input);
        }

        /// lex_output turns the input into various different tokens that can then be used
        /// to run utilities and functions with the shell.
        pub fn lex_output(self) -> Option<Vec<Tokens>> {
            let mut tokens = Vec::new();
            let command_output = self.output?;

            let mut char = command_output.chars().peekable();

            while let Some(comm_char) = char.next() {
                match comm_char {
                    ' ' | '\t' => continue,
                    '<' => tokens.push(Tokens::RedirectIn),
                    '>' => {
                        if char.peek() == Some(&'>') {
                            char.next();
                            tokens.push(Tokens::AppendOut)
                        } else {
                            tokens.push(Tokens::RedirectOut)
                        }
                    }
                    ';' => tokens.push(Tokens::Semicolon),
                    '|' => {
                        if char.peek() == Some(&'|') {
                            tokens.push(Tokens::LogicalOr);
                        } else {
                            tokens.push(Tokens::Pipe);
                        }
                    },
                    '\n' => tokens.push(Tokens::NewLine),
                    '&' => {
                        if char.peek() == Some(&'&') {
                            tokens.push(Tokens::LogicalAnd);
                        } else {
                            tokens.push(Tokens::Background);
                        }
                    },
                    '"' | '\'' => {
                        let mut full_path = String::new();
                        let quote = comm_char;
                        while let Some(quoted) = char.next() {
                            if quoted == quote {
                                break
                            } else {
                                full_path.push(quoted)
                            }
                        }
                        tokens.push(Tokens::FilePath(full_path))
                    }
                    '$' => {
                        let mut variable = String::from(comm_char);

                        while let Some(chars) = char.next() {
                            if !chars.is_alphanumeric() && chars != '_' {
                                char.next_back();
                                break;
                            }
                            variable.push(chars);
                        }

                        tokens.push(Tokens::Variable(variable));

                    }
                    '*' | '?' => {
                        let mut path = String::from(comm_char);

                        while let Some(chars) = char.next() {
                            if chars == ' ' {
                                break
                            }
                            path.push(chars);
                        }
                        tokens.push(Tokens::Wildcard(path))
                    }
                    '-' => {
                        let mut flag = String::from(comm_char);

                        if char.peek() == Some(&'-') {
                            char.next();
                            flag.push(comm_char)
                        }

                        while let Some(chars) = char.next() {
                            if chars.is_whitespace() {
                                break
                            }
                            flag.push(chars);
                        }
                        tokens.push(Tokens::Flag(flag));
                    }
                    _ => {

                        if comm_char.is_alphanumeric() || comm_char == '_' {
                            let mut command = String::from(comm_char);

                            while let Some(chars) = char.next() {
                                if chars.is_whitespace() {
                                    break
                                }
                                command.push(chars)
                            }

                            match command.as_str() {
                                "exit" | "ex" => tokens.push(Tokens::Command(Exit)),
                                "ls" => tokens.push(Tokens::Command(List)),
                                "touch" | "tc" => tokens.push(Tokens::Command(Touch)),
                                "mkdir" | "md" => tokens.push(Tokens::Command(MakeDirectory)),
                                "pr" | "print" | "echo" => tokens.push(Tokens::Command(Print)),
                                "ct" | "cat" | "concatenate" => tokens.push(Tokens::Command(Concatenate)),
                                "fd" | "find" => tokens.push(Tokens::Command(Find)),
                                "grp" | "grep" => tokens.push(Tokens::Command(Grep)),
                                _ => tokens.push(Tokens::FilePath(command)),
                            }

                        }
                    },
                }
            }
            Some(tokens)
        }
    }
}