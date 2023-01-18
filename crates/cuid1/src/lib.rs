//! # cuid-rust
//!
//! CUID generation in rust
//!
//! **NOTE:** The first version of the CUID specification is deprecated. Please
//! use the `cuid2()` function from this crate instead, which is a re-export
//! from the `cuid2` crate. Please upgrade to that crate for more CUID
//! construction options.
//!
//! ```rust
//! use cuid;
//!
//! // Get a CUID
//! println!("{}", cuid::cuid2());
//! ```
//!
//! Note that this crate also provides a very simple, single purpose
//! commandline interface:
//!
//! ```sh
//! $> cuid
//! ckfritrvg0000kdtwc766fful
//! ```
//!
//! You can generate v2 CUIDs via the commandline like so:
//!
//! ```sh
//! $> cuid --v2
//! ```
//!
//! See the [original implementation] for more details on CUIDs in general.
//!
//! [original implementation]: https://github.com/ericelliott/cuid
//!

mod cuid1;
mod error;
mod text;
mod time;

#[allow(deprecated)]
pub use cuid1::{cuid, is_cuid, is_slug, slug};
pub use cuid2::create_id as cuid2;
pub use error::CuidError;

const BASE: u8 = 36;
