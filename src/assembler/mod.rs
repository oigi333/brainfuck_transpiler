mod state;
pub use state::State;
mod memory;
pub use memory::{ Data };
mod generators;
pub use generators::*;

// TODO
// 1. Rename assembler to sth like intermediate_language_transpiler at first it was transpiling brainfuck assembly to brainfuck
// 2. Rename Mnemonic to sth else like OpCode, I've named it this way because initially it was supposed to contain only mnemonicish entries
// 3. Add branches for new op codes compiler

#[derive(Debug)]
pub enum Statement<'a> {
    Zero(&'a Data),
    Increment(&'a Data),
    Decrement(&'a Data),
    Add(&'a Data, &'a Data),
    AddMove(&'a Data, &'a Data),
    Subtract(&'a Data, &'a Data),
    SubtractMove(&'a Data, &'a Data),
    Multiply(&'a Data, &'a Data),
    Copy(&'a Data, &'a Data),
    Move(&'a Data, &'a Data),
    Write(&'a Data),
    Read(&'a Data),
    While(&'a Data, &'a Vec<Statement<'a>>)
}

pub fn assembler(statements: &Vec<Statement>, state: &mut State) {
    for statement in statements {
        match statement {
            Statement::Zero(var) => arithmetic::zero(state, var),
            Statement::Increment(var) => arithmetic::increment(state, var),
            Statement::Decrement(var) => arithmetic::decrement(state, var),
            Statement::Add(from, to) => arithmetic::add(state, from, to),
            Statement::AddMove(from, to) => arithmetic::add_move(state, from, to),
            Statement::Subtract(from, to) => arithmetic::subtract(state, from, to),
            Statement::SubtractMove(from, to) => arithmetic::subtract_move(state, from, to),
            Statement::Multiply(from, to) => arithmetic::multiply(state, from, to),
            Statement::Copy(from, to) => data::copy(state, from, to),
            Statement::Move(from, to) => data::move_variable(state, from, to),
            Statement::Write(var) => data::write(state, var),
            Statement::Read(var) => data::read(state, var),
            Statement::While(condition, loop_interior) =>
                control_flow::while_not_zero(state, condition, |state| assembler(loop_interior, state))
            //_ => ()
        }
    }
}