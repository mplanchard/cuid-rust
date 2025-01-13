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
//! i12sf8k69lbvktlr7qb4p6xv
//! ```
//!
//! See the [original v1 implementation] and [original v2 implementation] for
//! more details on CUIDs in general.
//!
//! [original v1 implementation]: https://github.com/ericelliott/cuid
//! [original v2 implementation]: https://github.com/paralleldrive/cuid2
//!

#[cfg(feature = "v1")]
pub use cuid1::{
    cuid as cuid1, is_cuid as is_cuid1, is_slug as is_cuid1_slug, slug as cuid1_slug,
};
#[cfg(feature = "v1")]
#[doc(hidden)]
pub use cuid1::{one_off_cuid1, one_off_cuid1_slug};

#[cfg(feature = "v2")]
pub use cuid2::{
    cuid as cuid2, is_cuid2, is_slug as is_cuid2_slug, slug as cuid2_slug,
    CuidConstructor as Cuid2Constructor,
};
