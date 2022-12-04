extern crate gcollections;
extern crate interval;

use gcollections::ops::{Overlap, Subset};
use interval::{ops::Range, Interval};

use crate::common::Solution;

pub struct Dec04 {}

impl Solution for Dec04 {
    fn solve_one(&self, lines: &Vec<&str>) -> String {
        lines
            .iter()
            .map(|line| parse(line))
            .map(|(a, b)| (a.is_subset(&b) || b.is_subset(&a)) as u32)
            .sum::<u32>()
            .to_string()
    }

    fn solve_two(&self, lines: &Vec<&str>) -> String {
        lines
            .iter()
            .map(|line| parse(line))
            .map(|(a, b)| a.overlap(&b) as u32)
            .sum::<u32>()
            .to_string()
    }
}

fn parse(line: &str) -> (Interval<u32>, Interval<u32>) {
    let parts = line
        .split(&[',', '-'])
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    (
        Interval::new(parts[0], parts[1]),
        Interval::new(parts[2], parts[3]),
    )
}

#[cfg(test)]
mod tests {
    use super::Dec04;
    use crate::{solve_one, solve_two};

    #[test]
    fn solution_one() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(solve_one(&Dec04 {}, input), "2");
    }

    #[test]
    fn solution_two() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(solve_two(&Dec04 {}, input), "4");
    }
}
