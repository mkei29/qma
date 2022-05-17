

use crate::log_record::{ LogValue };

#[derive(Clone)]
pub enum OpType {
    Count,
    Average,
}

pub trait Operation {
    fn update(&mut self,  v :&LogValue); 
    fn value(&self) -> LogValue;
}


pub fn build_operation(op_type: &OpType) -> Box<dyn Operation> {
    match op_type {
        OpType::Count => {
            Box::new(OpCount::new())
        },
        OpType::Average => {
            Box::new(OpAverage::new())
        } 
    }
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

pub struct OpAverage {
    sum: f64,
    count: u32,
}

impl Operation for OpAverage {
    fn update(&mut self, v: &LogValue) {
        match v {
            LogValue::Integer(x) => {
                self.sum += *x as f64;
                self.count += 1;
            },
            LogValue::Float(x) => {
                self.sum += *x;
                self.count += 1;
            },
            LogValue::Second(x) => {
                self.sum += *x;
                self.count += 1;
            }
            _ => { 
            }
        };
    }

    fn value(&self) -> LogValue{
        if self.count == 0 {
           return LogValue::None; 
        }
        LogValue::Float(self.sum/(self.count as f64))
    }
}

impl OpAverage {
    pub fn new() -> Self {
        Self { sum: 0., count: 0 }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn check_op_average() {
        let mut op = OpAverage::new();
        for n in 1..6 {
            op.update(&LogValue::Float(n as f64));
        }
        if let LogValue::Float(x) = op.value() {
            assert_eq!(x, 3.);
        } else {
            unreachable!();
        }
    }

    #[test]
    fn check_op_count() {
        let mut op = OpCount::new();
        for n in 1..6 {
            op.update(&LogValue::Integer(n));
        }
        if let LogValue::Integer(x) = op.value() {
            assert_eq!(x, 5);
        } else {
            unreachable!();
        }
    }
}