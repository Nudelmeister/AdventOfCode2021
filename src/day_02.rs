use std::fs;

pub fn solve_part1(input: &[Command]) -> i32 {
    let mut position = 0;
    let mut depth = 0;

    for command in input {
        match command.direction {
            Direction::Up => depth -= command.value,
            Direction::Down => depth += command.value,
            Direction::Forward => position += command.value,
        }
    }

    position * depth
}

pub fn solve_part2(input: &[Command]) -> i32 {
    let mut position = 0;
    let mut aim = 0;
    let mut depth = 0;

    for command in input {
        match command.direction {
            Direction::Up => aim -= command.value,
            Direction::Down => aim += command.value,
            Direction::Forward => {
                position += command.value;
                depth += aim * command.value;
            }
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
    value: i32,
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
            value: distance,
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Command> {
    input.lines().map(|l| Command::parse(l)).collect()
}
pub fn parse_input_from_file(path: &str) -> Vec<Command> {
    parse_input(&fs::read_to_string(path).unwrap())
}
