use std::vec;

use crate::{common::Solution, dec_05_parsing::PuzzleInput};

pub struct Dec05 {}

impl Solution for Dec05 {
    fn solve_one(&self, input: &str, _lines: &Vec<&str>) -> String {
        let (_, mut puzzle_input) = PuzzleInput::parse(input).unwrap();

        for a_move in puzzle_input.moves {
            for _ in 0..a_move.amount {
                let cargo = puzzle_input.stacks[a_move.from_index].pop().unwrap();
                puzzle_input.stacks[a_move.to_index].push(cargo);
            }
        }

        puzzle_input
            .stacks
            .iter()
            .map(|stack| stack.last().or(Some(&' ')).unwrap())
            .collect::<String>()
    }

    fn solve_two(&self, input: &str, _lines: &Vec<&str>) -> String {
        let (_, mut puzzle_input) = PuzzleInput::parse(input).unwrap();

        for a_move in puzzle_input.moves {
            let mut popped: Vec<char> = vec![];
            for _ in 0..a_move.amount {
                let cargo = puzzle_input.stacks[a_move.from_index].pop().unwrap();
                popped.push(cargo);
            }
            popped.reverse();
            puzzle_input.stacks[a_move.to_index].append(&mut popped);
        }

        puzzle_input
            .stacks
            .iter()
            .map(|stack| stack.last().or(Some(&' ')).unwrap())
            .collect::<String>()
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
