
mod aggregate;
mod config;
mod log_record;
mod operation;

use std::env;
use std::cmp;
use std::fs::File;
use std::io::{ BufReader };
use std::collections::{HashMap};

pub use crate::aggregate::{ TableRow, TableDef, Index, Field };
pub use crate::log_record::{ LogValueType, LogValue, LogRecord, Accessor };
pub use crate::config:: { Config };
pub use crate::operation::{OpType};


fn main() {

    let args: Vec<_> = env::args().collect();

    // とりあえず固定のパラメータ
    let filename = &args[1];
    let index = Index::new(Accessor::from_string("key", "httpRequest.requestMethod", LogValueType::String));
    let fields :Vec<Field> = vec![
        Field::new(Accessor::from_string("latency", "httpRequest.latency", LogValueType::Second), OpType::Average),
        Field::new(Accessor::from_string("method", "httpRequest.requestMethod", LogValueType::String), OpType::Count)
    ];

    let def = TableDef::new(index, fields);

    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

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

    display_as_markdown(&def, &table);
}

fn display_as_csv(def: &TableDef, table: &HashMap<String, TableRow>) {
    // # Display aggregated result as csv format.
    let mut header = String::from("");
    for  (i, &f) in def.field_accessor().iter().enumerate() {
        header += &f.name;
        if i != def.field_num() {
            header += ",";
        }
    }
    println!("{}", &header);

    for (key, row) in table.iter() {
        let mut row_str = String::from(key);
        row_str += ",";

        for (i, v) in row.get_row(&def.fields).iter().enumerate() {

            let s  = v.as_string();
            row_str += &s;
            if i != def.field_num() {
                row_str += ",";
            }

        }
        println!("{}", &row_str);
    }
}

fn display_as_markdown(def: &TableDef, table: &HashMap<String, TableRow>) {
    // # Display aggregated result as csv format.

    // count max chars of each fields;
    let mut col_width = vec![10; 1+def.fields.len()];
    col_width[0] = cmp::max(col_width[0], def.index.name().chars().count());
    for (i, &f) in def.field_accessor().iter().enumerate() {
        col_width[i+1] = cmp::max(col_width[i+1], f.name.chars().count());
    }
    
    // print header;
    let mut header = String::from("|");
    header += &format_string(def.index.name(), col_width[0]);
    header += "|";
    for (i, &f) in def.field_accessor().iter().enumerate() {
        let width = col_width[i+1];
        header += &format_string(&f.name, width);
        header += "|"
    }
    println!("{}", &header);

    // print separator
    let mut separator = String::new();
    for w in col_width.iter() {
        separator += &format!("|{}", "-".repeat(*w));
    }
    separator += "|";
    println!("{}", separator);

    // print table
    for (key, row) in table.iter() {
        let width = col_width[0];
        let mut row_str = String::from("|");
        row_str += &format_string(key, width);
        row_str += "|";

        for (i, v) in row.get_row(&def.fields).iter().enumerate() {
            let width = col_width[i+1];
            row_str += &format_log_value(v, width);
            if i != def.field_num() {
                row_str += "|";
            }

        }
        println!("{}", &row_str);
    }
}

fn format_log_value(v: &LogValue, width: usize) -> String{
    let s  = v.as_string();
    format!("{:>width$}", &s, width = width)
}

fn format_string(s: &str, width: usize) -> String{
    format!("{:width$}", s, width = width)
}