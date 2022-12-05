use crate::common::Solution;

pub struct Dec03 {}

impl Solution for Dec03 {
    fn solve_one(&self, _: &str, lines: &Vec<&str>) -> String {
        lines
            .iter()
            .map(|line| {
                let (first_half, second_half) = line.split_at(line.len() / 2);
                (to_bitmask(first_half) & to_bitmask(second_half)).trailing_zeros()
            })
            .sum::<u32>()
            .to_string()
    }

    fn solve_two(&self, _: &str, lines: &Vec<&str>) -> String {
        lines
            .chunks(3)
            .map(|chunks| {
                (to_bitmask(&chunks[0]) & to_bitmask(&chunks[1]) & to_bitmask(&chunks[2]))
                    .trailing_zeros()
            })
            .sum::<u32>()
            .to_string()
    }
}

fn to_bitmask(line: &str) -> u64 {
    line.chars()
        .map(|c| 1u64 << to_priority(&c))
        .fold(0u64, |acc, v| acc | v)
}

fn to_priority(c: &char) -> u16 {
    let ascii = c.clone() as u16;
    ascii - if ascii > 97 { 96 } else { 38 }
}

#[cfg(test)]
mod tests {
    use super::Dec03;
    use crate::{solve_one, solve_two};

    #[test]
    fn solution_one() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(solve_one(&Dec03 {}, input), "157");
    }

    #[test]
    fn solution_two() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(solve_two(&Dec03 {}, input), "70");
    }
}
