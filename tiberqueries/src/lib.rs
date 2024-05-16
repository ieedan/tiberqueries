use tiberius::Row;

#[cfg(feature = "derive")]
pub use tiberqueries_derive::FromRow;

#[cfg(feature = "query")]
pub mod query;
pub mod mappings;

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