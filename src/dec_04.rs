use crate::common::Solution;

pub struct Dec04 {}

impl Solution for Dec04 {
    fn solve_one(&self, lines: &Vec<&str>) -> String {
        let mut count = 0u32;
        for line in lines {
            let parts = line.split(",").collect::<Vec<&str>>();
            let first  = parts[0].split("-").collect::<Vec<&str>>();
            let second = parts[1].split("-").collect::<Vec<&str>>();

            let first_from = first[0].parse::<u32>().unwrap();
            let first_to = first[1].parse::<u32>().unwrap();

            let second_from = second[0].parse::<u32>().unwrap();
            let second_to = second[1].parse::<u32>().unwrap();

            if (first_from <= second_from && first_to >= second_to) || (second_from<=first_from   && second_to >=first_to ){
                count += 1;
            }

        }
        count.to_string()
    }

    fn solve_two(&self, lines: &Vec<&str>) -> String {
        let mut count = 0u32;
        for line in lines {
            let parts = line.split(",").collect::<Vec<&str>>();
            let first  = parts[0].split("-").collect::<Vec<&str>>();
            let second = parts[1].split("-").collect::<Vec<&str>>();

            let first_from = first[0].parse::<u32>().unwrap();
            let first_to = first[1].parse::<u32>().unwrap();

            let second_from = second[0].parse::<u32>().unwrap();
            let second_to = second[1].parse::<u32>().unwrap();

            if (first_from >= second_from && first_from <= second_to) || (first_to >= second_from && first_to <= second_to) ||
            (second_from >= first_from && second_from <= first_to) || (second_to >= first_from && second_to <= first_to){
                count += 1;
            }

        }
        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Dec04;
    use crate::{solve_one, solve_two};

    #[test]
    fn solution_one() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(solve_one(&Dec04 {}, input), "2");
    }

    #[test]
    fn solution_two() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(solve_two(&Dec04 {}, input), "4");
    }
}
