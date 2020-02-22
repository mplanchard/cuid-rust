use std::process::exit;
use cuid::cuid;


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
