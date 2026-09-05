pub enum Command {
    Exit,
    NotFound(String),
    Echo(String),
}

impl From<(String, String)> for Command {
    fn from((command, arguments): (String, String)) -> Self {
        match command.as_str() {
            "echo" => Self::Echo(arguments),
            "exit" => Self::Exit,
            _ => Self::NotFound(command),
        }
    }
}
