use std::fmt::{Display, self};


#[derive(Debug,Clone,Copy)]
pub struct Assignment {
    start: usize,
    end: usize,
}

impl Assignment {
    pub fn new(line: &str) -> Assignment {
        let assn_strs = line
            .split("-")
            .collect::<Vec<&str>>();
        assert!(assn_strs.len() == 2);
    
        Assignment {
            start: assn_strs[0].parse::<usize>().unwrap_or(0),
            end: assn_strs[1].parse::<usize>().unwrap_or(0),
        }
    }

    pub fn contains(self, other: Assignment) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub(crate) fn overlaps_with(&self, other: Assignment) -> bool {
        other.start >= self.start && other.start <= self.end || 
        self.start >= other.start && self.start <= other.end
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}-{}", self.start, self.end))
    }
}

#[derive(Debug,Clone,Copy)]
pub(crate) struct AssignmentPair {
    assignments: (Assignment, Assignment),
}

impl AssignmentPair {
    pub fn new(line: &str) -> AssignmentPair {
        let assignments = line.split(",")
            .map(|assn_str| Assignment::new(assn_str) )
            .collect::<Vec<Assignment>>();
        assert!(assignments.len() == 2);

        AssignmentPair {
            assignments: (assignments[0], assignments[1]),
        }
    }

    pub fn first(self) -> Assignment {
        self.assignments.0
    }

    pub fn second(self) -> Assignment {
        self.assignments.1
    }

    pub fn contains(self) -> bool {
        self.first().contains(self.second()) || self.second().contains(self.first())
    }

    pub fn overlaps(self) -> bool {
        self.first().overlaps_with(self.second())
    }
}