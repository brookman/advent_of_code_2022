use crate::common::Solution;

pub struct Dec07 {}

impl Solution for Dec07 {
    fn solve_one(&self, _input: &str, lines: &Vec<&str>) -> String {
        lines.iter().count().to_string()
    }

    fn solve_two(&self, _input: &str, lines: &Vec<&str>) -> String {
        lines.iter().count().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{solve_one, solve_two};

    #[test]
    fn solution_one() {
        let input = "1
2
3";
        assert_eq!(solve_one(&Dec07 {}, input), "3");
    }

    #[test]
    fn solution_two() {
        let input = "1
2
3";
        assert_eq!(solve_two(&Dec07 {}, input), "3");
    }
}
