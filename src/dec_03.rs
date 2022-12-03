use crate::common::Solution;

pub struct Dec03 {}

impl Solution for Dec03 {
    fn solve_one(&self, lines: &Vec<String>) -> String {
        lines
            .iter()
            .map(|line| {
                let half = line.len() >> 1;
                (to_bitmask(&line[..half]) & to_bitmask(&line[half..])).trailing_zeros()
            })
            .sum::<u32>()
            .to_string()
    }

    fn solve_two(&self, lines: &Vec<String>) -> String {
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
    use crate::common::Solution;

    use super::Dec03;

    #[test]
    fn solution_one() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let lines: Vec<String> = input
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let result = Dec03 {}.solve_one(&lines);

        assert_eq!(result, "157");
    }

    #[test]
    fn solution_two() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let lines: Vec<String> = input
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let result = Dec03 {}.solve_two(&lines);

        assert_eq!(result, "70");
    }
}
