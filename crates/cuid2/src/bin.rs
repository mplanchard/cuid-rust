//! Provide a simple binary for generating v2 cuids

use cuid2::create_id;

use std::{env, process::exit};

/// Generates a new CUID and print it to stdout
pub fn main() {
    parse_args();

    println!("{}", create_id());
}

const HELP: &str = r#"Usage: cuid2 [OPTION]...
Generate and print a CUID.

Options:
  -h, --help     display this help and exit
  -v, --version  display version information and exit"#;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn parse_args() {
    // The first argument should be the binary name. Skip it.
    env::args().skip(1).for_each(|arg| match arg.as_str() {
        "-h" | "--help" => {
            println!("{}", HELP);
            exit(0);
        }
        "-v" | "--version" => {
            println!("{}", VERSION);
            exit(0);
        }
        _ => {
            println!("error: unrecognized argument {}", arg);
            println!();
            println!("{}", HELP);
            exit(1);
        }
    });
}
