
mod aggregate;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap};
use serde_json::{Result, Value, };


pub use crate::aggregate::{ LogRecord, TableRow, Field };


fn show(r :&LogRecord) {
    let mut s = String::new();
    for (key, value) in &r.values {
        let key = key.as_str();
        let v = match value {
            None => "None",
            Some(v) => v.as_str()
        };
        s += format!("{} : {}, ", key ,v).as_str();
    }
    println!("{}", s);
}

enum ValueType {
    Integer,
    Float,
    Second,
    MilleSecond,
}

fn main() {

    let args: Vec<_> = env::args().collect();

    let filename = &args[1];

    let fields :Vec<Field> = vec![
        Field::new("latency", &["httpRequest".to_owned(), "latency".to_owned()]),
        Field::new("method", &["httpRequest".to_owned(), "requestMethod".to_owned()]),
    ];

    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    let mut table: HashMap<String, TableRow> = HashMap::new();

    // 一行ずつ読み込んで集計していく
    loop {
        let record = parse_line(&mut reader, &fields);
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


fn parse_line(reader: &mut BufReader<File>, fields :&[Field]) -> Result<LogRecord> {
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("error");

    let v: Value = serde_json::from_str(&buf)?;
    let mut record: LogRecord = LogRecord::new("this is test");

    for f in fields {
        let value = get_value(&v, &f.accessor, 0);
        record.set(f.name.as_str(), value);
    }
    Ok(record)
}

fn get_value(v :&Value, accessor: &[String], pos: usize) -> Option<String>{
    if accessor.len() == pos {
        return v.as_str().map(String::from);
    }
    let key = &accessor[pos];
    let nxt = &v[key];
    get_value(nxt, accessor, pos+1)
}

