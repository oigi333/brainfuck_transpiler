use super::{ State, Data, arithmetic, pointer };

pub fn copy(state: &mut State, from: &Data, to: &Data) {
    arithmetic::zero(state, to);
    arithmetic::add(state, from , to);
}

pub fn move_variable(state: &mut State, from: &Data, to: &Data) {
    match (from, to) {
        (Data::Variable(_), Data::Variable(_)) => {
            arithmetic::zero(state, to);
            arithmetic::add_move(state, from, to);
        },
        _ => panic!("You can't move from or to constant")
    }
}

pub fn read(state: &mut State, var: &Data) {
    match var {
        Data::Variable(_) => {
            pointer::set(state, var);
            state.emit(',');
        },
        _ => panic!("You can't read constant")
    }
}

pub fn write(state: &mut State, var: &Data) {
    match var {
        Data::Variable(pointer) => {
            pointer::set(state, var);
            state.emit('.');
        },
        Data::Constant(_value) => {
            let temp = state.memory.alloc();
            arithmetic::add(state, var, &temp);
            write(state, &temp);
            arithmetic::zero(state, &temp);
            state.memory.free(temp);
        }
    }
}