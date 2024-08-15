//! Provide a simple binary for generating v2 cuids

use std::str::FromStr;
use std::{env, process::exit};

use cuid2::DEFAULT_LENGTH;

struct ParsedArgs {
    pub cuid_length: u16,
}

/// Generates a new CUID and print it to stdout
pub fn main() {
    let parsed_args = parse_args();

    println!(
        "{}",
        cuid2::CuidConstructor::new()
            .with_length(parsed_args.cuid_length)
            .create_id()
    );
}

const HELP: &str = r#"Usage: cuid2 [OPTION]...
Generate and print a CUID.

Options:
  -h, --help            display this help and exit
  -v, --version         display version information and exit
  -l, --length [LENGTH] set the length of the CUID (default: 24)"#;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn parse_args() -> ParsedArgs {
    // defaults here
    let mut parsed_args = ParsedArgs {
        cuid_length: DEFAULT_LENGTH.into(),
    };

    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("{}", HELP);
                exit(0);
            }
            "-v" | "--version" => {
                println!("{}", VERSION);
                exit(0);
            }
            "-l" | "--length" /* takes 1 arg (integer) */ => {
                let length_str = args.next().unwrap_or_else(|| {
                    eprintln!("error: {} expects an argument", arg);
                    eprintln!();
                    eprintln!("{}", HELP);
                    exit(1)
                });

                if let Ok(length) = u16::from_str(&length_str) {
                    parsed_args.cuid_length = length
                } else {
                    eprintln!("error: length '{}' must be an integer", length_str);
                    eprintln!();
                    eprintln!("{}", HELP);
                    exit(1);
                }
            }
            other => {
                eprintln!("error: unrecognized argument '{}'", other);
                eprintln!();
                eprintln!("{}", HELP);
                exit(1);
            }
        }
    }

    parsed_args
}
