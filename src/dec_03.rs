use std::collections::HashSet;

use crate::common::Solution;

pub struct Dec03 {}

impl Solution for Dec03 {
    fn solve_one(&self, lines: &Vec<String>) -> String {
        let mut sum = 0u32;
        for line in lines {
            let chars: Vec<char> = line.chars().collect();
            let half = chars.len() / 2;

            let first_half: HashSet<char> = HashSet::from_iter(chars[..half].iter().cloned());
            let second_half: HashSet<char> = HashSet::from_iter(chars[half..].iter().cloned());

            let in_both = first_half.intersection(&second_half).next().unwrap();
            sum += to_priority(in_both);
        }

        sum.to_string()
    }

    fn solve_two(&self, lines: &Vec<String>) -> String {
        let mut sum = 0u32;
        for chunk in lines.chunks(3) {
            let sets: Vec<HashSet<char>> = chunk
                .iter()
                .map(|line| line.chars())
                .map(|chars| HashSet::from_iter(chars))
                .collect();

            let intersection = sets.iter().skip(1).fold(sets[0].clone(), |acc, hs| {
                acc.intersection(hs).cloned().collect()
            });

            sum += to_priority(intersection.iter().next().unwrap());
        }

        sum.to_string()
    }
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
