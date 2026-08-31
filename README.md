# tiberqueries
A lightweight ORM for [Tiberius](https://github.com/prisma/tiberius). 

Includes derive functionality as well as a trait implementation for the Client. The goal is not to be a query builder and rather to just make mapping your types simple. 

If you are looking for a query builder for SQL Server try [CANYON-SQL](https://github.com/zerodaycode/Canyon-SQL).

## Driver backend

`tiberqueries` builds against a TDS driver of your choosing. Exactly one backend
feature must be enabled:

| Feature | Crate | Notes |
|---|---|---|
| `upstream` | [`tiberius`](https://crates.io/crates/tiberius) | Default. |
| `ng` | [`tiberius-ng`](https://crates.io/crates/tiberius-ng) | Community fork; opt-in. |

```toml
# default -- upstream tiberius
tiberqueries = "0.2"

# opt in to tiberius-ng instead
tiberqueries = { version = "0.2", default-features = false, features = ["derive", "query", "ng"] }
```

Whichever backend you select is re-exported as `tiberqueries::tiberius`. Driver
types such as `Row` and `Query` appear in this crate's public API, so **refer to
the driver through that re-export** rather than depending on a TDS crate
directly:

```rust
use tiberqueries::tiberius::{Client, Query};
```

If you depend on a TDS crate directly *and* it is not the one `tiberqueries` was
built against, the two copies are distinct types and will not unify -- you will
see errors like `` expected `tiberius::row::Row`, found `tiberius::Row` ``. Going
through `tiberqueries::tiberius` keeps a single source of truth.

Note that Cargo features are additive: if two crates in one dependency graph
select different backends, both features turn on and the build fails with a
`compile_error!` naming the conflict.

## Usage
Add `FromRow` to the derive attribute above your struct to automatically generate a `from_row` method on the struct. You can manually map the rows from here or use the `query` feature to extend the client with methods that automatically map the result.

Here we use `FromRow` on an User struct and then select all from the Users table using the `qry` client extension.

```rust
#[derive(FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub picture: String,
    pub phone_number: Option<String>,
    pub role_id: i32,
    pub admin: bool,
    pub joined: NaiveDateTime
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    /* -- Initialize Client  -- */

    let mut qry = Query::new("SELECT * FROM Users");

    let users: Vec<User> = client.qry(qry).await?;
    
    Ok(())
}
```

### Naming

Often your names or casing conventions will be different in your database to your application. When this is the case **tiberqueries** exports a few attributes to make your life easier.

#### to_pascal
This converts all members of the struct to **PascalCase**.

```rust
#[derive(FromRow, Debug)]
#[to_pascal]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub picture: String,
    pub phone_number: Option<String>,
    pub role_id: i32,
    pub admin: bool,
    pub joined: NaiveDateTime
}

/* Generated implementation

impl ::tiberqueries::FromRow for User {
    fn from_row(row: ::tiberqueries::tiberius::Row) -> Self {
        use ::tiberqueries::string;
        Self {
            id: row.get("Id").unwrap(),
            username: string(row.get("Username")).unwrap(),
            email: string(row.get("Email")).unwrap(),
            picture: string(row.get("Picture")).unwrap(),
            phone_number: string(row.get("PhoneNumber")),
            role_id: row.get("RoleId").unwrap(),
            admin: row.get("Admin").unwrap(),
            joined: row.get("Joined").unwrap(),
        }
    }
}

*/
```

#### sql_name
This allows you to specify the name of the field as it appears in SQL. This will override [to_pascal](#to_pascal).

```rust
#[derive(FromRow)]
pub struct Point {
    pub x: i32,
    #[sql_name="why"]
    pub y: i32,
}

/*

impl ::tiberqueries::FromRow for Point {
    fn from_row(row: ::tiberqueries::tiberius::Row) -> Self {
        use ::tiberqueries::string;
        Self {
            x: row.get("x").unwrap(),
            y: row.get("why").unwrap(),
        }
    }
}

*/
```

#### sql_ignore
Sometimes you need values to be included on your type that aren't mapped to SQL. In this case add `#[sql_ignore="true"]` above the property. You will also need to wrap the type of the property in an `Option<_>` type or you will get an error.

```rust
#[derive(FromRow)]
pub struct Point {
    pub x: i32,
    #[sql_ignore="true"]
    pub y: Option<i32>,
}

/*

impl ::tiberqueries::FromRow for Point {
    fn from_row(row: ::tiberqueries::tiberius::Row) -> Self {
        use ::tiberqueries::string;
        Self {
            x: row.get("x").unwrap(),
            y: None,
        }
    }
}

*/
```
