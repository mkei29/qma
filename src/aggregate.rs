
use crate::log_record::{ Accessor, LogRecord, LogValue };
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

    pub fn update(&mut self, v :&LogValue) {
        match v {
            LogValue::None => {},
            _ => { self.count += 1 }
        };
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
            self.values.entry(f.name().to_string())
                .or_insert_with(OpCount::new);

            let v = record.get(f.name());
            if let Some(op) = self.values.get_mut(f.name()) {
                op.update(&v);
            }
        }
    }

    ///
    /// 
    pub fn get_row(&self, fields: &[Field]) -> Vec<Option<u32>> {
        let mut result: Vec<Option<u32>> = Vec::new();
        for f in fields {
            let v = self.values.get(f.name()).map(|x| x.value());
            result.push(v);
        }
        result
    }
}


pub struct TableDef {
    pub index: Index,
    pub fields: Vec<Field>
}

impl TableDef {
    pub fn new(index: Index, fields: Vec<Field>) -> Self {
        Self { index, fields }
    }

    pub fn field_accessor(&self) -> Vec<&Accessor> {
        let mut result :Vec<&Accessor> = Vec::new();
        for f in self.fields.iter() {
            result.push(&f.accessor);
        } 
        result
    }

    pub fn key_accessor(&self) -> &Accessor {
        &self.index.accessor
    }

    pub fn field_num(&self) -> usize {
        self.fields.len()
    }
}

#[derive(Clone)]
pub struct Field {
    pub accessor: Accessor
}

impl Field {
    pub fn new(accessor: Accessor) -> Self {
        Self { accessor }
    }

    pub fn name(&self) -> &str {
        &self.accessor.name
    }
}

pub struct Index {
    pub accessor: Accessor
}

impl Index {
    pub fn new(accessor: Accessor) -> Self {
        Self { accessor }
    }    

    pub fn name(&self) -> &str {
        &self.accessor.name
    }
}

