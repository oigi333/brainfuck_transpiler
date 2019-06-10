use super::{State, Data};

pub fn move_left(state: &mut State, by: usize) {
    state.emit_multiple('<', by );
    state.current_pointer -= by;
}

pub fn move_right(state: &mut State, by: usize) {
    state.emit_multiple('>', by );
    state.current_pointer += by;
}

pub fn set_ptr(state: &mut State, pointer: usize) {
    let difference = (pointer as i64) - (state.current_pointer as i64);
    if difference > 0 {
        move_right(state, difference as usize);
    }
    else if difference < 0 {
        move_left(state, (-difference) as usize);
    }
}

pub fn set(state: &mut State, var: &Data) {
    match var {
        Data::Variable(ptr) => set_ptr(state, *ptr),
        _ => panic!("You can't set pointer to constant")
    }
}

pub fn is_equal(left: &Data, right: &Data) -> bool {
    match (left, right) {
        (Data::Variable(left_ptr), Data::Variable(right_ptr)) => left_ptr == right_ptr,
        _ => panic!("You can't pointerwise compare two constants")
    }
}