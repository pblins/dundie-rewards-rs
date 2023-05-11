# dundie-rewards-rs

Rust implementation of [dundie-rewards](https://github.com/rochacbruno/dundie-rewards).

## Apply database migrations [Sqlite]

```sh
export DATABASE_URL=<path-to-database-file>
diesel migration run
```

## Compilation [debug mode]

```sh
cargo build
```

## Usage

```sh
./target/debug/dundie-rewards-rs --help
```