use tiberius::{numeric::Numeric, time::chrono::{NaiveDate, NaiveDateTime, NaiveTime}, Uuid};
use crate::{string, FromRow};

// FromSql<'a> Trait
//
// |Rust type|Server type|
// |--------|--------|
// |`u8`|`tinyint`|
// |`i16`|`smallint`|
// |`i32`|`int`|
// |`i64`|`bigint`|
// |`f32`|`float(24)`|
// |`f64`|`float(53)`|
// |`bool`|`bit`|
// |`String`/`&str`|`nvarchar`/`varchar`/`nchar`/`char`/`ntext`/`text`|
// |`Vec<u8>`/`&[u8]`|`binary`/`varbinary`/`image`|
// |[`Uuid`]|`uniqueidentifier`|
// |[`Numeric`]|`numeric`/`decimal`|
// |[`Decimal`] (with feature flag `rust_decimal`)|`numeric`/`decimal`|
// |[`XmlData`]|`xml`|
// |[`NaiveDateTime`] (with feature flag `chrono`)|`datetime`/`datetime2`/`smalldatetime`|
// |[`NaiveDate`] (with feature flag `chrono`)|`date`|
// |[`NaiveTime`] (with feature flag `chrono`)|`time`|
// |[`DateTime`] (with feature flag `chrono`)|`datetimeoffset`|

impl FromRow for String {
    fn from_row(row: tiberius::Row) -> Self {
        string(row.get(0)).unwrap()
    }
}

impl FromRow for u8 {
    fn from_row(row: tiberius::Row) -> Self {
        row.get(0).unwrap()
    }
}

impl FromRow for i16 {
    fn from_row(row: tiberius::Row) -> Self {
        row.get(0).unwrap()
    }
}

impl FromRow for i32 {
    fn from_row(row: tiberius::Row) -> Self {
        row.get(0).unwrap()
    }
}

impl FromRow for i64 {
    fn from_row(row: tiberius::Row) -> Self {
        row.get(0).unwrap()
    }
}

impl FromRow for f32 {
    fn from_row(row: tiberius::Row) -> Self {
        row.get(0).unwrap()
    }
}

impl FromRow for f64 {
    fn from_row(row: tiberius::Row) -> Self {
        row.get(0).unwrap()
    }
}

impl FromRow for NaiveDateTime {
    fn from_row(row: tiberius::Row) -> Self {
        row.get(0).unwrap()
    }
}

impl FromRow for NaiveDate {
    fn from_row(row: tiberius::Row) -> Self {
        row.get(0).unwrap()
    }
}

impl FromRow for NaiveTime {
    fn from_row(row: tiberius::Row) -> Self {
        row.get(0).unwrap()
    }
}

impl FromRow for Uuid {
    fn from_row(row: tiberius::Row) -> Self {
        row.get(0).unwrap()
    }
}

impl FromRow for Numeric {
    fn from_row(row: tiberius::Row) -> Self {
        row.get(0).unwrap()
    }
}

impl FromRow for bool {
    fn from_row(row: tiberius::Row) -> Self {
        row.get(0).unwrap()
    }
}