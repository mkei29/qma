

pub trait Operation {

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

    pub fn update(&mut self) {
        self.count += 1;
    }

    pub const fn value(&mut self) -> u32{
        self.count
    }
}
