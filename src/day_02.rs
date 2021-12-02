use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve_part1(input: impl Iterator<Item = Command>) -> i32 {
    let mut position = 0;
    let mut depth = 0;

    for command in input {
        match command.direction {
            Direction::Up => depth -= command.distance,
            Direction::Down => depth += command.distance,
            Direction::Forward => position += command.distance,
        }
    }

    position * depth
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Forward,
}

impl Direction {
    fn parse(s: &str) -> Self {
        match s {
            "up" => Self::Up,
            "down" => Self::Down,
            "forward" => Self::Forward,
            other => panic!("{} is not a valid direction", other),
        }
    }
}

#[derive(Clone)]
pub struct Command {
    direction: Direction,
    distance: i32,
}

impl Command {
    fn parse(s: &str) -> Self {
        let mut words = s.split_whitespace();
        let direction = Direction::parse(words.next().expect("Missing direction"));
        let distance = words
            .next()
            .expect("Missing distane")
            .parse()
            .expect("Error parsing distance");

        Self {
            direction,
            distance,
        }
    }
}

pub fn read_input(path: &str) -> impl Iterator<Item = Command> {
    BufReader::new(File::open(path).unwrap_or_else(|e| panic!("Error opening file: {}", e)))
        .lines()
        .map(|l| l.expect("Error reading a line from input"))
        .filter(|l| !l.trim().is_empty())
        .map(|l| Command::parse(&l))
}
