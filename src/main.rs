use rust_shell::run;
use std::process;

fn main() {
    match run() {
        Ok(_) => process::exit(0),
        Err(error) => {
            eprintln!("Error: {error}");
            process::exit(1)
        }
    }
}
