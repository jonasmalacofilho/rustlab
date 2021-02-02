use sqlx::{
    prelude::*,
    sqlite::SqliteConnection,
};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut conn = SqliteConnection::connect("sqlite::memory:").await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&mut conn).await?;

    assert_eq!(row.0, 150);

    Ok(())
}
