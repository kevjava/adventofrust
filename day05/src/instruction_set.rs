
#[derive(Debug,Clone,Copy)]
pub(crate) struct Instruction {
    pub move_n: usize,
    pub from: usize,
    pub to: usize,
}

#[derive(Debug)]
pub(crate) struct InstructionSet {
    pub instructions: Vec<Instruction>,
}

impl InstructionSet {
    pub fn new(instructions: Vec<Instruction>) -> InstructionSet {
        InstructionSet {
            instructions,
        }
    }
}