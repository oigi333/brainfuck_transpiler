// TODO
// 1. Add freeing variables to destructor if possible

#[derive(Debug)]
pub enum Data {
    Variable(usize),
    Constant(u8)
}

#[derive(Debug)]
pub struct Memory {
    pointers: Vec<usize>
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            pointers: vec![0]
        }
    }

    pub fn alloc(&mut self) -> Data {
        let pointer = self.pointers.pop().unwrap();
        if self.pointers.len() == 0 {
            self.pointers.push(pointer + 1)
        }
        Data::Variable(pointer)
    }

    pub fn free(&mut self, variable: Data) {
        match variable {
            Data::Variable(pointer) => {
                let mut index: Option<usize> = Option::None;
                for i in (0..self.pointers.len()).rev() {
                    if self.pointers[i] > pointer {
                        index = Some(i);
                        break;
                    }
                }

                match index {
                    Some(index) => self.pointers.insert(index + 1, pointer),
                    None => ()
                }
            },
            _ => panic!("You can't free constants")
        }
    }
}

