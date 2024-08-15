//! # cuid-rust
//!
//! CUID generation in rust
//!
//! ```rust
//! use cuid;
//!
//! // Get a v1 CUID
//! println!("{}", cuid::cuid1());
//!
//! // Get a v2 CUID
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
//! See the [original v1 implementation] and [original v2 implementation] for
//! more details on CUIDs in general.
//!
//! [original v1 implementation]: https://github.com/ericelliott/cuid
//! [original v2 implementation]: https://github.com/paralleldrive/cuid2
//!

mod cuid1;
mod error;
mod text;
mod time;

#[allow(deprecated)]
pub use cuid1::{cuid, cuid1, cuid1_slug, is_cuid, is_cuid1, is_cuid1_slug, is_slug, slug};
#[doc(hidden)]
pub use cuid1::{one_off_cuid1, one_off_cuid1_slug};

pub use cuid2::{
    cuid as cuid2, is_cuid2, is_slug as is_cuid2_slug, slug as cuid2_slug,
    CuidConstructor as Cuid2Constructor,
};
pub use error::CuidError;

const BASE: u8 = 36;
