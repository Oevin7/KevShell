#[derive(Debug)]
pub enum Commands {
    Exit,
    List,
    Touch, ///Touch in traditional shells
    MakeDirectory,
    Print, ///Echo in traditional shells
    Concatenate, ///Cat in traditional shells
    Find, ///Find in traditional shells
    Grep, ///Grep in traditional shells
}

#[derive(Debug)]
pub struct Command {
    command : Commands,
    flags : Vec<String>,
    arguments : Vec<String>,
    input_redirection: Option<String>,
    output_redirection: Option<String>,
    append_output: bool,
    background: bool,
    pipe_to: Option<Box<Command>>,
}

impl Command {
    pub fn new(command: Commands) -> Command {
        Command {
            command,
            flags : vec![],
            arguments : vec![],
            input_redirection : None,
            output_redirection : None,
            append_output : false,
            background : false,
            pipe_to : None
        }
    }
}