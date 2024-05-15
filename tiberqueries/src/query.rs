use tiberius::{Client, ExecuteResult, Query};
use tokio::net::TcpStream;
use tokio_util::compat::Compat;

use crate::FromRow;

pub type SqlConnection = Client<Compat<TcpStream>>;

pub trait SqlCommands {
    async fn qry<'a, T: FromRow>(
        &mut self,
        qry: Query<'a>,
    ) -> Result<Vec<T>, tiberius::error::Error>;

    async fn qry_first_or_none<'a, T: FromRow>(
        &mut self,
        qry: Query<'a>,
    ) -> Result<Option<T>, tiberius::error::Error>;

    async fn exec<'a>(&mut self, qry: Query<'a>) -> Result<ExecuteResult, tiberius::error::Error>;
}

impl SqlCommands for SqlConnection {
    /// Returns the mapped result of the query as a Vector of the provided generic.
    /// 
    /// # Examples
    /// 
    /// ```ignore
    /// let mut qry = Query::new("SELECT * FROM Users");
    ///
    /// let users: Vec<User> = client.qry(qry).await?;
    /// ```
    async fn qry<'a, T: FromRow>(
        &mut self,
        qry: Query<'a>,
    ) -> Result<Vec<T>, tiberius::error::Error> {
        let res = qry.query(self).await?;

        let mut items: Vec<T> = vec![];

        for result in res.into_results().await? {
            for row in result {
                items.push(T::from_row(row));
            }
        }

        Ok(items)
    }

    /// Returns the first mapped result of the query or None.
    /// 
    /// # Examples
    /// 
    /// ```ignore
    /// let mut qry = Query::new("SELECT 1 Users.* FROM Users");
    ///
    /// let users: Option<User> = client.qry_first_or_none(qry).await?;
    /// ```
    async fn qry_first_or_none<'a, T: FromRow>(
        &mut self,
        qry: Query<'a>,
    ) -> Result<Option<T>, tiberius::error::Error> {
        let res = qry.query(self).await?;

        if let Some(row) = res.into_row().await? {
            Ok(Some(T::from_row(row)))
        } else {
            Ok(None)
        }
    }

    /// Executes the provided query and returns the result.
    /// 
    /// # Examples
    /// 
    /// ```ignore
    /// let mut qry = Query::new("DELETE FROM Users WHERE Id = @P1");
    /// 
    /// qry.bind(10);
    /// 
    /// client.exec(qry).await?;
    /// ```
    async fn exec<'a>(&mut self, qry: Query<'a>) -> Result<ExecuteResult, tiberius::error::Error> {
        Ok(qry.execute(self).await?)
    }
}
