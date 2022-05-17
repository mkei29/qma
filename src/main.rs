
mod aggregate;
mod config;
mod log_record;
mod operation;

use std::env;
use std::fs::File;
use std::io::{ BufReader };
use std::collections::{HashMap};

pub use crate::aggregate::{ TableRow, TableDef, Index, Field };
pub use crate::log_record::{ LogRecord, Accessor };
pub use crate::config:: { Config };
pub use crate::operation::{OpType};


fn main() {

    let args: Vec<_> = env::args().collect();

    // とりあえず固定のパラメータ
    let filename = &args[1];
    let index = Index::new(Accessor::from_string("key", "httpRequest.requestMethod"));
    let fields :Vec<Field> = vec![
        Field::new(Accessor::from_string("latency", "httpRequest.latency"), OpType::Average),
        Field::new(Accessor::from_string("method", "httpRequest.requestMethod"), OpType::Count)
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

    // CSV形式で出力する
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