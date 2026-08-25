mod commands;
mod errors;
pub mod utilities;

#[allow(unused_imports)]
use std::io::{self, Write};

use anyhow::{Context, Result};

use crate::{
    commands::Command,
    errors::CustomError,
    utilities::{get_command, print_error, print_prompt},
};

pub fn run() -> Result<()> {
    loop {
        print_prompt();
        let command = get_command().context("Getting command")?;

        match command {
            Command::Exit => break,
            Command::NotFound(command) => print_error(CustomError::CommandNotFound(command)),
        }
    }
    Ok(())
}
