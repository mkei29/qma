
use crate::log_record::{ Accessor, LogRecord, LogValue };
use crate::operation::{ Operation, OpType, build_operation };

use std::collections::{ HashMap };
use std::io::{ BufRead };


pub struct Table {
    pub definition :TableDef,
    pub rows :HashMap<String, TableRow>,
    pub order: Option<Vec<String>>
}

impl Table {

    pub fn new(definition: TableDef) -> Self {
        Self {
            definition,
            rows: HashMap::new(),
            order: None,
        }
    }

    // Read one line from input reader and update row informations.
    pub fn aggregate(&mut self, mut reader: Box<dyn BufRead>) {
        loop {
            let record = LogRecord::parse(
                &mut reader, self.definition.key_accessor(), &self.definition.field_accessor()[..]);

            if let Ok(r) = record {
                let key = r.key.to_string();

                if let Some(row) = self.rows.get_mut(&key) {
                    row.update(&r, &self.definition.fields)
                } else {
                    self.rows.insert(r.key.to_string(), TableRow::new());
                }
            } else {
                break;
            }
        }
    }

    pub fn sort(&mut self) {

        if let Some(x) = &self.definition.order_by {
            // let mut order: Vec<(String, LogValue)> = vec![];
            // for (&key, row) in self.rows.iter() {
            //     horder.push((key, row.get("latent")));
            // }
        } else {
            let mut order: Vec<String> = vec![];
            for (key, _) in self.rows.iter() {
                order.push(key.clone());
                order.sort();
            }
            self.order = Some(order);
        }
    }
}

/// Struct which describe table row.
pub struct TableRow {
    // row name -> value
    values: HashMap<String, Box<dyn Operation>>,
}

impl Default for TableRow {
    fn default() -> Self {
        TableRow::new()
    }
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
                .or_insert_with(|| build_operation(&f.op_type));

            let v = record.get(f.name());
            if let Some(op) = self.values.get_mut(f.name()) {
                op.update(&v);
            }
        }
    }

    pub fn get(&self, field: &Field) -> LogValue {
        let v = self.values.get(field.name());
        if let Some(x) = v {
           return x.value(); 
        } else {
            return LogValue::None;
        }
    }

    /// Extracts data from records in the order specified by `field` argument.
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
    pub fields: Vec<Field>,
    pub order_by: Option<Field>
}

impl TableDef {
    pub fn new(index: Index, fields: Vec<Field>) -> Self {
        Self { index, fields, order_by: None }
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
    pub fn new(accessor: Accessor, op_type: OpType) -> Self {
        Self { accessor, op_type }
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

