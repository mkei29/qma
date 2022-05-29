
mod aggregate;
mod config;
mod log_record;
mod operation;
mod visualize;

use std::str;
use std::io;
use std::fs::File;
use std::io::BufRead;
use std::io::{ BufReader };
use std::collections::{HashMap};
use std::error::Error;

pub use crate::aggregate::{ TableRow, TableDef, Index, Field };
pub use crate::log_record::{ LogValueType, LogValue, LogRecord, Accessor };
pub use crate::config::qma_config:: { Config };
pub use crate::operation::{OpType};
pub use crate::visualize::{VisualizeType};


pub fn run(config_path: &str, filename: Option<&str>) {
    // initialize config
    let config = load_config(config_path).expect("Failed to load config file");
    let output_format = match &config.output_format {
        Some(f) => {
            // let f = str::to_lowercase(f);
            match f.as_str() {
                "csv"=> VisualizeType::Csv,
                "markdown"=> VisualizeType::Markdown,
                _ => VisualizeType::Markdown
            }
        },
        None => VisualizeType::Markdown
    };

    // initialize reader.
    // https://www.reddit.com/r/rust/comments/jv3q3e/how_to_select_between_reading_from_a_file_and/
    // https://github.com/reismannnr2/logrep/blob/master/src/main.rs
    let mut reader :Box<dyn BufRead> = match filename {
        Some(f) => {
            let file = File::open(f).unwrap();
            Box::new(BufReader::new(file))
        },
        None => {
            let stdin = std::io::stdin();
            let stdin = Box::leak(Box::new(stdin));
            Box::new(stdin.lock())
        }
    };

    // とりあえず固定のパラメータ
    let result = build_table_def(&config);
    if let Ok(def) = result {
        let mut table: HashMap<String, TableRow> = HashMap::new();

        // 一行ずつ読み込んで集計していく
        loop {
            let record = LogRecord::parse(&mut reader, def.key_accessor(), &def.field_accessor()[..]);

            if let Ok(r) = record {
                let key = r.key.to_string();

                if let Some(row) = table.get_mut(&key) {
                    row.update(&r, &def.fields)
                } else {
                    table.insert(r.key.to_string(), TableRow::new());
                }
            } else {
                break;
            }
        }
        match output_format {
            VisualizeType::Csv => {
                visualize::display_as_csv(&def, &table);
            },
            VisualizeType::Markdown => {
                visualize::display_as_markdown(&def, &table);
            }
        };
    };
}

fn build_table_def(config: &Config) -> Result<TableDef, io::Error> {
    // build index.
    let index = Index::new(Accessor::from_string(
        &config.index.name, &config.index.accessor, LogValueType::String));

    // build fields
    let mut fields :Vec<Field> = vec![];
    for qma_field in config.fields.iter() {
        let dtype = match qma_field.dtype.as_str() {
            "string" => LogValueType::String,
            "integer" => LogValueType::Integer,
            "float" => LogValueType::Float,
            "second" => LogValueType::Second,
            _ => LogValueType::None
        };
        let accessor = Accessor::from_string(&qma_field.name, &qma_field.accessor, dtype);

        let op_type = match qma_field.operation.as_str() {
            "average" => OpType::Average,
            "count" => OpType::Count,
            _ => OpType::Average
        };
        fields.push(Field::new(accessor, op_type));
    }
    let table_def = TableDef::new(index, fields);
    Ok(table_def)
}

fn load_config(filepath: &str) -> Result<Config, Box<dyn Error>>{
    let contents = std::fs::read_to_string(filepath)?;
    Ok(Config::parse(&contents))
}