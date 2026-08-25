mod commands;
mod errors;
mod utilities;

#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

use anyhow::Result;

use crate::{
    commands::Command,
    errors::CustomError,
    utilities::{get_user_input, print_error},
};

pub fn run() -> Result<()> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let user_input = get_user_input()?;
        let command = Command::from(user_input.as_str());

        match command {
            Command::Exit => process::exit(0),
            Command::NotFound(command) => print_error(CustomError::CommandNotFound(command)),
        }
    }
    #[allow(unreachable_code)]
    Ok(())
}
