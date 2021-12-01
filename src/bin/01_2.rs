use aoc2021::day_01::{read_input, solve_part2};
use std::time::Instant;

fn main() {
    println!("Puzzle 01 - Sonar Sweep - Part 2");

    let start = Instant::now();
    let input = read_input("inputs/01.txt");
    let solution = solve_part2(input);
    let time = start.elapsed();

    println!("Time: {:?}", time);
    println!("Solution is: {}", solution);
}
