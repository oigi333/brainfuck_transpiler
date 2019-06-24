
mod interpret;
mod intermediate;

use intermediate::{intermediate_transpiler, State, Statement, Data};


fn main() {
    /*let mut state = State::new();
    state.append('+', 49);
    state.add('.');*/
    let mut state = intermediate::State::new();

    let penultimate = state.memory.alloc();
    let last = state.memory.alloc();
    let current = state.memory.alloc();
    let i = state.memory.alloc();
    intermediate_transpiler(&vec![
        Statement::Add(&Data::Constant(0), &last),
        Statement::Add(&Data::Constant(1), &current),
        Statement::Read(&i),
        Statement::Subtract(&Data::Constant(48), &i),
        Statement::While(&i, &vec![
            Statement::Copy(&last, &penultimate),
            Statement::Copy(&current, &last),
            Statement::Copy(&last, &current),
            Statement::Add(&penultimate, &current),
            Statement::Decrement(&i)
        ]),
        Statement::Add(&Data::Constant(48), &current),
        Statement::Write(&current)

    ], &mut state);

    println!("{:?}", state.get_brainfuck_code());

    interpret::interpret(
        format!("{}", state.get_brainfuck_code()),
        1024);
    state.memory.free(last);
    state.memory.free(current);
    state.memory.free(i);

    println!("{:?}", state.memory);
}