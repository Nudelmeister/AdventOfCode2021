pub mod day_01;
pub mod day_02;

pub mod day_03 {
    use std::fs;

    pub fn solve_part1(input: &[Vec<bool>]) -> i32 {
        let mut bit_counters = vec![0; input[0].len()];

        for bits in input {
            assert_eq!(bit_counters.len(), bits.len());
            for (idx, &bit) in bits.iter().enumerate() {
                bit_counters[idx] += bit as i32;
            }
        }

        let mut gamma_rate = 0;
        let mut epsilon_rate = 0;
        let num_bits = bit_counters.len();
        let num_samples = input.len() as i32;
        for (left_idx, counter) in bit_counters.into_iter().enumerate() {
            let bit_common = counter > num_samples / 2;
            let idx = num_bits - left_idx - 1;
            gamma_rate |= (bit_common as i32) << idx;
            epsilon_rate |= (!bit_common as i32) << idx;
        }

        gamma_rate * epsilon_rate
    }

    pub fn solve_part2(input: &[Vec<bool>]) -> i32 {
        todo!()
    }

    fn parse_input_line(line: &str) -> Vec<bool> {
        line.chars()
            .map(|c| match c {
                '1' => true,
                '0' => false,
                x => panic!("Invalid character: `{:?}`", x),
            })
            .collect()
    }

    pub fn parse_input(input: &str) -> Vec<Vec<bool>> {
        input.lines().map(parse_input_line).collect()
    }

    pub fn parse_input_from_file(path: &str) -> Vec<Vec<bool>> {
        parse_input(&fs::read_to_string(path).unwrap())
    }

    #[cfg(test)]
    mod test {
        const EXAMPLE: &str = "00100\n\
                               11110\n\
                               10110\n\
                               10111\n\
                               10101\n\
                               01111\n\
                               00111\n\
                               11100\n\
                               10000\n\
                               11001\n\
                               00010\n\
                               01010";

        #[test]
        fn part_1_example() {
            let input = super::parse_input(EXAMPLE);
            let solution = super::solve_part1(&input);
            assert_eq!(198, solution);
        }

        #[test]
        fn part_1_puzzle() {
            let input = super::parse_input_from_file("inputs/03.txt");
            let solution = super::solve_part1(&input);
            assert_eq!(3882564, solution);
        }

        #[test]
        fn part_2_example() {
            let input = super::parse_input(EXAMPLE);
            let solution = super::solve_part2(&input);
            assert_eq!(900, solution);
        }

        #[test]
        fn part_2_puzzle() {
            let input = super::parse_input_from_file("inputs/03.txt");
            let solution = super::solve_part2(&input);
            assert_eq!(1604592846, solution);
        }
    }
}
