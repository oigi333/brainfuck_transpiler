use super::memory::{Memory};

// TODO
// Move pointer code generators to generator module

#[derive(Debug)]
pub struct State {
    code: Vec<char>,
    pub memory: Memory,
    pub current_pointer: usize
}

impl State {
    pub fn new() -> State {
        State {
            code: Vec::new(),
            memory: Memory::new(),
            current_pointer: 0
        }
    }

    pub fn get_brainfuck_code(&self) -> String {
        self.code.iter().collect()
    }

    pub fn add(&mut self, op: char) {
        self.code.push(op);
    }

    pub fn append(&mut self, op: char, times: usize) {
        let mut ops = vec![op; times];
        self.code.append(&mut ops);
    }

    pub fn shift_pointer_left(&mut self, by: usize) {
        self.append('<', by );
        self.current_pointer -= by;
    }

    pub fn shift_pointer_right(&mut self, by: usize) {
        self.append('>', by );
        self.current_pointer += by;
    }

    pub fn set_pointer_to(&mut self, pointer: usize) {
        let difference = (pointer as i64) - (self.current_pointer as i64);
        if difference > 0 {
            self.shift_pointer_right(difference as usize);
        }
        else if difference < 0 {
            self.shift_pointer_left((-difference) as usize)
        }
    }
}