use crate::instruction::*;

pub struct Compiler<'a> {
    code: &'a str,
    length: usize,
    position: usize,
    instructions: Vec<FoldedInstruction>,
}

impl Compiler<'_> {
    pub fn new(code: &str) -> Compiler {
        Compiler {
            code,
            length: code.len(),
            position: 0,
            instructions: Vec::new(),
        }
    }

    pub fn compile(&mut self) -> Vec<FoldedInstruction> {
        let mut loop_stack: Vec<usize> = Vec::new();
        while self.position < self.length {
            let instruction = self.code.chars().nth(self.position);
            match instruction {
                Some('+') => self.fold_instruction('+', Instruction::Plus),
                Some('-') => self.fold_instruction('-', Instruction::Minus),
                Some('<') => self.fold_instruction('<', Instruction::Left),
                Some('>') => self.fold_instruction('>', Instruction::Right),
                Some('.') => self.fold_instruction('.', Instruction::PutChar),
                Some(',') => self.fold_instruction(',', Instruction::ReadChar),
                Some('[') => {
                    let instruction_position =
                        self.emit_instruction(FoldedInstruction::new(Instruction::JumpIfZero, 0));
                    loop_stack.push(instruction_position);
                }
                Some(']') => {
                    if let Some(last_jmp_zero) = loop_stack.pop() {
                        let close_instruction_position = self.emit_instruction(
                            FoldedInstruction::new(Instruction::JumpIfNotZero, last_jmp_zero),
                        );
                        self.instructions[last_jmp_zero].argument = close_instruction_position;
                    }
                }
                None => panic!("error during compilation"),
                _ => {}
            }
            self.position += 1;
        }
        println!("{}", self.instructions.len());
        self.instructions.clone()
    }

    fn fold_instruction(&mut self, chr: char, instruction: Instruction) {
        let mut instruction_count = 1;
        while self.position < self.length - 1
            && self.code.chars().nth(self.position + 1) == Some(chr)
        {
            instruction_count += 1;
            self.position += 1;
        }
        self.emit_instruction(FoldedInstruction::new(instruction, instruction_count));
    }

    fn emit_instruction(&mut self, fi: FoldedInstruction) -> usize {
        self.instructions.push(fi);
        self.instructions.len() - 1
    }
}
