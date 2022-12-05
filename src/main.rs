use std::{fs, time::Instant};

use common::Solution;
use dec_01::Dec01;
use dec_02::Dec02;
use dec_03::Dec03;
use dec_04::Dec04;
use dec_05::Dec05;
use dec_06::Dec06;

mod common;
mod dec_01;
mod dec_02;
mod dec_03;
mod dec_04;
mod dec_05;
mod dec_05_parsing;
mod dec_06;

fn main() {
    solve(Dec01 {}, "01");
    solve(Dec02 {}, "02");
    solve(Dec03 {}, "03");
    solve(Dec04 {}, "04");
    solve(Dec05 {}, "05");
    solve(Dec06 {}, "06");
}

fn solve<T: Solution>(solution: T, prefix: &str) {
    println!("\nDecember {}, 2022", prefix);

    println!("--- Part One ---");
    let input = fs::read_to_string(&format!("data/in_{}_1.txt", prefix)).unwrap();
    solve_one(&solution, input.as_str());
    println!("--- Part Two ---");
    let input = fs::read_to_string(&format!("data/in_{}_2.txt", prefix)).unwrap();
    solve_two(&solution, input.as_str());
}

pub fn solve_one<T: Solution>(solution: &T, input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let start = Instant::now();
    let solution = solution.solve_one(input, &lines);
    let elapsed = start.elapsed();
    println!("{}     in {:?}", solution, elapsed);
    solution
}

pub fn solve_two<T: Solution>(solution: &T, input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let start = Instant::now();
    let solution = solution.solve_two(input, &lines);
    let elapsed = start.elapsed();
    println!("{}     in {:?}", solution, elapsed);
    solution
}
