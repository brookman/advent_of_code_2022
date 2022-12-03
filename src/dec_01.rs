use itertools::Itertools;

use crate::common::Solution;

pub struct Dec01 {}

impl Solution for Dec01 {
    fn solve_one(&self, lines: &Vec<&str>) -> String {
        get_sum_of_blocks(lines).iter().max().unwrap().to_string()
    }

    fn solve_two(&self, lines: &Vec<&str>) -> String {
        get_sum_of_blocks(lines)
            .into_iter()
            .sorted_by(|a, b| b.cmp(a))
            .take(3)
            .sum::<u32>()
            .to_string()
    }
}

fn get_sum_of_blocks(lines: &Vec<&str>) -> Vec<u32> {
    let mut sums: Vec<u32> = vec![0];
    for line in lines {
        if line.is_empty() {
            sums.push(0);
        } else {
            let index = sums.len() - 1;
            sums[index] += line.parse::<u32>().unwrap();
        }
    }
    sums
}
