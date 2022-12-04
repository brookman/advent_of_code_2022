
use crate::common::Solution;

pub struct Dec05 {}

impl Solution for Dec05 {
    fn solve_one(&self, lines: &Vec<&str>) -> String {
        lines
            .iter()
            .count()
            .to_string()
    }

    fn solve_two(&self, lines: &Vec<&str>) -> String {
        lines
            .iter()
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Dec05;
    use crate::{solve_one, solve_two};

    #[test]
    fn solution_one() {
        let input = "1
2
3";
        assert_eq!(solve_one(&Dec05 {}, input), "3");
    }

    #[test]
    fn solution_two() {
        let input = "1
2
3";
        assert_eq!(solve_two(&Dec05 {}, input), "3");
    }
}
