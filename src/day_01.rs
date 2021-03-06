use std::fs;

pub fn solve_part1(input: &[i32]) -> i32 {
    let mut increase_count = 0;
    let mut previous = i32::MAX;
    for &next in input {
        if previous < next {
            increase_count += 1;
        }
        previous = next;
    }
    increase_count
}

pub fn solve_part2(input: &[i32]) -> i32 {
    assert!(input.len() > 3);
    let mut windows = [
        input[0..3].to_vec(),
        input[1..3].to_vec(),
        input[2..3].to_vec(),
    ];

    let mut increase_count = 0;

    for &sample in &input[3..] {
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

pub fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}
pub fn parse_input_from_file(path: &str) -> Vec<i32> {
    parse_input(&fs::read_to_string(path).unwrap())
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = "199\n\
                           200\n\
                           208\n\
                           210\n\
                           200\n\
                           207\n\
                           240\n\
                           269\n\
                           260\n\
                           263";
    #[test]
    fn part_1_example() {
        let input = super::parse_input(EXAMPLE);
        let solution = super::solve_part1(&input);
        assert_eq!(7, solution);
    }

    #[test]
    fn part_1_puzzle() {
        let input = super::parse_input_from_file("inputs/01.txt");
        let solution = super::solve_part1(&input);
        assert_eq!(1462, solution);
    }

    #[test]
    fn part_2_example() {
        let input = super::parse_input(EXAMPLE);
        let solution = super::solve_part2(&input);
        assert_eq!(5, solution);
    }

    #[test]
    fn part_2_puzzle() {
        let input = super::parse_input_from_file("inputs/01.txt");
        let solution = super::solve_part2(&input);
        assert_eq!(1497, solution);
    }
}
