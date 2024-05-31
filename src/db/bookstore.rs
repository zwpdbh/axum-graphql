use crate::db::DB_FOR_DEV;
use std::error::Error;

pub struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

impl Book {
    pub async fn create(self, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let query = "insert into book (title, author, isbn) values ($1, $2, $3)";
        sqlx::query(query)
            .bind(self.title)
            .bind(self.author)
            .bind(self.isbn)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn update(self, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let query = "update book set title = $1, author = $2 where isbn = $3";
        sqlx::query(query)
            .bind(self.title)
            .bind(self.author)
            .bind(self.isbn)
            .execute(pool)
            .await?;

        Ok(())
    }
}

pub async fn create_book_example() -> Result<(), Box<dyn Error>> {
    let pool = sqlx::postgres::PgPool::connect(DB_FOR_DEV).await?;
    sqlx::migrate!("migrations/bookstore").run(&pool).await?;

    let book = Book {
        title: "book01".to_string(),
        author: "fox".to_string(),
        isbn: "000-111-222-33".to_string(),
    };

    let _ = book.create(&pool).await?;

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

    let _ = book.update(&pool).await?;

    Ok(())
}
