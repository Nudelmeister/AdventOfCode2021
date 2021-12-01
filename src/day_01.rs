use std::{
    fs::File,
    io::{BufRead, BufReader},
    mem,
};

pub fn solve_part1(input: impl Iterator<Item = i32>) -> i32 {
    let mut increase_count = 0;
    let mut previous = i32::MAX;
    for next in input {
        if previous < next {
            increase_count += 1;
        }
        previous = next;
    }
    increase_count
}

pub fn solve_part2(mut input: impl Iterator<Item = i32>) -> i32 {
    let one = input.next().unwrap();
    let two = input.next().unwrap();
    let three = input.next().unwrap();

    let mut windows = [vec![one, two, three], vec![two, three], vec![three]];

    let mut increase_count = 0;

    for sample in input {
        windows[1].push(sample);
        windows[2].push(sample);

        if has_increased(&windows[0], &windows[1]) {
            increase_count += 1;
        }

        windows.rotate_left(1);
        windows[2].clear();
        windows[2].push(sample);
    }

    increase_count
}

fn has_increased(prev: &[i32], next: &[i32]) -> bool {
    assert_eq!(prev.len(), 3);
    assert_eq!(next.len(), 3);
    prev[0] + prev[1] + prev[2] < next[0] + next[1] + next[2]
}

pub fn read_input(path: &str) -> impl Iterator<Item = i32> {
    BufReader::new(File::open(path).unwrap_or_else(|e| panic!("Error opening file: {}", e)))
        .lines()
        .map(|l| l.expect("Error reading a line from input"))
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.parse().expect("Error parsing line as number"))
}
