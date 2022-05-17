

use crate::log_record::{ LogValue };

#[derive(Clone)]
pub enum OpType {
    COUNT,
    COUNT_RATE,
}

pub trait Operation {
    fn update(&mut self,  v :&LogValue); 
    fn value(&self) -> LogValue;
}

#[derive(Clone)]
pub struct OpCount {
    count: u32
}

impl Operation for OpCount {
    fn update(&mut self, v :&LogValue) {
        match v {
            LogValue::None => {},
            _ => { self.count += 1 }
        };
    }

    fn value(&self) -> LogValue{
        LogValue::Integer(self.count)
    }
}

impl OpCount {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}