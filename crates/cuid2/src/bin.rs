//! Provide a simple binary for generating v2 cuids

use std::{env, process::exit};
use std::str::FromStr;

struct ParsedArgs {
    pub cuid_length: u16
}

/// Generates a new CUID and print it to stdout
pub fn main() {
    let parsed_args = parse_args();

    println!("{}", cuid2::CuidConstructor::new()
        .with_length(parsed_args.cuid_length)
        .create_id());
}

const HELP: &str = r#"Usage: cuid2 [OPTION]... [LENGTH]
Generate and print a CUID. The default LENGTH is 24.

Options:
  -h, --help     display this help and exit
  -v, --version  display version information and exit"#;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn parse_args() -> ParsedArgs {
    // defaults here
    let mut parsed_args = ParsedArgs {
        cuid_length: 24
    };

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
        length_str => {
            if let Ok(length) = u16::from_str(length_str) {
                parsed_args.cuid_length = length
            } else {
                println!("error: unrecognized argument {}", arg);
                println!();
                println!("{}", HELP);
                exit(1);
            }
        }
    });

    parsed_args
}
