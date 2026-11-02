use tiberius::Row;

#[cfg(feature = "derive")]
pub use tiberqueries_derive::FromRow;

pub mod mappings;
#[cfg(feature = "query")]
pub mod query;

pub trait FromRow {
    fn from_row(row: Row) -> Self;
}

#[cfg(feature = "derive")]
/// Helper method for tiberqueries_derive
pub fn string(str: Option<&str>) -> Option<String> {
    str.map(String::from)
}
