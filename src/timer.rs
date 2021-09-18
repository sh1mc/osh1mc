use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref TIMER: Mutex<Timer> = Mutex::new(Timer { value: 0 });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timer {
    value: u64,
}

impl Timer {
    pub fn inc(&mut self) {
        self.value += 1;
    }
    pub fn get(&self) -> u64 {
        self.value
    }
}
