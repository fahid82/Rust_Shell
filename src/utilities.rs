use anyhow::{Context, Result};
use std::fmt::Display;
use std::io::stdin;

pub fn get_user_input() -> Result<String> {
    let mut user_input: String = String::new();
    stdin()
        .read_line(&mut user_input)
        .context("Reading user input")?;
    Ok(user_input.trim().to_owned())
}

pub fn print_error(message: impl Display) {
    eprintln!("{message}");
}
