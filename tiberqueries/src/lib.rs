//! A lightweight ORM for [Tiberius](https://github.com/prisma/tiberius).
//!
//! # Choosing a driver backend
//!
//! `tiberqueries` does not hard-require a single TDS driver. Exactly one of the
//! following features selects the crate it builds against:
//!
//! - `upstream` (default) — [`tiberius`](https://crates.io/crates/tiberius)
//! - `ng` — [`tiberius-ng`](https://crates.io/crates/tiberius-ng)
//!
//! ```toml
//! # default: upstream tiberius
//! tiberqueries = "0.2"
//!
//! # opt in to tiberius-ng instead
//! tiberqueries = { version = "0.2", default-features = false, features = ["derive", "query", "ng"] }
//! ```
//!
//! Whichever backend is selected is re-exported as [`tiberqueries::tiberius`](crate::tiberius).
//! Because driver types such as `Row` appear in this crate's public API, downstream
//! code must refer to the driver through that re-export rather than depending on a
//! TDS crate directly — otherwise the two copies are distinct types and will not
//! unify:
//!
//! ```ignore
//! use tiberqueries::tiberius::{Client, Query};
//! ```

#[cfg(all(feature = "upstream", feature = "ng"))]
compile_error!(
    "tiberqueries: features `upstream` and `ng` are mutually exclusive; enable exactly one. \
     Note that Cargo features are additive, so this can also mean two crates in your \
     dependency graph selected different backends."
);

#[cfg(not(any(feature = "upstream", feature = "ng")))]
compile_error!(
    "tiberqueries: no driver backend selected; enable exactly one of `upstream` or `ng`."
);

/// The TDS driver `tiberqueries` was built against.
///
/// Use this re-export instead of depending on a TDS crate directly, so that your
/// driver types are the same types this crate's API is built on.
#[cfg(feature = "upstream")]
pub use tiberius_upstream as tiberius;

/// The TDS driver `tiberqueries` was built against.
///
/// Use this re-export instead of depending on a TDS crate directly, so that your
/// driver types are the same types this crate's API is built on.
#[cfg(all(feature = "ng", not(feature = "upstream")))]
pub use tiberius_ng as tiberius;

#[cfg(any(feature = "upstream", feature = "ng"))]
use crate::tiberius::Row;

#[cfg(feature = "derive")]
pub use tiberqueries_derive::FromRow;

#[cfg(all(feature = "query", any(feature = "upstream", feature = "ng")))]
pub mod query;

#[cfg(any(feature = "upstream", feature = "ng"))]
pub mod mappings;

#[cfg(any(feature = "upstream", feature = "ng"))]
pub trait FromRow {
    fn from_row(row: Row) -> Self;
}

#[cfg(feature = "derive")]
/// Helper method for tiberqueries_derive
pub fn string(str: Option<&str>) -> Option<String> {
    if let Some(str) = str {
        Some(String::from(str))
    } else {
        None
    }
}
