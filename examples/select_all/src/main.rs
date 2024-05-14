use tiberqueries::FromRow;
use tiberqueries_derive::FromRow;
use std::error::Error;

#[derive(FromRow, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

fn main() {
    // let point = Point::from_row();

    // dbg!(point);
}
