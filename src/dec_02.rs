use crate::common::Solution;

pub struct Dec02 {}

#[derive(PartialEq)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl Solution for Dec02 {
    fn solve_one(&self, lines: &Vec<&str>) -> String {
        let mut score = 0u32;
        for line in lines {
            let columns = line.split(" ").collect::<Vec<&str>>();

            let enemy = match columns[0] {
                "A" => Choice::Rock,
                "B" => Choice::Paper,
                "C" => Choice::Scissors,
                _ => panic!(),
            };

            let me = match columns[1] {
                "X" => Choice::Rock,
                "Y" => Choice::Paper,
                "Z" => Choice::Scissors,
                _ => panic!(),
            };

            let outcome = calc_outcome(&me, &enemy);

            score += me as u32;
            score += outcome as u32;
        }

        score.to_string()
    }

    fn solve_two(&self, lines: &Vec<&str>) -> String {
        let mut score = 0u32;
        for line in lines {
            let columns = line.split(" ").collect::<Vec<&str>>();

            let enemy = match columns[0] {
                "A" => Choice::Rock,
                "B" => Choice::Paper,
                "C" => Choice::Scissors,
                _ => panic!(),
            };

            let outcome = match columns[1] {
                "X" => Outcome::Loss,
                "Y" => Outcome::Draw,
                "Z" => Outcome::Win,
                _ => panic!(),
            };

            let me = calc_choice(&enemy, &outcome);
            
            score += me as u32;
            score += outcome as u32;
        }

        score.to_string()
    }
}

fn calc_outcome(me: &Choice, enemy: &Choice) -> Outcome {
    match me {
        Choice::Rock => match enemy {
            Choice::Rock => Outcome::Draw,
            Choice::Paper => Outcome::Loss,
            Choice::Scissors => Outcome::Win,
        },
        Choice::Paper => match enemy {
            Choice::Rock => Outcome::Win,
            Choice::Paper => Outcome::Draw,
            Choice::Scissors => Outcome::Loss,
        },
        Choice::Scissors => match enemy {
            Choice::Rock => Outcome::Loss,
            Choice::Paper => Outcome::Win,
            Choice::Scissors => Outcome::Draw,
        },
    }
}

fn calc_choice(enemy: &Choice, outcome: &Outcome) -> Choice {
    match outcome {
        Outcome::Loss => match enemy {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        },
        Outcome::Draw => match enemy {
            Choice::Rock => Choice::Rock,
            Choice::Paper => Choice::Paper,
            Choice::Scissors => Choice::Scissors,
        },
        Outcome::Win => match enemy {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        },
    }
}
