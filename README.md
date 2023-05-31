# dundie-rewards-rs

Rust implementation of [dundie-rewards](https://github.com/rochacbruno/dundie-rewards).

## Apply database migrations [SQLite]

```
> export DATABASE_URL=<path-to-database-file>
> diesel migration run
```

## Compilation [debug mode]

```
> cargo build
```

## Usage

```
> ./target/debug/dundie-rewards-rs --help

Dunder Mifflin Rewards System.
 This cli application controls Dunder Mifflin rewards.
  - admins can load information tot he people database and assign points.
  - users can view reports and transfer points.

Usage: dundie-rewards-rs <COMMAND>

Commands:
  load       Loads the file to the database.
  show       Shows information about user or dept.
  add        Add points to the user or dept.
  remove     Remove points to the user or dept.
  transfer   Transfer points to a specific user.
  movements  Lists movements.
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

To run the initial `load` with the csv file (assets/people.csv), use the admin user (username: `admin`, pwd: `admin`).