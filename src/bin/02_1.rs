use aoc2021::day_02::{read_input, solve_part1};
use std::time::Instant;

fn main() {
    println!("Puzzle 02 - Dive! - Part 1");

    let start = Instant::now();
    let input = read_input("inputs/02.txt");
    let solution = solve_part1(input);
    let time = start.elapsed();

    println!("Time: {:?}", time);
    println!("Solution is: {}", solution);
}
