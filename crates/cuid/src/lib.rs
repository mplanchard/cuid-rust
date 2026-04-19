//! # CUID generation in rust
//!
//! This crate contains implementations of both the [v1] and [v2] CUID
//! algorithms. By default, both v1 and v2 CUIDs are available. If you
//! are optimizing for binary size, you can exclude one or the other
//! by setting `default_features = false` and selecting the one you
//! need (see [Features](#Features), below).
//!
//! In addition, both CUID algorithms are provided as their own
//! independent crates, which this one merely wraps. They are creatively
//! named [cuid1](https://docs.rs/cuid/latest/cuid1/) and [cuid2](https://docs.rs/cuid/latest/cuid2/).
//!
//! ## Usage
//!
//! ```rust
//! use cuid;
//!
//! // Get a v1 CUID
//! println!("{}", cuid::cuid1());
//! println!("{}", cuid::v1::cuid());
//!
//! // Get a v2 CUID
//! println!("{}", cuid::cuid2());
//! println!("{}", cuid::v2::cuid());
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
//! v2 CUIDs also support some customization, allowing the specification
//! of:
//! - length
//! - a counter function
//! - a system fingerprint function
//!
//! See the [v2 CuidConstructor](https://docs.rs/cuid2/0.1.5/cuid2/struct.CuidConstructor.html)
//! docs for more details.
//!
//! ## Should I Use v1 or v2?
//!
//! According to the original implementation, v1 CUIDs are deprecated
//! because they are "insecure," due to the fact that they are k-sortable,
//! which is to say, as one goes forward in time, v1 CUIDs go up.
//!
//! K-sortability is actually a common property of IDs, and is sometimes
//! desired. For example, k-sortable IDs are great for PKs for high-volume
//! timeseries data, since they significantly improve index locality of
//! adjacent rows. As an example, v7 UUIDs are k-sortable, and you can find
//! plenty of material online discussing performance improvements using
//! them for primary keys in postgres or other databases.
//!
//! That said, k-sortable IDs are significantly more "guessable" than
//! non-k-sortable IDs, which can potentially be a security issue for
//! certain applications.
//!
//! For CUID, the v1 algorithm is simpler and faster than the v2
//! algorithm. Generating a v1 CUID is around 7-8x faster than a v2
//! CUID (~127 ns vs ~962 ns on my machine).
//!
//! As such, if your use-case is not sensitive to guessability, I
//! would recommend going with v1.
//!
//! For more information, see [this issue](https://github.com/mplanchard/cuid-rust/issues/17)
//!
//! ## Prior Art
//!
//! See the [original v1 implementation][v1] and [original v2
//! implementation][v2] for more details on CUIDs in general.
//!
//! [v1]: https://github.com/ericelliott/cuid
//! [v2]: https://github.com/paralleldrive/cuid2
//!
//! ## Features
//! - `v1` (enabled by default): provides access to v1 CUIDs
//! - `v2` (enabled by default): provides access to v2 CUIDs
//!

#[cfg(feature = "v1")]
pub use cuid1::{
    self as v1, cuid as cuid1, is_cuid as is_cuid1, is_slug as is_cuid1_slug, slug as cuid1_slug,
};
#[cfg(feature = "v1")]
#[doc(hidden)]
pub use cuid1::{one_off_cuid1, one_off_cuid1_slug};

#[cfg(feature = "v2")]
pub use cuid2::{
    self as v2, CuidConstructor as Cuid2Constructor, cuid as cuid2, is_cuid2,
    is_slug as is_cuid2_slug, slug as cuid2_slug,
};

#[cfg(test)]
mod test {
    use super::*;

    /// Run an already-defined test in WASM as well.
    macro_rules! wasm_test {
        ($name:ident) => {
            paste::paste! {
                #[wasm_bindgen_test::wasm_bindgen_test]
                fn [<wasm_ $name>]() {
                    $name()
                }
            }
        };
    }

    #[cfg(feature = "v1")]
    #[test]
    fn test_v1() {
        is_cuid1(cuid1());
        is_cuid1_slug(cuid1_slug());
    }
    wasm_test!(test_v1);

    #[cfg(feature = "v2")]
    #[test]
    fn test_v2() {
        is_cuid2(cuid2());
        is_cuid2_slug(cuid2_slug());
    }
    wasm_test!(test_v2);
}
