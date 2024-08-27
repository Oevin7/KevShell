#[derive(Debug, Clone, Copy)]
pub enum Utils {
    Exit,
    List,
    Touch,
    MakeDirectory,
    Print,
    Concatenate,
    Find,
    Grep,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub command : Option<Utils>,
    pub flags : Vec<String>,
    pub arguments : Vec<String>,
    pub input_redirection: Option<String>,
    pub output_redirection: Option<String>,
    pub append_output: bool,
    pub background: bool,
    pub pipe_to: Option<Box<Command>>,
}

impl Command {
    pub fn new() -> Command {
        Command {
            command : None,
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