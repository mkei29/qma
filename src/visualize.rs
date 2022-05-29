

use std::cmp;
use std::collections::{HashMap};

pub use crate::aggregate::{ TableRow, TableDef, Index, Field };
pub use crate::log_record::{ LogValueType, LogValue, LogRecord, Accessor };


pub enum VisualizeType {
    Csv,
    Markdown
}

pub fn display_as_csv(def: &TableDef, table: &HashMap<String, TableRow>) {
    // # Display aggregated result as csv format.
    let mut header = String::from("");
    for  (i, &f) in def.field_accessor().iter().enumerate() {
        header += &f.name;
        if i != def.field_num() {
            header += ",";
        }
    }
    println!("{}", &header);

    for (key, row) in table.iter() {
        let mut row_str = String::from(key);
        row_str += ",";

        for (i, v) in row.get_row(&def.fields).iter().enumerate() {

            let s  = v.as_string();
            row_str += &s;
            if i != def.field_num() {
                row_str += ",";
            }

        }
        println!("{}", &row_str);
    }
}


pub fn display_as_markdown(def: &TableDef, table: &HashMap<String, TableRow>) {
    // # Display aggregated result as csv format.

    // count max chars of each fields;
    let mut col_width = vec![10; 1+def.fields.len()];
    col_width[0] = cmp::max(col_width[0], def.index.name().chars().count());
    for (i, &f) in def.field_accessor().iter().enumerate() {
        col_width[i+1] = cmp::max(col_width[i+1], f.name.chars().count());
    }
    
    // print header;
    let mut header = String::from("|");
    header += &format_string(def.index.name(), col_width[0]);
    header += "|";
    for (i, &f) in def.field_accessor().iter().enumerate() {
        let width = col_width[i+1];
        header += &format_string(&f.name, width);
        header += "|"
    }
    println!("{}", &header);

    // print separator
    let mut separator = String::new();
    for w in col_width.iter() {
        separator += &format!("|{}", "-".repeat(*w));
    }
    separator += "|";
    println!("{}", separator);

    // print table
    for (key, row) in table.iter() {
        let width = col_width[0];
        let mut row_str = String::from("|");
        row_str += &format_string(key, width);
        row_str += "|";

        for (i, v) in row.get_row(&def.fields).iter().enumerate() {
            let width = col_width[i+1];
            row_str += &format_log_value(v, width);
            if i != def.field_num() {
                row_str += "|";
            }

        }
        println!("{}", &row_str);
    }
}


fn format_log_value(v: &LogValue, width: usize) -> String{
    let s  = v.as_string();
    format!("{:>width$}", &s, width = width)
}

fn format_string(s: &str, width: usize) -> String{
    format!("{:width$}", s, width = width)
}