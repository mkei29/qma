

enum OpType {
    COUNT,
    COUNT_RATE,
}

enum OpValue {
    Integer(u32)
}
pub trait Operation {
    pub fn update(&mut self); 
    pub fn value(&self) -> OpValue;
}

pub struct OpCount {
    count: u32
}

impl OpCount {
    pub fn new() -> Self{
        Self {
            count: 0
        }
    }
}

impl Operation for OpCount {
    fn update(&mut self) {
        self.count += 1;
    }

    const fn value(&mut self) -> OpValue{
        OpValue(self.count)
    }
}
