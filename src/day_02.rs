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

#[cfg(test)]
mod test {
    const EXAMPLE: &str = "forward 5\n\
                           down 5\n\
                           forward 8\n\
                           up 3\n\
                           down 8\n\
                           forward 2";

    #[test]
    fn part_1_example() {
        let input = super::parse_input(EXAMPLE);
        let solution = super::solve_part1(&input);
        assert_eq!(150, solution);
    }

    #[test]
    fn part_1_puzzle() {
        let input = super::parse_input_from_file("inputs/02.txt");
        let solution = super::solve_part1(&input);
        assert_eq!(1660158, solution);
    }

    #[test]
    fn part_2_example() {
        let input = super::parse_input(EXAMPLE);
        let solution = super::solve_part2(&input);
        assert_eq!(900, solution);
    }

    #[test]
    #[ignore = "not solved yet"]
    fn part_2_puzzle() {
        let input = super::parse_input_from_file("inputs/02.txt");
        let solution = super::solve_part2(&input);
        assert_eq!(1660158, solution);
    }
}
