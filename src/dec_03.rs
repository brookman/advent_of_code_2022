use std::collections::HashSet;

use crate::common::Solution;

pub struct Dec03 {}

impl Solution for Dec03 {
    fn solve_one(&self, lines: &Vec<String>) -> String {
        lines
            .iter()
            .map(|line| {
                let first_half = line[..(line.len() / 2)].to_string();
                let second_half = line[(line.len() / 2)..].to_string();
                calc_priority_of_single_intersection(&[first_half, second_half])
            })
            .sum::<u32>()
            .to_string()
    }

    fn solve_two(&self, lines: &Vec<String>) -> String {
        lines
            .chunks(3)
            .map(|chunk| calc_priority_of_single_intersection(chunk))
            .sum::<u32>()
            .to_string()
    }
}

fn calc_priority_of_single_intersection(lines: &[String]) -> u32 {
    let single_char = calc_intersection(lines).into_iter().next().unwrap();
    to_priority(&single_char)
}

fn calc_intersection(lines: &[String]) -> HashSet<char> {
    let sets: Vec<HashSet<char>> = lines
        .iter()
        .map(|line| line.chars())
        .map(|chars| HashSet::from_iter(chars))
        .collect();

    sets.iter().skip(1).fold(sets[0].clone(), |acc, hs| {
        acc.intersection(hs).cloned().collect()
    })
}

fn to_priority(c: &char) -> u32 {
    let ascii = c.clone() as u32;
    if ascii > 97 {
        return ascii - 96;
    } else {
        return ascii - 38;
    }
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
