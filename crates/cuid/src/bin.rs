use cuid::{one_off_cuid1, one_off_cuid1_slug};
use std::{
    env::{self, Args},
    process::exit,
};

/// Generate a new CUID and print it to stdout
pub fn main() {
    let args: CuidArgs = env::args().into();

    let id = match args.version {
        CuidVersion::V1 => {
            if args.slug {
                one_off_cuid1_slug()
            } else {
                one_off_cuid1()
            }
        }
        CuidVersion::V2 => {
            if args.slug {
                // construct a v2 cuid with the same length as cuid1 slugs
                cuid2::CuidConstructor::new().with_length(10).create_id()
            } else {
                cuid2::create_id()
            }
        }
    };

    println!("{}", id);
}

const HELP: &str = r#"Usage: cuid [OPTION]...
Generate and print a CUID.

Options:
  -h, --help     display this help and exit
  -v, --version  display version information and exit
  --cuid <1|2>   generate a CUID/slug using the specified version (default 1)
  --slug         generate a slug instead of a full CUID"#;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
enum CuidVersion {
    V1,
    V2,
}

/// Commandline arguments for the CUID binary
#[derive(Debug)]
struct CuidArgs {
    /// Whether to produce a slug instead of a CUID
    slug: bool,
    version: CuidVersion,
}
impl From<Args> for CuidArgs {
    fn from(args: Args) -> Self {
        let mut slug = false;
        let mut version = CuidVersion::V1;

        // start on 1 to skip binary name.
        let mut idx = 1;
        let args = args.collect::<Vec<_>>();
        loop {
            match args.get(idx) {
                None => {
                    break;
                }
                Some(arg) => match arg.as_str() {
                    "-h" | "--help" => {
                        println!("{}", HELP);
                        exit(0);
                    }
                    "-v" | "--version" => {
                        println!("{}", VERSION);
                        exit(0);
                    }
                    "--slug" => slug = true,
                    // yeah yeah I should probably just use clap at this point,
                    // but we'll get to it eventually
                    "--cuid" => {
                        idx += 1;
                        match args.get(idx) {
                            None => print_error_and_exit("--cuid requires an argument"),
                            Some(arg) => match arg.as_str() {
                                "1" => version = CuidVersion::V1,
                                "2" => version = CuidVersion::V2,
                                _ => {
                                    print_error_and_exit(
                                        "unrecognized cuid version, must be one of: 1|2",
                                    );
                                }
                            },
                        }
                    }
                    arg if arg.starts_with("--cuid=") => match arg.split_once("=").unwrap().1 {
                        "1" => version = CuidVersion::V1,
                        "2" => version = CuidVersion::V2,
                        _ => {
                            print_error_and_exit("unrecognized cuid version, must be one of: 1|2");
                        }
                    },
                    _ => {
                        print_error_and_exit(&format!("unrecognized argument {}", arg));
                    }
                },
            }
            idx += 1;
        }

        CuidArgs { slug, version }
    }
}

fn print_error_and_exit(msg: &str) {
    println!("error: {}", msg);
    println!();
    println!("{}", HELP);
    exit(1);
}
