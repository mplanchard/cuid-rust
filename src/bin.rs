use cuid::cuid;
use std::process::exit;

/// Generate a new CUID and print it to stdout
pub fn main() {
    match cuid() {
        Ok(id) => println!("{}", id),
        Err(err) => {
            eprintln!("{:?}", err);
            exit(1)
        }
    }
}
