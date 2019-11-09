extern crate cuid;

use cuid::cuid;
use std::process::exit;


pub fn main() {
    match cuid() {
        Ok(id) => println!("{}", id),
        Err(err) => {
            eprintln!("{:?}", err);
            exit(1)
        }
    }
}
