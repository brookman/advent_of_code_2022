use crate::common::read_numbers;

mod common;

fn main() {
    println!("\nDecember 1st, 2022");
    dec_01_one("data/in_01_1.txt");
    dec_01_two("data/in_01_2.txt");
}

fn dec_01_one(filename: &str) {
    println!("--- Part One ---");

    let numbers = read_numbers(filename);
    for number in numbers {
        println!("{:?}", number);
    }
}

fn dec_01_two(filename: &str) {
    println!("--- Part Two ---");

    let numbers = read_numbers(filename);
    for number in numbers {
        println!("{:?}", number);
    }
}
