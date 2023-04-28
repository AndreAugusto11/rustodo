Instructions to run:

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

Initialize 
```sh
diesel setup
diesel migration generate todo
diesel migration generate users
diesel migration run
```
