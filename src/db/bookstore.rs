use crate::{command_line::ExVersion, db::DB_FOR_DEV};
use futures::stream::StreamExt;
use sqlx::{prelude::FromRow, Row};
use std::error::Error;
#[allow(unused)]
use tracing::{error, info, warn};

#[derive(Debug, FromRow)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

pub async fn create_book_example() -> Result<(), Box<dyn Error>> {
    let pool = sqlx::postgres::PgPool::connect(DB_FOR_DEV).await?;
    sqlx::migrate!("migrations/bookstore").run(&pool).await?;

    let book = Book {
        title: "book01".to_string(),
        author: "fox".to_string(),
        isbn: "000-111-222-33".to_string(),
    };

    let query = "insert into book (title, author, isbn) values ($1, $2, $3)";
    sqlx::query(query)
        .bind(book.title)
        .bind(book.author)
        .bind(book.isbn)
        .execute(&pool)
        .await?;

    Ok(())
}

pub async fn update_book_example() -> Result<(), Box<dyn Error>> {
    let pool = sqlx::postgres::PgPool::connect(DB_FOR_DEV).await?;
    sqlx::migrate!("migrations/bookstore").run(&pool).await?;

    let book = Book {
        title: "book01_changed".to_string(),
        author: "fox new name".to_string(),
        isbn: "000-111-222-33".to_string(),
    };

    let query = "update book set title = $1, author = $2 where isbn = $3";
    sqlx::query(query)
        .bind(book.title)
        .bind(book.author)
        .bind(book.isbn)
        .execute(&pool)
        .await?;

    Ok(())
}

pub async fn read_book_example(v: ExVersion) -> Result<Vec<Book>, Box<dyn Error>> {
    let pool = sqlx::postgres::PgPool::connect(DB_FOR_DEV).await?;
    sqlx::migrate!("migrations/bookstore").run(&pool).await?;

    let books = match v {
        ExVersion::V1 => fetch_books_v1(&pool).await?,
        ExVersion::V2 => fetch_books_v2(&pool).await?,
        ExVersion::V3 => fetch_books_v3(&pool).await?,
        ExVersion::V4 => fetch_books_v4(&pool).await?,
    };

    info!("books ==> {:?}", books);

    Ok(books)
}

/// Example: cargo run -- sqlx bookstore read -v v1
async fn fetch_books_v1(pool: &sqlx::PgPool) -> Result<Vec<Book>, sqlx::Error> {
    let rows = sqlx::query("SELECT title, author, isbn FROM book")
        .fetch_all(pool)
        .await?;

    let books = rows
        .into_iter()
        .map(|row| Book {
            title: row.get("title"),
            author: row.get("author"),
            isbn: row.get("isbn"),
        })
        .collect();

    Ok(books)
}

/// Example: cargo run -- sqlx bookstore read -v v2
async fn fetch_books_v2(pool: &sqlx::PgPool) -> Result<Vec<Book>, sqlx::Error> {
    let books = sqlx::query_as!(Book, "SELECT title, author, isbn FROM book")
        .fetch_all(pool)
        .await?;

    Ok(books)
}

/// Example: cargo run -- sqlx bookstore read -v v3
/// Need install dependency: futures = "0.3"
/// And use futures::stream::StreamExt;
/// It is useful for large dataset
async fn fetch_books_v3(pool: &sqlx::PgPool) -> Result<Vec<Book>, sqlx::Error> {
    let mut books: Vec<Book> = vec![];
    let mut book_stream = sqlx::query_as!(Book, "SELECT title, author, isbn FROM book").fetch(pool);

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

/// Example: cargo run -- sqlx bookstore read -v v4
/// use sqlx::prelude::FromRow
/// FromRow trait is specifically used for mapping query results (rows) from the database to Rust structs
async fn fetch_books_v4(pool: &sqlx::PgPool) -> Result<Vec<Book>, sqlx::Error> {
    let mut books: Vec<Book> = vec![];
    let mut book_stream =
        sqlx::query_as::<_, Book>("SELECT title, author, isbn FROM book").fetch(pool);

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
