use serde::Deserialize;
use structopt::StructOpt;

use std::io::BufRead;
use std::collections::HashMap;

mod strategy;
use strategy::Strategy;

#[derive(Debug, StructOpt)]
#[structopt(name = "pg_anomimize", about = "Anonimize postgres COPY output")]
struct Opts {
#[structopt()]
    /// The table we're COPY-ing
    table: String,
}

#[derive(Deserialize)]
struct Config {
    tables: HashMap<TableName, TableConfig>,
}

type TableConfig = HashMap<ColumnName, ColumnConfig>;

#[derive(Deserialize)]
struct ColumnConfig {
    column: usize,
    strategy: Strategy,
}

type ColumnName = String;
type TableName = String;

fn process_row<'a>(config: &HashMap<TableName, TableConfig>, row: &'a str, table: &str) {
    // If we don't have a translation config for a table, we just print the row unchanged.
    let table_config = match config.get(table) {
        Some(config) => config,
        None => {
            println!("{}", row);
            return;
        }
    };

    let mut new_row: Vec<String> = row.split('\t').map(|s| s.to_string()).collect();
    for (_column_name, config) in table_config {
        if config.column >= new_row.len() {
            panic!("Column {} does not exist in {}", config.column, table);
        }
        let new_value = config.strategy.generate();
        new_row[config.column] = new_value;
    }
    println!("{}", new_row.join("\t"));
}

fn main() {
    let opts = Opts::from_args();
    let config_raw = std::fs::read_to_string("config.yaml").expect("Failed to read config.yaml");
    let config: Config = serde_yaml::from_str(&config_raw).expect("Failed to decode config.yaml");
    let stdin = std::io::stdin();

    for row in stdin.lock().lines() {
        let row = row.unwrap();
        process_row(&config.tables, &row, &opts.table);
    }
}
