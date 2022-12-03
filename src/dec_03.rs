use crate::common::Solution;
use itertools::Itertools;

pub struct Dec03 {}

impl Solution for Dec03 {
    fn solve_one(&self, lines: &Vec<String>) -> String {
        for line in lines {
            let columns = line.split(" ").collect::<Vec<&str>>();
        }

        "?".to_string()
    }

    fn solve_two(&self, lines: &Vec<String>) -> String {
        for line in lines {
            let columns = line.split(" ").collect::<Vec<&str>>();
        }

        "?".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Solution;

    use super::Dec03;

    #[test]
    fn solution_one() {
        let input = "1¨
2
3";
        let lines: Vec<String> = input
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let result = Dec03 {}.solve_one(&lines);

        assert_eq!(result, "?");
    }
    #[test]
    fn solution_two() {
        let input = "1¨
2
3";
        let lines: Vec<String> = input
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let result = Dec03 {}.solve_one(&lines);
        
        assert_eq!(result, "?");
    }
}
