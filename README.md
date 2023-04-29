## A Todo App implemented in Rust using Diesel and Clap

### Instructions to develop:

Install postgres and start service:
```sh
brew install postgresql
brew services start postgresql
brew install libpq
export RUSTFLAGS="-L/opt/homebrew/opt/libpq/lib"
```

```sh
cargo install diesel_cli --no-default-features --features postgres
```

Initialize diesel and run migrations
```sh
diesel setup
diesel migration generate todo
diesel migration generate users
diesel migration run
```

### How to run:
```sh
cargo run todo <create|delete|update|show> <other args>
cargo run user <create|delete|update|show> <other args>
```
