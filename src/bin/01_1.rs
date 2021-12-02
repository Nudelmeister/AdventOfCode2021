use aoc2021::day_01::{parse_input_from_file, solve_part1};
use std::time::Instant;

fn main() {
    println!("Puzzle 01 - Sonar Sweep - Part 1");

    let start = Instant::now();
    let input = parse_input_from_file("inputs/01.txt");
    let solution = solve_part1(&input);
    let time = start.elapsed();

    println!("Time: {:?}", time);
    println!("Solution is: {}", solution);
}
