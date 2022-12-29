use std::{fmt, collections::VecDeque};

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
        write!(f, "") 
    }
}