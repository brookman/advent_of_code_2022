use crate::common::Solution;

pub struct Dec06 {}

impl Solution for Dec06 {
    fn solve_one(&self, input: &str, _lines: &Vec<&str>) -> String {
        find_first_unique_slice(input, 4).unwrap().to_string()
    }

    fn solve_two(&self, input: &str, _lines: &Vec<&str>) -> String {
        find_first_unique_slice(input, 14).unwrap().to_string()
    }
}

fn find_first_unique_slice(input: &str, n: usize) -> Option<usize> {
    let chars: Vec<u8> = input.chars().map(|c| c as u8 - 'a' as u8).collect();

    for i in 0..(input.len() - n) {
        if all_different(&chars[i..i + n]) {
            return Some(i + n);
        }
    }
    None
}

fn all_different(slice: &[u8]) -> bool {
    slice
        .iter()
        .map(|c| 1 << c)
        .fold(0u32, |acc, v| acc | v)
        .count_ones()
        == slice.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{solve_one, solve_two};

    #[test]
    fn solution_one() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(solve_one(&Dec06 {}, input), "7");
    }

    #[test]
    fn solution_two() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(solve_two(&Dec06 {}, input), "19");
    }
}
