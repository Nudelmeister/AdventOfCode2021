use aoc2021::day_01::{read_input, solve_part1};
use std::time::Instant;

fn main() {
    println!("Puzzle 01 - Sonar Sweep - Part 1");

    let input = read_input("inputs/01.txt");

    let start = Instant::now();
    let solution = solve_part1(input);
    let time = start.elapsed();

    println!("Time: {:?}", time);
    println!("Solution is: {}", solution);
}
