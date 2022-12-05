use std::vec;

use crate::common::Solution;

pub struct Dec05 {}

impl Solution for Dec05 {
    fn solve_one(&self, input: &str, lines: &Vec<&str>) -> String {
        let mut stack_lines: Vec<&str> = vec![];
        let mut move_lines: Vec<&str> = vec![];

        for line in lines {
            if line.starts_with(" 1") {
                // ignore
            } else if line.starts_with("move") {
                move_lines.push(line);
            } else if line.trim().starts_with("[") {
                stack_lines.push(line);
            }
        }

        let mut stacks: Vec<Vec<char>> = vec![];

        stack_lines.reverse();

        for stack_line in stack_lines {
            let stack_line_chars: Vec<char> = stack_line.chars().collect();
            let mut index = 0;
            for i in (1..stack_line_chars.len()).step_by(4) {
                let c = stack_line_chars[i];
                if stacks.len() <= index {
                    stacks.push(vec![]);
                }
                if c != ' ' {
                    stacks[index].push(c)
                }
                index += 1;
            }
        }

        for move_line in move_lines {
            let parts: Vec<&str> = move_line.split(' ').collect();
            let amount = parts[1].parse::<usize>().unwrap();
            let from = parts[3].parse::<usize>().unwrap() - 1;
            let to = parts[5].parse::<usize>().unwrap() - 1;

            for _ in 0..amount {
                let cargo = stacks[from].pop().unwrap();
                stacks[to].push(cargo);
            }
        }

        stacks.iter().map(|stack|stack.last().or(Some(&' ')).unwrap()).collect::<String>()
    }

    fn solve_two(&self, input: &str, lines: &Vec<&str>) -> String {
        let mut stack_lines: Vec<&str> = vec![];
        let mut move_lines: Vec<&str> = vec![];

        for line in lines {
            if line.starts_with(" 1") {
                // ignore
            } else if line.starts_with("move") {
                move_lines.push(line);
            } else if line.trim().starts_with("[") {
                stack_lines.push(line);
            }
        }

        let mut stacks: Vec<Vec<char>> = vec![];

        stack_lines.reverse();

        for stack_line in stack_lines {
            let stack_line_chars: Vec<char> = stack_line.chars().collect();
            let mut index = 0;
            for i in (1..stack_line_chars.len()).step_by(4) {
                let c = stack_line_chars[i];
                if stacks.len() <= index {
                    stacks.push(vec![]);
                }
                if c != ' ' {
                    stacks[index].push(c)
                }
                index += 1;
            }
        }

        for move_line in move_lines {
            let parts: Vec<&str> = move_line.split(' ').collect();
            let amount = parts[1].parse::<usize>().unwrap();
            let from = parts[3].parse::<usize>().unwrap() - 1;
            let to = parts[5].parse::<usize>().unwrap() - 1;

            let mut popped: Vec<char> = vec![];
            for _ in 0..amount {
                let cargo = stacks[from].pop().unwrap();
                popped.push(cargo);
            }
            popped.reverse();
            stacks[to].append(&mut popped);
        }

        stacks.iter().map(|stack|stack.last().or(Some(&' ')).unwrap()).collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::Dec05;
    use crate::{solve_one, solve_two};

    #[test]
    fn solution_one() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(solve_one(&Dec05 {}, input), "CMZ");
    }

    #[test]
    fn solution_two() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(solve_two(&Dec05 {}, input), "MCD");
    }
}
