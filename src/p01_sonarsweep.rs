use std::{
    fs::File,
    io::{BufRead, BufReader},
    mem,
};

pub fn solve_loop(input: impl Iterator<Item = i32>) -> i32 {
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

//pub fn solve_loop(mut input: impl Iterator<Item = i32>) -> i32 {
//    let mut increase_count = 0;
//    let Some(mut previous) = input.next() else { return 0 };
//    for next in input {
//        if previous < next {
//            increase_count += 1;
//        }
//        previous = next;
//    }
//    increase_count
//}

pub fn solve_iter(input: impl Iterator<Item = i32>) -> i32 {
    input
        .scan(i32::MAX, |p, n| Some((mem::replace(p, n), n)))
        .filter(|(prev, next)| prev < next)
        .count() as i32
}

pub fn read_input(path: &str) -> impl Iterator<Item = i32> {
    BufReader::new(File::open(path).unwrap_or_else(|e| panic!("Error opening file: {}", e)))
        .lines()
        .map(|l| l.expect("Error reading a line from input"))
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.parse().expect("Error parsing line as number"))
}
