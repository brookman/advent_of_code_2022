use crate::common::Solution;

pub struct Dec04 {}

impl Solution for Dec04 {
    fn solve_one(&self, lines: &Vec<&str>) -> String {
        for line in lines {}
        "".to_string()
    }

    fn solve_two(&self, lines: &Vec<&str>) -> String {
        for line in lines {}
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::{solve_one, solve_two};

    use super::Dec04;

    #[test]
    fn solution_one() {
        let input = "1
2
3";
        assert_eq!(solve_one(&Dec04 {}, input), "");
    }

    #[test]
    fn solution_two() {
        let input = "1
2
3";
        assert_eq!(solve_two(&Dec04 {}, input), "");
    }
}
