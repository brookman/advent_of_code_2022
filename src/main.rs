use common::Solution;
use dec_01::Dec01;
use dec_02::Dec02;
use dec_03::Dec03;

use crate::common::read_strings;

mod common;
mod dec_01;
mod dec_02;
mod dec_03;

fn main() {
    solve(Dec01 {}, "01");
    solve(Dec02 {}, "02");
    solve(Dec03 {}, "03");
}

fn solve<T: Solution>(solution: T, prefix: &str) {
    println!("\nDecember {}, 2022", prefix);
    println!("--- Part One ---");
    let lines1 = read_strings(&format!("data/in_{}_1.txt", prefix));
    println!("{}", solution.solve_one(&lines1));
    println!("--- Part Two ---");
    let lines2 = read_strings(&format!("data/in_{}_2.txt", prefix));
    println!("{}", solution.solve_two(&lines2));
}
