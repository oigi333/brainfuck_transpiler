use super::memory::{Memory};
use std::collections::HashMap;

#[derive(Debug)]
pub struct State {
    code: Vec<char>,
    pub memory: Memory,
    pub current_pointer: usize
}

impl State {
    pub fn new() -> Self {
        let mut memory = Memory::new();

        State {
            code: Vec::new(),
            memory,
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