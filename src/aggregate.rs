
use std::collections::{ HashMap };

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

    pub fn update(&mut self) {
        self.count += 1;
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
            self.values.entry(f.name.clone())
                .or_insert(OpCount::new());

            if let Some(op) = self.values.get_mut(&f.name) {
                op.update();
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
    pub op: OpCount,
}

impl Field {
    pub fn new(name: &str, accessor: &[String]) -> Self {
        Self {
            name: name.to_owned(),
            accessor: accessor.to_vec(),
            op: OpCount::new() 
            // 型のタイプ
        }
    }    
}


pub struct LogRecord {
    pub key: String,
    pub values: HashMap<String, Option<String>>
}

impl LogRecord {
    pub fn new (key: &str) -> Self {
        Self {
            key: String::from(key),
            values: HashMap::new()
        }
    }

    pub fn set(&mut self, key: &str, value: Option<String>) {
        self.values.insert(key.to_string(), value);
    }
}