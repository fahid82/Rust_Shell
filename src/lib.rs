mod utilities;

#[allow(unused_imports)]
use std::io::{self, Write};

use anyhow::Result;

use crate::utilities::get_user_input;

pub fn run() -> Result<()> {
    print!("$ ");
    io::stdout().flush().unwrap();

    let user_input: String = get_user_input()?;
    println!("User input: ${user_input}");

    Ok(())
}
