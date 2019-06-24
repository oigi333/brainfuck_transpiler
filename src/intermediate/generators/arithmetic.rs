use super::{ State, Data, pointer, control_flow };


pub fn zero(state: &mut State, var: &Data) {
    match var {
        Data::Variable(_) => {
            pointer::set(state, var);
            control_flow::basic_loop(state, |state| decrement(state, var));
        },
        _ => panic!("You can't zero constant")
    }
}

pub fn increment(state: &mut State, var: &Data) {
    match var {
        Data::Variable(_) => {
            pointer::set(state, var);
            state.emit('+');
        },
        _ => panic!("You can't increment constant")
    }
}

pub fn decrement(state: &mut State, var: &Data) {
    match var {
        Data::Variable(pointer) => {
            pointer::set(state, var);
            state.emit('-');
        },
        _ => panic!("You can't decrement constant")
    }
}

pub fn add(state: &mut State, b: &Data, a: &Data) {
    if let Data::Variable(_) = a {
        match b {
            Data::Constant(value) => {
                pointer::set(state, a);
                state.emit_multiple('+', *(value) as usize);
            },
            Data::Variable(_) if !pointer::is_equal(a, b)=> {
                let temp = state.memory.alloc();

                pointer::set(state, b);
                control_flow::basic_loop(state, |state| {
                    increment(state, a);
                    increment(state, &temp);
                    decrement(state, b);
                });
                add_move(state, &temp, b);

                state.memory.free(temp);
            },
            _ => {
                let temp = state.memory.alloc();
                add(state, b, &temp);
                add(state, &temp, a);
                state.memory.free(temp);
            }
        }
    } else {
        panic!("You can't add to constant")
    }
}

pub fn add_move(state: &mut State, from: &Data, to: &Data) {
    match (from, to) {
        (Data::Variable(_), Data::Variable(_)) if !pointer::is_equal(from, to) => {
            pointer::set(state, from);
            control_flow::basic_loop(state, |state| {
                increment(state, to);
                decrement(state, from);
            });
        },
        _ => panic!("You can only add move two distinct variables")
    }
}

pub fn subtract(state: &mut State, b: &Data, a: &Data) {
    if let Data::Variable(_) = a {
        match b {
            Data::Constant(value) => {
                pointer::set(state, a);
                state.emit_multiple('-', *(value) as usize);
            },
            Data::Variable(_) if !pointer::is_equal(a, b) => {
                let temp = state.memory.alloc();

                pointer::set(state, b);
                control_flow::basic_loop(state, |state| {
                    decrement(state, a);
                    increment(state, &temp);
                    decrement(state, b);
                });
                add_move(state, &temp, b);
                state.memory.free(temp);

            },
            _ => {
                zero(state, a);
            }
        }
    } else {
        panic!("You can't subtract from constant")
    }
}

pub fn subtract_move(state: &mut State, from: &Data, to: &Data) {
    match (from, to) {
        (Data::Variable(_), Data::Variable(_)) if !pointer::is_equal(from, to) => {
            pointer::set(state, from);
            control_flow::basic_loop(state, |state| {
                decrement(state, to);
                decrement(state, from);
            });
        },
        _ => panic!("You can't move from or to constant")
    }
}

pub fn multiply(state: &mut State, b: &Data, a: &Data) {
    if let Data::Variable(_) = a {
        match b {
            Data::Constant(value) => {

            },
            Data::Variable(_) if !pointer::is_equal(a, b) => {
                let temp = state.memory.alloc();
                add_move(state, a, &temp);

                pointer::set(state, &temp);
                control_flow::basic_loop(state, |state| {
                    add(state, b, a);
                    decrement(state, &temp);
                });

                state.memory.free(temp);
            },
            _ => {
                let temp = state.memory.alloc();

                add_move(state, b, &temp);
                multiply(state, &temp, a);

                zero(state, &temp);
                state.memory.free(temp);
            }
        }
    } else {
        panic!("You can't multiply and store into a constant")
    }
}