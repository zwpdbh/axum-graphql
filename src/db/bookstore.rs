use crate::command_line::ExVersion;
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::types::Json;
use sqlx::Postgres;
use sqlx::Row;
use sqlx::{Decode, Encode};
use std::error::Error;

#[allow(unused)]
use tracing::{error, info, warn};

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub metadata: Option<Metadata>,
}

impl<'r> sqlx::FromRow<'r, PgRow> for Book {
    /// FromRow trait is specifically used for mapping query results (rows) from the database to Rust structs
    /// Default decode is not enought. We explicitly specify how to decode the metadata
    /// from `JSONB` format in Postgres into `Metadata` type
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Book {
            title: row.try_get("title")?,
            author: row.try_get("author")?,
            isbn: row.try_get("isbn")?,
            metadata: row
                .try_get::<Option<Json<Metadata>>, _>("metadata")?
                .map(|json| json.0),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Decode, Encode)]
pub struct Metadata {
    pub avg_review: f32,
    pub tags: Vec<String>,
}

impl sqlx::Type<Postgres> for Metadata {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        <Postgres as sqlx::Database>::TypeInfo::with_name("metadata")
    }
}

/// Example show how to create records
/// cargo run -- sqlx bookstore create
pub async fn create_book_example(pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "insert into book (title, author, isbn) values ($1, $2, $3)";
    sqlx::query(query)
        .bind("book01".to_string())
        .bind("fox".to_string())
        .bind("000-111-222-33".to_string())
        .execute(pool)
        .await?;

    let book = Book {
        title: "A Game of Thrones".to_string(),
        author: "Martin".to_string(),
        isbn: "111-222-333-444".to_string(),
        metadata: Some(Metadata {
            avg_review: 9.4,
            tags: vec!["fantasy".to_string(), "epic".to_string()],
        }),
    };
    let q = r#"
        insert into book (title, author, isbn, metadata) values ($1, $2, $3, $4)
        "#;
    sqlx::query(q)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .bind(serde_json::to_value(&book.metadata).unwrap())
        .execute(pool)
        .await?;

    Ok(())
}

/// Example show how to update records
/// cargo run -- sqlx bookstore update
pub async fn update_book_example(pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "update book set title = $1, author = $2, metadata = $3 where isbn = $4";
    sqlx::query(query)
        .bind("book01_changed".to_string())
        .bind("fox new name".to_string())
        .bind(
            serde_json::to_value(Some(Metadata {
                avg_review: 7.0,
                tags: vec!["art".to_string()],
            }))
            .unwrap(),
        )
        .bind("000-111-222-33".to_string())
        .execute(pool)
        .await?;

    let query = "update book set author = $1 where isbn = $2";
    sqlx::query(query)
        .bind("Margin games".to_string())
        .bind("111-222-333-444".to_string())
        .execute(pool)
        .await?;

    Ok(())
}

/// Shows how to read records from db in different ways.
pub async fn read_book_example(
    pool: &sqlx::PgPool,
    v: ExVersion,
) -> Result<Vec<Book>, Box<dyn Error>> {
    // let _ = sqlx::migrate!("migrations/bookstore").run(&pool).await?;

    let books = match v {
        ExVersion::V1 => fetch_books_v1(pool).await?,
        ExVersion::V2 => fetch_books_v2(pool).await?,
        ExVersion::V3 => fetch_books_v3(pool).await?,
        _ => todo!("not implemented"),
    };

    info!("books ==> {:?}", books);

    Ok(books)
}

/// Example shows how to manually extract each column from the fetched rows
/// cargo run -- sqlx bookstore read -v v1
async fn fetch_books_v1(pool: &sqlx::PgPool) -> Result<Vec<Book>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
    SELECT title, author, isbn, metadata FROM book
    "#,
    )
    .fetch_all(pool)
    .await?;

    let books = rows
        .into_iter()
        .map(|row| {
            let metadata: Option<Json<Metadata>> = row.try_get("metadata").ok();
            Book {
                title: row.get("title"),
                author: row.get("author"),
                isbn: row.get("isbn"),
                metadata: metadata.map(|json| json.0),
            }
        })
        .collect();

    Ok(books)
}

/// Example: show record row is automatically decoded into Rust object
/// cargo run -- sqlx bookstore read -v v2
async fn fetch_books_v2(pool: &sqlx::PgPool) -> Result<Vec<Book>, sqlx::Error> {
    let books = sqlx::query_as::<_, Book>(
        r#"
        SELECT title, author, isbn, metadata FROM book
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(books)
}

/// Example: show record row is automatically decoded into Rust object
/// Different from v2: v3 is improved on using `futures::stream::StreamExt` which is good for big data.
/// cargo run -- sqlx bookstore read -v v3
async fn fetch_books_v3(pool: &sqlx::PgPool) -> Result<Vec<Book>, sqlx::Error> {
    let mut books: Vec<Book> = vec![];
    let mut book_stream = sqlx::query_as::<_, Book>(
        r#"
        SELECT * FROM book
    "#,
    )
    .fetch(pool);

    while let Some(book) = book_stream.next().await {
        match book {
            Ok(book) => {
                books.push(book);
            }
            Err(e) => error!("Error fetching book: {}", e),
        }
    }

    Ok(books)
}
