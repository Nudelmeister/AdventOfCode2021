use std::time::Instant;

use aoc2021::p01_sonarsweep::{read_input, solve_loop};

fn main() {
    println!("Puzzle 01 - Sonar Sweep");

    let input = read_input("inputs/01-sonarsweep.txt");

    let start = Instant::now();
    let solution = solve_loop(input);
    let time = start.elapsed();

    println!("Time: {:?}", time);
    println!("Solution is: {}", solution);
}
