
use std::fs::File;
use std::fmt;
use std::io::{BufRead, BufReader};
use std::collections::{ HashMap };
use serde_json::{Result, Value, };

#[derive(Clone)]
pub struct Accessor {
    /// struct to describe json query.
    pub name: String,
    pub accessor: Vec<String>,
    pub dtype: String
}

impl Accessor {
    pub fn from_string(name: &str, accessor: &str) -> Self{
        let mut new_vec: Vec<String> = Vec::new();
        for s in accessor.split('.') {
            new_vec.push(s.to_string());
        }
        Self {
            name: name.to_string(),
            accessor: new_vec,
            dtype: String::from("string")
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

    pub fn parse(reader :&mut BufReader<File>, index: &Accessor, fields :&[&Accessor]) -> Result<LogRecord> {
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
    Float(f64),
    Second(f64),
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

impl LogValue {
    pub fn as_string(&self) -> String {
        match self {
            LogValue::String(x) => {
                x.clone()
            },
            LogValue::Integer(x) => {
                x.to_string()
            },
            LogValue::Float(x) => {
                x.to_string()
            },
            LogValue::Second(x) => {
                x.to_string()
            },
            LogValue::None => {
                String::from("None")
            }
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
            if let Ok(num) = s.parse::<u32>() {
                LogValue::Integer(num)
            } else {
                LogValue::None
            }
        },
        "float" => {
            if let Ok(num) = s.parse::<f64>() {
                LogValue::Float(num)
            } else {
                LogValue::None
            }            
        },
        "second" => {
            let replace_and_float = |inp :&str, pattern: &str| {
                let raw = inp.replace(pattern, "");
                if let Ok(num) = raw.parse::<f64>() {
                    LogValue::Float(num)
                } else {
                    LogValue::None
                }     
            };
            if s.ends_with('s') {
                replace_and_float(s, "s")
            } else if s.ends_with("sec") {
                replace_and_float(s, "sec")
            } else {
                LogValue::None
            }
        },
        _ => { LogValue::None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_parse_value() {
        // Check string case.
        let v = parse_value("string", "test_string");
        if let LogValue::String(s) = v {
            assert_eq!(s, "test_string");
        } else {
            unreachable!();
        }

        // Check integer case.
        let v = parse_value("integer", "123");
        if let LogValue::Integer(n) = v {
            assert_eq!(n, 123);
        } else {
            unreachable!();
        }
        let v = parse_value("integer", "abc");
        assert!(matches!(v, LogValue::None));

        // Check float case.
        let v = parse_value("float", "123.4");
        if let LogValue::Float(n) = v {
            assert_eq!(n, 123.4);
        } else {
            unreachable!();
        }
        let v = parse_value("integer", "abc");
        assert!(matches!(v, LogValue::None));

        // Check second case.
        let v = parse_value("second", "123.4s");
        if let LogValue::Float(n) = v {
            assert_eq!(n, 123.4);
        } else {
            unreachable!();
        }
        let v = parse_value("second", "123.4h");
        assert!(matches!(v, LogValue::None));
    }
}
