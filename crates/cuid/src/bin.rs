use cuid::{one_off_cuid1, one_off_cuid1_slug};
use std::{
    env::{self, Args},
    process::exit,
};

/// Generate a new CUID and print it to stdout
pub fn main() {
    let args: CuidArgs = env::args().into();

    let id = match args.v2 {
        true => {
            if args.slug {
                // construct a v2 cuid with the same length as cuid1 slugs
                cuid2::CuidConstructor::new().with_length(10).create_id()
            } else {
                cuid2::create_id()
            }
        }
        false => {
            if args.slug {
                one_off_cuid1_slug()
            } else {
                one_off_cuid1()
            }
        }
    };

    println!("{}", id);
}

const HELP: &str = r#"Usage: cuid [OPTION]...
Generate and print a CUID.

Options:
  --v2           generate a v2 CUID/slug (this will eventually be the default)
  --slug         generate a slug instead of a full CUID
  -h, --help     display this help and exit
  -v, --version  display version information and exit"#;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Commandline arguments for the CUID binary
#[derive(Debug)]
struct CuidArgs {
    /// Whether to produce a slug instead of a CUID
    slug: bool,
    v2: bool,
}
impl From<Args> for CuidArgs {
    fn from(args: Args) -> Self {
        let mut slug = false;
        let mut v2 = false;

        // The first argument should be the binary name. Skip it.
        args.skip(1).for_each(|arg| match arg.as_str() {
            "-h" | "--help" => {
                println!("{}", HELP);
                exit(0);
            }
            "-v" | "--version" => {
                println!("{}", VERSION);
                exit(0);
            }
            "--slug" => slug = true,
            "--v2" => v2 = true,
            _ => {
                println!("error: unrecognized argument {}", arg);
                println!();
                println!("{}", HELP);
                exit(1);
            }
        });

        CuidArgs { slug, v2 }
    }
}
