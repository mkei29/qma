
mod aggregate;

use std::env;
use std::fs::File;
use std::io::{ BufReader };
use std::collections::{HashMap};

pub use crate::aggregate::{ LogRecord, TableRow, Index, Field };


fn main() {

    let args: Vec<_> = env::args().collect();

    // とりあえず固定のパラメータ
    let filename = &args[1];
    let index = Index::new("index", &["httpRequest".to_owned(), "requestMethod".to_owned()]);
    let fields :Vec<Field> = vec![
        Field::new("latency", &["httpRequest".to_owned(), "latency".to_owned()]),
        Field::new("method", &["httpRequest".to_owned(), "requestMethod".to_owned()]),
    ];

    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    let mut table: HashMap<String, TableRow> = HashMap::new();

    // 一行ずつ読み込んで集計していく
    loop {
        let record = LogRecord::parse(&mut reader, &index, &fields);

        if let Ok(r) = record {
            let key = r.key.to_string();

            if let Some(row) = table.get_mut(&key) {
                 row.update(&r, &fields)
             } else {
                table.insert(r.key.to_string(), TableRow::new());
             }
        } else {
            break;
        }
    }

    // CSV形式で出力する
    let mut header = String::from("");
    for  (i, f) in fields.iter().enumerate() {
        header += &f.name;
        if i != fields.len() {
            header += ",";
        }
    }
    println!("{}", &header);

    for (key, row) in table.iter() {
        let mut row_str = String::from(key);
        row_str += ",";

        for (i, v) in row.get_row(&fields).iter().enumerate() {
            if let Some(x) = v {
                row_str += &x.to_string();
            } else {
                row_str += "";
            }
            if i != fields.len() {
                row_str += ",";
            }

        }
        println!("{}", &row_str);
    }
}

