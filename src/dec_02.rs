use itertools::Itertools;

use crate::common::Solution;

pub struct Dec02 {}

impl Solution for Dec02 {
    fn solve_one(&self, lines: &Vec<String>) -> String {
        lines.iter().join(", ")
    }

    fn solve_two(&self, lines: &Vec<String>) -> String {
        lines.iter().join(", ")
    }
}
