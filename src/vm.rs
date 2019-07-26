use crate::instruction::*;
use std::io::{Read, Write};

pub struct Machine {
    int_ptr: usize,
    data_ptr: usize,
    memory: [u8; 30000],
    buffer: [u8; 1],
    code: Vec<FoldedInstruction>,
    input: Box<Read>,
    output: Box<Write>,
}

impl Machine {
    pub fn new(code: Vec<FoldedInstruction>, input: Box<Read>, output: Box<Write>) -> Machine {
        Machine {
            int_ptr: 0,
            data_ptr: 0,
            memory: [0; 30000],
            buffer: [0; 1],
            code: code,
            input: input,
            output: output,
        }
    }

    pub fn execute(&mut self) {
        while self.int_ptr < self.code.len() {
            let folded_inst: FoldedInstruction = self.code[self.int_ptr];
            match folded_inst.instruction {
                Instruction::Plus => self.memory[self.data_ptr] += folded_inst.argument as u8,
                Instruction::Minus => self.memory[self.data_ptr] -= folded_inst.argument as u8,
                Instruction::Right => self.data_ptr += folded_inst.argument as usize,
                Instruction::Left => self.data_ptr -= folded_inst.argument as usize,
                Instruction::PutChar => {
                    for _ in 0..folded_inst.argument {
                        self.put_char()
                    }
                }
                Instruction::ReadChar => {
                    for _ in 0..folded_inst.argument {
                        self.read_char()
                    }
                }
                Instruction::JumpIfZero => {
                    if self.memory[self.data_ptr] == 0 {
                        self.int_ptr = folded_inst.argument as usize;
                        continue;
                    }
                }
                Instruction::JumpIfNotZero => {
                    if self.memory[self.data_ptr] != 0 {
                        self.int_ptr = folded_inst.argument as usize;
                        continue;
                    }
                }
            }
            self.int_ptr += 1;
        }
    }

    fn read_char(&mut self) {
        //self.buffer[0] = self.memory[self.data_ptr]
        // let buf : &mut [u8] = ;
        let r = self.input.read(&mut (self.buffer));
        match r {
            // This probably errors out
            Ok(bytes_read) => {
                if bytes_read == 1 {
                    self.memory[self.data_ptr] = self.buffer[0]
                } else {
                    panic!("wrong number of bytes read")
                }
            }
            Err(e) => panic!(e),
        }
    }

    fn put_char(&mut self) {
        self.buffer[0] = self.memory[self.data_ptr];
        let r = self.output.write(&(self.buffer));
        match r {
            Ok(bytes_written) => {
                if bytes_written != 1 {
                    panic!("wrong number of bytes written")
                }
            }
            Err(e) => panic!(e),
        }
        // let f = self.output.flush();
        // match f {
        //     Ok(()) => {}
        //     Err(e) => panic!(e),
        // }
    }
}
