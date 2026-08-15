mod errors;
mod utilities;

use crate::errors::CustomError;
use crate::utilities::print_error;

#[allow(unused_imports)]
use std::io::{self, Write};

use anyhow::Result;

use crate::utilities::get_user_input;

pub fn run() -> Result<()> {
    print!("$ ");
    io::stdout().flush().unwrap();

    let user_input: String = get_user_input()?;

    let error: CustomError = CustomError::CommandNotFound(user_input);

    print_error(error);

    Ok(())
}
