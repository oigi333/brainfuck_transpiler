
mod interpret;
mod assembler;

use assembler::{assembler, Mnemonic, Data};


fn main() {

    /*let mut state = State::new();
    state.append('+', 49);
    state.add('.');*/
    let mut state = assembler::State::new();

    let a = state.memory.alloc();
    let b = state.memory.alloc();
    assembler(vec![
        Mnemonic::Add(&Data::Constant(7), &a),
        Mnemonic::Add(&Data::Constant(7), &b),
        Mnemonic::Multiple(&b, &a),
        Mnemonic::Write(&a)

    ], &mut state);

    println!("{:?}", state.get_brainfuck_code());

    interpret::interpret(
        format!("{}", state.get_brainfuck_code()),
        1024);


}