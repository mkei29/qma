
use std::fs::File;
use std::fmt;
use std::io::{BufRead, BufReader};
use std::collections::{ HashMap };
use serde_json::{Result, Value, };

#[derive(Clone)]
pub struct OpCount {
    count: u32
}

impl OpCount {
    pub fn new() -> Self{
        Self {
            count: 0
        }
    }

    pub fn update(&mut self, v :&LogValue) {
        if let LogValue::String(s) = v {
            self.count += 1;
        }
    }

    pub fn value(&self) -> u32{
        self.count
    }
}


pub struct TableRow {
    // row name -> value
    values: HashMap<String, OpCount>,
}

impl TableRow {
    pub fn new() -> Self{
        Self {
            values: HashMap::new(),
        }
    }

    pub fn update(&mut self, record: &LogRecord, fields: &[Field]) {
        for f in fields {
            // Insert field if not exist.
            self.values.entry(f.name.clone())
                .or_insert_with(OpCount::new);

            let v = record.get(&f.name);
            if let Some(op) = self.values.get_mut(&f.name) {
                op.update(&v);
            }
        }
    }

    ///
    /// 
    pub fn get_row(&self, fields: &[Field]) -> Vec<Option<u32>> {
        let mut result: Vec<Option<u32>> = Vec::new();
        for f in fields {
            let v = self.values.get(&f.name).map(|x| x.value());
            result.push(v);
        }
        result
    }
}


#[derive(Clone)]
pub struct Field {
    pub name: String,
    pub accessor: Vec<String>,
}

impl Field {
    pub fn new(name: &str, accessor: &[String]) -> Self {
        Self {
            name: name.to_owned(),
            accessor: accessor.to_vec(),
            // 型のタイプ
        }
    }    
}

pub struct Index {
    pub name: String,
    pub accessor: Vec<String>
}

impl Index {
    pub fn new(name: &str, accessor: &[String]) -> Self {
        Self {
            name: name.to_owned(),
            accessor: accessor.to_vec(),
        }
    }    
}



pub struct LogRecord {
    pub key: String,
    pub values: HashMap<String, LogValue>
}

impl LogRecord {
    pub fn new (key: &str) -> Self {
        Self {
            key: String::from(key),
            values: HashMap::new()
        }
    }

    pub fn parse(reader :&mut BufReader<File>, index: &Index, fields :&[Field]) -> Result<LogRecord> {
        // read line
        let mut buf = String::new();
        reader.read_line(&mut buf).expect("error");
        let v: Value = serde_json::from_str(&buf)?;

        // Read key and init log record.
        let key = match get_value(&v, &index.accessor, 0) {
            Some(x) => x,
            None => "undefined".to_owned()
        };
        let mut record = LogRecord {
            key, values: HashMap::new()
        };

        // Read data
        for f in fields {
            let value = get_value(&v, &f.accessor, 0);
            if let Some(v) = value {
                record.set(f.name.as_str(), v);
            }
        }
        Ok(record)
    }

    pub fn set(&mut self, key: &str, value: String) {
        let v = parse_value("string", &value);
        self.values.insert(key.to_string(), v);
    }

    pub fn get(&self, key :&str) -> LogValue {
        if let Some(x) = self.values.get(key) {
            println!("{}", x);
            x.clone()
        } else {
            LogValue::None
        }
    }

}

#[derive(Clone, Debug)]
pub enum LogValue {
    String(String),
    Integer(u32),
    Float(i32),
    None,
}

impl fmt::Display for LogValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogValue::String(s) => write!(f, "String({})", s ),
            _ => write!(f, "")
        }
    }
}

fn get_value(v :&Value, accessor: &[String], pos: usize) -> Option<String>{
    if accessor.len() == pos {
        return v.as_str().map(String::from);
    }
    let key = &accessor[pos];
    let nxt = &v[key];
    get_value(nxt, accessor, pos+1)
}

fn parse_value(typ: &str, s :&str) -> LogValue {
    match typ {
        "string" => {
            LogValue::String(s.to_string())
        },
        "integer" => {
            LogValue::None
        },
        _ => { LogValue::None }
    }
}



