mod state;
pub use state::State;
mod memory;
pub use memory::{ Data };
mod generators;

// TODO
// 1. Rename assembler to sth like intermediate_language_transpiler at first it was transpiling brainfuck assembly to brainfuck
// 2. Rename Mnemonic to sth else like OpCode, I've named it this way because initially it was supposed to contain only mnemonicish entries
// 3. Add branches for new op codes

#[derive(Debug)]
pub enum Mnemonic<'a> {
    Zero(&'a Data),
    Increment(&'a Data),
    Decrement(&'a Data),
    Add(&'a Data, &'a Data),
    AddMove(&'a Data, &'a Data),
    Subtract(&'a Data, &'a Data),
    SubtractMove(&'a Data, &'a Data),
    Multiple(&'a Data, &'a Data),
    Copy(&'a Data, &'a Data),
    Move(&'a Data, &'a Data),
    Write(&'a Data),
    Read(&'a Data),
}

pub fn assembler(mnemonics: Vec<Mnemonic>, state: &mut State) {
    for op in mnemonics {
        match op {
            Mnemonic::Zero(var) => generators::zero(state, var),
            Mnemonic::Increment(var) => generators::increment(state, var),
            Mnemonic::Decrement(var) => generators::decrement(state, var),
            Mnemonic::Add(from, to) => generators::add(state, from, to),
            Mnemonic::AddMove(from, to) => generators::add_move(state, from, to),
            Mnemonic::Subtract(from, to) => generators::subtract(state, from, to),
            Mnemonic::SubtractMove(from, to) => generators::subtract_move(state, from, to),
            Mnemonic::Multiple(from, to) => generators::multiply(state, from, to),
            Mnemonic::Copy(from, to) => generators::copy(state, from, to),
            Mnemonic::Move(from, to) => generators::move_variable(state, from, to),
            Mnemonic::Write(var) => generators::write(state, var),
            Mnemonic::Read(var) => generators::read(state, var),
            //_ => ()
        }
    }
}