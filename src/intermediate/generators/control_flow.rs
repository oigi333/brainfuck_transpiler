use super::{State, Data, pointer, arithmetic, data};

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

pub fn if_not_zero<C : FnOnce(&mut State)> (state: &mut State, condition: &Data, on_truth: C) {
    match condition {
        Data::Variable(_) => {

            let temp = state.memory.alloc();
            data::copy(state, condition, &temp);
            if_not_zero_move(state, &temp, on_truth);
            state.memory.free(temp);
        },
        Data::Constant(value) if *value != 0 => on_truth(state),
        _ => ()
    }
}

pub fn if_not_zero_move<C : FnOnce(&mut State)> (state: &mut State, condition: &Data, on_truth: C) {
    match condition {
        Data::Variable(_) => {
            pointer::set(state, condition);
            state.emit('[');
                on_truth(state);
                arithmetic::zero(state, condition);
            state.emit(']');
            arithmetic::zero(state, condition);
        },
        Data::Constant(value) if *value != 0 => on_truth(state),
        _ => ()
    }
}