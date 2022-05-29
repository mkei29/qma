
use std::fmt;
use std::io::{BufRead };
use std::collections::{ HashMap };
use serde_json::{Result, Value, };

#[derive(Clone)]
pub struct Accessor {
    /// struct to describe json query.
    pub name: String,
    pub accessor: Vec<String>,
    pub dtype: LogValueType
}

impl Accessor {
    pub fn from_string(name: &str, accessor: &str, dtype: LogValueType) -> Self{
        let mut new_vec: Vec<String> = Vec::new();
        for s in accessor.split('.') {
            new_vec.push(s.to_string());
        }
        Self {
            name: name.to_string(),
            accessor: new_vec,
            dtype
        }
    }
}

pub struct LogRecord {
    pub key: Option<String>,
    pub values: HashMap<String, LogValue>
}

impl LogRecord {
    pub fn new (key: &str) -> Self {
        Self {
            key: Some(String::from(key)),
            values: HashMap::new()
        }
    }

    pub fn parse(reader :&mut Box<dyn BufRead>, index: &Accessor, fields :&[&Accessor]) -> Result<LogRecord> {
        // read line
        let mut buf = String::new();
        reader.read_line(&mut buf).expect("error");
        let v: Value = serde_json::from_str(&buf)?;

        // Read key and init log record.
        let key = get_value(&v, &index.accessor, 0); 
        let mut record = LogRecord {
            key, values: HashMap::new()
        };

        // Read data
        for f in fields {
            let value = get_value(&v, &f.accessor, 0);
            if let Some(v) = value {
                record.set(f.name.as_str(), v, &f.dtype);
            }
        }
        Ok(record)
    }

    pub fn set(&mut self, key: &str, value: String, typ: &LogValueType) {
        let v = parse_value(typ, &value);
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

#[derive(Clone)]
pub enum LogValueType {
    String,
    Integer,
    Float,
    Second,
    None,
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
            LogValue::Integer(s) => write!(f, "Integer({})", s),
            LogValue::Float(s) => write!(f, "Float({})", s),
            LogValue::Second(s) => write!(f, "Second({}s)", s),
            _ => write!(f, "None")
        }
    }
}

impl LogValue {
    pub fn as_string(&self) -> String {
        let s = match self {
            LogValue::String(x) => {
                x.clone()
            },
            LogValue::Integer(x) => {
                x.to_string()
            },
            LogValue::Float(x) => {
                format!("{:.4}", x.to_string())
            },
            LogValue::Second(x) => {
                format!("{:.4}sec", x)
            },
            LogValue::None => {
                String::from("-")
            }
        };
        s
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

fn parse_value(typ: &LogValueType, s :&str) -> LogValue {
    match typ {
        LogValueType::String => {
            LogValue::String(s.to_string())
        },
        LogValueType::Integer => {
            if let Ok(num) = s.parse::<u32>() {
                LogValue::Integer(num)
            } else {
                LogValue::None
            }
        },
        LogValueType::Float => {
            if let Ok(num) = s.parse::<f64>() {
                LogValue::Float(num)
            } else {
                LogValue::None
            }            
        },
        LogValueType::Second => {
            let replace_and_second = |inp :&str, pattern: &str| {
                let raw = inp.replace(pattern, "");
                if let Ok(num) = raw.parse::<f64>() {
                    LogValue::Second(num)
                } else {
                    LogValue::None
                }     
            };
            if s.ends_with('s') {
                replace_and_second(s, "s")
            } else if s.ends_with("sec") {
                replace_and_second(s, "sec")
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
        let v = parse_value(&LogValueType::String, "test_string");
        if let LogValue::String(s) = v {
            assert_eq!(s, "test_string");
        } else {
            unreachable!();
        }

        // Check integer case.
        let v = parse_value(&LogValueType::Integer, "123");
        if let LogValue::Integer(n) = v {
            assert_eq!(n, 123);
        } else {
            unreachable!();
        }
        let v = parse_value(&LogValueType::Integer, "abc");
        assert!(matches!(v, LogValue::None));

        // Check float case.
        let v = parse_value(&LogValueType::Float, "123.4");
        if let LogValue::Float(n) = v {
            assert_eq!(n, 123.4);
        } else {
            unreachable!();
        }
        let v = parse_value(&LogValueType::Float, "abc");
        assert!(matches!(v, LogValue::None));

        // Check second case.
        let v = parse_value(&LogValueType::Second, "123.4s");
        if let LogValue::Second(n) = v {
            assert_eq!(n, 123.4);
        } else {
            unreachable!();
        }
        let v = parse_value(&LogValueType::Second, "123.4h");
        assert!(matches!(v, LogValue::None));
    }
}
