use tiberius::Row;

pub trait FromRow {
    fn from_row(row: Row) -> Self;
}