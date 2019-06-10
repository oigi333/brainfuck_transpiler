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

    pub fn emit(&mut self, op: char) {
        self.code.push(op);
    }

    pub fn emit_multiple(&mut self, op: char, times: usize) {
        let mut ops = vec![op; times];
        self.code.append(&mut ops);
    }


}