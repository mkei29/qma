
use crate::log_record::{ Accessor, LogRecord, LogValue };
use crate::operation::{ Operation, OpCount, OpType };
use std::collections::{ HashMap };


pub struct TableRow {
    // row name -> value
    values: HashMap<String, Box<dyn Operation>>,
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
                .or_insert(Box::new(OpCount::new()));

            let v = record.get(f.name());
            if let Some(op) = self.values.get_mut(f.name()) {
                op.update(&v);
            }
        }
    }

    ///
    /// 
    pub fn get_row(&self, fields: &[Field]) -> Vec<LogValue> {
        let mut result: Vec<LogValue> = Vec::new();
        for f in fields {
            let v = self.values.get(f.name()).map(|x| x.value());
            if let Some(x) = v {
                result.push(x);
            } else {
                result.push(LogValue::None);
            }
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
    pub accessor: Accessor,
    pub op_type: OpType
}

impl Field {
    pub fn new(accessor: Accessor) -> Self {
        Self { accessor, op_type: OpType::COUNT }
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

