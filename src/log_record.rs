
use std::fmt;
use std::io::{BufRead };
use std::cmp::{ Ordering, PartialOrd, Ord, PartialEq, Eq };
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


impl PartialEq for LogValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(x), Self::String(y)) => {
                x == y
            }
            (Self::Integer(x), Self::Integer(y)) => {
                x == y
            },
            (Self::Float(x), Self::Float(y)) => {
                (x - y) < 1e-10 
            }
            (Self::Second(x), Self::Second(y)) => {
                (x - y) < 1e-10 
            },
            _ => false
        }
    }
}

impl Eq for LogValue {}

impl PartialOrd for LogValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
     }
}

impl Ord for LogValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // Compare with same type.
            (Self::String(x), Self::String(y)) => {
                x.cmp(y)
            }
            (Self::Integer(x), Self::Integer(y)) => {
                x.cmp(y)
            },
            (Self::Float(x), Self::Float(y)) => {
                match x.partial_cmp(y) {
                    Some(x) => x,
                    None => Ordering::Equal
                }
            }
            (Self::Second(x), Self::Second(y)) => {
                match x.partial_cmp(y) {
                    Some(x) => x,
                    None => Ordering::Equal
                }
            },
            (Self::None, Self::None) => Ordering::Equal,

            // Different type comparison.
            (Self::String(_), _) => Ordering::Less,
            (Self::Integer(_), _) => Ordering::Less,
            (Self::Float(_), _) => Ordering::Less,
            (Self::Second(_), _) => Ordering::Less,
            (Self::None, _) => Ordering::Equal,

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
