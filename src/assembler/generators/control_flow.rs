use super::{State, Data, pointer};

pub fn basic_loop<C : FnOnce(&mut State)> (state: &mut State, code: C) {
    state.emit('[');
    code(state);
    state.emit(']');
}

pub fn while_not_zero<C : FnOnce(&mut State)> (state: &mut State, condition: &Data, code: C) {
    match condition {
        Data::Variable(_) => {
            pointer::set(state, condition);
            state.emit('[');
                code(state);
                pointer::set(state, condition);
            state.emit(']');
        },
        Data::Constant(value) if *value != 0 => panic!("Infinite loop"),
        _ => ()
    }
}