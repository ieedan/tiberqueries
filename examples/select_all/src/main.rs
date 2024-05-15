use tiberqueries::FromRow;
use tiberqueries_derive::FromRow;

#[derive(FromRow)]
#[to_pascal]
pub struct Point {
    pub x: i32,
    pub y: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub super_description: f64
}

fn main() {
    // let point = Point::from_row();

    // dbg!(point);
}
