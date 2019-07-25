#[derive(Clone, Copy)]
pub enum Instruction {
    Plus,
    Minus,
    Right,
    Left,
    PutChar,
    ReadChar,
    JumpIfZero,
    JumpIfNotZero,
}

#[derive(Clone, Copy)]
pub struct FoldedInstruction {
    pub instruction: Instruction,
    pub argument: usize,
}

impl FoldedInstruction {
    pub fn new(i: Instruction, arg: usize) -> FoldedInstruction {
        FoldedInstruction {
            instruction: i,
            argument: arg,
        }
    }
}