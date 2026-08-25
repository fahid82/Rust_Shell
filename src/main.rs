use rust_shell::{run, utilities::exit};

fn main() {
    match run() {
        Ok(_) => exit(0),
        Err(error) => {
            eprintln!("Error: {error}");
            exit(1)
        }
    }
}
