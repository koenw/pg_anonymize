# pg_anonymize

Anonymize PostgreSQL tables, e.g. for production-like data in development
environments.

`pg_anonymize` takes the output of the PostgreSQL `COPY` command and replaces
the values of columns according to your configuration. This makes it convenient
to anonymize data on-the-fly while importing from another database, as well as
making a anonymized copy of a table in the same database.

## Examples

```bash
psql -d sensitive_db -c 'COPY <table_name> TO STDOUT' |
  pg_anonymize -c config.yaml <table_name> |
  psql -d anonymized_db -c 'COPY <table_name> from STDIN'
```

where `config.yaml` looks as follows:

```yaml
tables:
  <table_name>:
    some_private_column:
      # `some_private_column` is the nth column (starting at 0). This order is
      # the same as the order in the *psql* output of `\d some_private_column`.
      order: 1
      # The kind of data to fill this column with.
      strategy: Name
```

## Building

`cargo build --release`

If you haven't got rust/cargo installed on your system yet, I suggest
[rustup](https://rustup.rs/).

## TODO

* more documentation
* tests
* Dockerfile/image
* Make `Strategy` a Trait
