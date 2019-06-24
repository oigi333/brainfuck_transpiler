// This code is from taken directly from "https://github.com/Overv/bf"




use std::io::Read;

/// Opcodes determined by the lexer
#[derive(Debug)]
#[derive(Clone)]
enum OpCode {
    MovePointer(i128),
    AddValue(i128),
    Write,
    Read,
    LoopBegin,
    LoopEnd,
}

#[derive(Debug)]
#[derive(Clone)]
enum Instruction {
    MovePointer(i128),
    AddValue(i128),
    Write,
    Read,
    Loop(Vec<Instruction>)
}

/// Lexer turns the source code into a sequence of opcodes
fn lex(source: String) -> Vec<OpCode> {
    let mut operations: Vec<OpCode> = Vec::new();

    for symbol in source.chars() {
        let op = match symbol {
            '>' => {
                if let Some(OpCode::MovePointer(last)) = operations.last_mut() {
                    *last += 1;
                    None
                } else {
                    Some(OpCode::MovePointer(1))
                }
            },
            '<' => {
                if let Some(OpCode::MovePointer(last)) = operations.last_mut() {
                    *last -= 1;
                    None
                } else {
                    Some(OpCode::MovePointer(-1))
                }
            },
            '+' => {
                if let Some(OpCode::AddValue(last)) = operations.last_mut() {
                    *last += 1;
                    None
                } else {
                    Some(OpCode::AddValue(1))
                }
            },
            '-' => {
                if let Some(OpCode::AddValue(last)) = operations.last_mut() {
                    *last -= 1;
                    None
                } else {
                    Some(OpCode::AddValue(-1))
                }
            },
            '.' => Some(OpCode::Write),
            ',' => Some(OpCode::Read),
            '[' => Some(OpCode::LoopBegin),
            ']' => Some(OpCode::LoopEnd),
            _ => None
        };

        // Non-opcode characters are simply comments
        match op {
            Some(op) => operations.push(op),
            None => ()
        }
    }

    operations
}

fn parse(opcodes: Vec<OpCode>) -> Vec<Instruction>  {
    let mut program: Vec<Instruction> = Vec::new();
    let mut loop_stack = 0;
    let mut loop_start = 0;

    for (i, op) in opcodes.iter().enumerate() {
        if loop_stack == 0 {
            let instr = match op {
                OpCode::AddValue(value) => Some(Instruction::AddValue(*value)),
                OpCode::MovePointer(value) => Some(Instruction::MovePointer(*value)),
                OpCode::Write => Some(Instruction::Write),
                OpCode::Read => Some(Instruction::Read),

                OpCode::LoopBegin => {
                    loop_start = i;
                    loop_stack += 1;
                    None
                },

                OpCode::LoopEnd => panic!("loop ending at #{} has no beginning", i),
            };

            match instr {
                Some(instr) => program.push(instr),
                None => ()
            }
        } else {
            match op {
                OpCode::LoopBegin => {
                    loop_stack += 1;
                },
                OpCode::LoopEnd => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        program.push(Instruction::Loop(parse(opcodes[loop_start+1..i].to_vec())));
                    }
                },
                _ => (),
            }
        }
    }

    if loop_stack != 0 {
        panic!("loop that starts at #{} has no matching ending!", loop_start);
    }

    program
}

/// Executes a program that was previously parsed
fn run(instructions: &Vec<Instruction>, tape: &mut Vec<u64>, data_pointer: &mut usize) {
    for instr in instructions {
        match instr {
            Instruction::AddValue(value) => if *value > 0 { tape[*data_pointer] += *value as u64 } else {tape[*data_pointer] -= (-*value) as u64},
            Instruction::MovePointer(by) => if *by > 0 { *data_pointer += *by as usize } else {*data_pointer -= (-*by) as usize},
            Instruction::Write => {print!("{}", tape[*data_pointer] as u8 as char)},
            Instruction::Read => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin().read_exact(&mut input).expect("failed to read stdin");
                tape[*data_pointer] = input[0] as u64;
            },
            Instruction::Loop(nested_instructions) => {
                while tape[*data_pointer] != 0 {
                    run(&nested_instructions, tape, data_pointer)
                }
            }
        }
    }
}

pub fn interpret(code: String, tape_length: usize)  {
    let mut tape: Vec<u64> = vec![0; tape_length];
    let mut data_pointer = 0;
    //println!("{:?}", parse(lex(code)));

    run(&parse(lex(code)), &mut tape, &mut data_pointer);
    println!("{:?}", tape);
}