# How to Build a Powerful GraphQL API with Rust

## Show installed rust versions

```sh
$ rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/zw/.rustup

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu
nightly-2024-01-18-x86_64-unknown-linux-gnu
nightly-2024-01-19-x86_64-unknown-linux-gnu
nightly-2024-01-29-x86_64-unknown-linux-gnu
nightly-x86_64-unknown-linux-gnu (default)
1.72.0-x86_64-unknown-linux-gnu

active toolchain
----------------

1.72.0-x86_64-unknown-linux-gnu (overridden by '/home/zw/code/rust_programming/axum-graphql/rust-toolchain.toml')
rustc 1.72.0 (5680fa18f 2023-08-23)
```

## How to do query verification using sqlx

- Make sure `sqlx-cli` is installed, otherwise run: `cargo install sqlx-cli`.
- Set up `DATABASE_URL` like: `export DATABASE_URL="postgres://postgres:postgres@localhost:5432/myapp"`
- Run `cargo sqlx prepare`

## References

- [How to Build a Powerful GraphQL API with Rust](https://oliverjumpertz.com/blog/how-to-build-a-powerful-graphql-api-with-rust/)
- [SQLx is my favorite PostgreSQL driver to use with Rust.](https://www.youtube.com/watch?v=TCERYbgvbq0)


## How to database migration 

- `cargo install sea-orm-cli`
- `sea-orm-cli migrate init`