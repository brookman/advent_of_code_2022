use itertools::Itertools;

use crate::common::read_strings;

mod common;

fn main() {
    println!("\nDecember 1st, 2022");
    dec_01_one("data/in_01_1.txt");
    dec_01_two("data/in_01_1.txt");
}

fn dec_01_one(filename: &str) {
    println!("--- Part One ---");

    let sums = get_sum_of_blocks(filename);
    let result = sums.iter().max().unwrap();

    println!("{:?}", result);
}

fn dec_01_two(filename: &str) {
    println!("--- Part Two ---");

    let sums = get_sum_of_blocks(filename);
    let result: u32 = sums.into_iter().sorted_by(|a, b| b.cmp(a)).take(3).sum();

    println!("{:?}", result);
}

fn get_sum_of_blocks(filename: &str) -> Vec<u32> {
    let strings = read_strings(filename);

    let mut sums: Vec<u32> = vec![0];
    for string in strings {
        if string.is_empty() {
            sums.push(0);
        } else {
            let index = sums.len() - 1;
            sums[index] += string.parse::<u32>().unwrap();
        }
    }
    sums
}
