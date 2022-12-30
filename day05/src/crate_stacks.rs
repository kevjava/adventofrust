use std::{fmt, collections::VecDeque};

use crate::instruction_set::Instruction;

#[derive(Clone,Copy)]
pub(crate) struct Crate {
    label: char,
}
impl Crate {
    pub(crate) fn new(label: char) -> Crate {
        Crate { label }
    }
}

impl fmt::Debug for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("[{label}]", label = self.label))
    }
}

pub(crate) struct CrateStacks {
    stacks: Vec<VecDeque<Crate>>,
}

impl CrateStacks {
    pub fn new(stacks: Vec<VecDeque<Crate>>) -> CrateStacks {
        CrateStacks {
            stacks,
        }
    }

    fn max_stack_size(&self) -> usize {
        self.stacks.iter().map(|q| q.len()).max().unwrap_or(0)
    }

    pub(crate) fn perform_instruction(&mut self, instruction: &Instruction) -> () {
        for _ in 0..instruction.move_n {
            let this_crate = self.stacks[instruction.from-1].pop_back().unwrap();
            self.stacks[instruction.to-1].push_back(this_crate);
        }
    }

    pub(crate) fn perform_instruction_multiple(&mut self, instruction: &Instruction) -> () {
        let mut moved_crates = VecDeque::new();
        for _ in 0..instruction.move_n {
            let this_crate = &self.stacks[instruction.from-1].pop_back().unwrap();
            moved_crates.push_back( Crate { label: this_crate.label } );
        }

        for moved_crate in moved_crates.iter().rev().collect::<VecDeque<&Crate>>() {
            let _ = &self.stacks[instruction.to-1].push_back(Crate { label: moved_crate.label } );
        }
    }

    pub(crate) fn get_top(&self) -> String {
        let mut ret = String::new();
        for stack in &self.stacks {
            let s = stack.back().unwrap().label.to_string();
            ret.push_str(&s);
       } 
       ret
    }

}

impl fmt::Debug for CrateStacks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in (0..self.max_stack_size()).rev() {
            for j in 0..self.stacks.len() {
                let maybe_crate = self.stacks[j].get(i);
                match maybe_crate {
                    Some(a_crate) => { f.write_fmt(format_args!("{:?} ", a_crate))?; }
                    _ => { f.write_str("    ")?; }
                }
            }
            f.write_str("\n")?;
        }
        for i in 0..self.stacks.len() {
            write!(f, " {}  ", (i+1))?
        }
        write!(f, "\n") 
    }
}