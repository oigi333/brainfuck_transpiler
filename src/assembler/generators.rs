use super::{State, Data};

// TODO
// 1. Split file to separate modules like arithmetic, memory, control_flow etc.
// 2. Move pointer manipulation to memory module from state
// 3. Rename stuff like loop_basic to sth more appropriate
// 4. Add new generators

pub fn loop_basic<C : FnOnce(&mut State)> (state: &mut State, code: C) {
    state.add('[');
    code(state);
    state.add(']');
}

pub fn zero(state: &mut State, var: &Data) {
    match var {
        Data::Variable(pointer) => {
            state.set_pointer_to(*pointer);
            loop_basic(state, |state| decrement(state, var));
        },
        _ => panic!("You can't zero constant")
    }
}

pub fn increment(state: &mut State, var: &Data) {
    match var {
        Data::Variable(pointer) => {
            state.set_pointer_to(*pointer);
            state.add('+');
        },
        _ => panic!("You can't increment constant")
    }
}

pub fn decrement(state: &mut State, var: &Data) {
    match var {
        Data::Variable(pointer) => {
            state.set_pointer_to(*pointer);
            state.add('-');
        },
        _ => panic!("You can't decrement constant")
    }
}

pub fn add(state: &mut State, b: &Data, a: &Data) {
    if let Data::Variable(to_ptr) = a {
        match b {
            Data::Constant(value) => {
                state.set_pointer_to(*to_ptr);
                state.append('+', *(value) as usize);
            },
            Data::Variable(from_ptr) if from_ptr != to_ptr => {
                let temp = state.memory.alloc();

                state.set_pointer_to(*from_ptr);
                loop_basic(state, |state| {
                    increment(state, a);
                    increment(state, &temp);
                    decrement(state, b);
                });
                add_move(state, &temp, b);

                state.memory.free(temp);
            },
            _ => {
                let temp = state.memory.alloc();
                add_move(state, b, &temp);
                add(state, &temp, &a);
                state.memory.free(temp);
            }
        }
    } else {
        panic!("You can't add to constant")
    }
}

pub fn add_move(state: &mut State, from: &Data, to: &Data) {
    match (from, to) {
        (Data::Variable(from_ptr), Data::Variable(_to_ptr)) => {
            state.set_pointer_to(*from_ptr);
            loop_basic(state, |state| {
                increment(state, to);
                decrement(state, from);
            });
        },
        _ => panic!("You can't move from or to constant")
    }
}

pub fn subtract(state: &mut State, b: &Data, a: &Data) {
    if let Data::Variable(to_ptr) = a {
        match b {
            Data::Constant(value) => {
                state.set_pointer_to(*to_ptr);
                state.append('-', *(value) as usize);
            },
            Data::Variable(from_ptr) if from_ptr != to_ptr => {
                let temp = state.memory.alloc();

                state.set_pointer_to(*from_ptr);
                loop_basic(state, |state| {
                    decrement(state, a);
                    increment(state, &temp);
                    decrement(state, b);
                });
                add_move(state, &temp, b);

                state.memory.free(temp);
            },
            _ => {
                let temp = state.memory.alloc();
                add_move(state, b, &temp);
                subtract(state, &temp, a);
                state.memory.free(temp);
            }
        }
    } else {
        panic!("You can't subtract from constant")
    }
}

pub fn subtract_move(state: &mut State, from: &Data, to: &Data) {
    match (from, to) {
        (Data::Variable(from_ptr), Data::Variable(_to_ptr)) => {
            state.set_pointer_to(*from_ptr);
            loop_basic(state, |state| {
                decrement(state, to);
                decrement(state, from);
            });
        },
        _ => panic!("You can't move from or to constant")
    }
}

pub fn multiply(state: &mut State, b: &Data, a: &Data) {
    if let Data::Variable(to_ptr) = a {
        match b {
            Data::Variable(from_ptr) if from_ptr != to_ptr => {
                let temp = state.memory.alloc();
                add(state, b, &temp);
                decrement(state, &temp);
                loop_basic(state, |state| {
                    add(state, b, a);
                    state.add(' ');
                    decrement(state, &temp);
                });
                state.memory.free(temp);
            },
            _ => {
                let temp = state.memory.alloc();

                match b {
                    Data::Variable(_) => add_move(state, b, &temp),
                    _ => add(state, b, &temp)
                }

                multiply(state, &temp, a);
                state.memory.free(temp);
            }
        }
    } else {
        panic!("You can't multiply and store into a constant")
    }
}

pub fn copy(state: &mut State, from: &Data, to: &Data) {
    zero(state, to);
    add(state, from , to);
}

pub fn move_variable(state: &mut State, from: &Data, to: &Data) {
    match (from, to) {
        (Data::Variable(_), Data::Variable(_)) => {
            zero(state, to);
            add_move(state, from, to);
        },
        _ => panic!("You can't move from or to constant")
    }
}

pub fn read(state: &mut State, var: &Data) {
    match var {
        Data::Variable(pointer) => {
            state.set_pointer_to(*pointer);
            state.add(',');
        },
        _ => panic!("You can't read constant")
    }
}

pub fn write(state: &mut State, var: &Data) {
    match var {
        Data::Variable(pointer) => {
            state.set_pointer_to(*pointer);
            state.add('.');
        },
        Data::Constant(_value) => {
            let temp = state.memory.alloc();
            add(state, var, &temp);
            write(state, &temp);
            zero(state, &temp);
            state.memory.free(temp);
        }
    }
}