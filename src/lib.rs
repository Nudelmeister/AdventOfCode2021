pub mod day_01;
pub mod day_02;

pub mod day_03 {
    use std::fs;

    pub fn solve_part1<const INPUT_WIDTH: usize>(input: &[u32]) -> i32 {
        let mut bit_counters = [0; INPUT_WIDTH];

        for &bits in input {
            for (idx, counter) in bit_counters.iter_mut().enumerate() {
                *counter += (bits & (1 << idx) != 0) as i32;
            }
        }

        let mut gamma_rate = 0;
        let mut epsilon_rate = 0;
        let num_samples = input.len() as i32;

        for (idx, counter) in bit_counters.into_iter().enumerate() {
            let bit_common = counter > num_samples / 2;
            gamma_rate |= (bit_common as i32) << idx;
            epsilon_rate |= (!bit_common as i32) << idx;
        }

        gamma_rate * epsilon_rate
    }

    pub fn solve_part2<const INPUT_WIDTH: usize>(input: &[u32]) -> i32 {
        todo!()
    }

    fn parse_input_line(line: &str) -> u32 {
        u32::from_str_radix(line, 2).unwrap()
    }

    pub fn parse_input(input: &str) -> Vec<u32> {
        input.lines().map(parse_input_line).collect()
    }

    pub fn parse_input_from_file(path: &str) -> Vec<u32> {
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
            let solution = super::solve_part1::<5>(&input);
            assert_eq!(198, solution);
        }

        #[test]
        fn part_1_puzzle() {
            let input = super::parse_input_from_file("inputs/03.txt");
            let solution = super::solve_part1::<12>(&input);
            assert_eq!(3882564, solution);
        }

        #[test]
        fn part_2_example() {
            let input = super::parse_input(EXAMPLE);
            let solution = super::solve_part2::<5>(&input);
            assert_eq!(0, solution);
        }

        #[test]
        fn part_2_puzzle() {
            let input = super::parse_input_from_file("inputs/03.txt");
            let solution = super::solve_part2::<12>(&input);
            assert_eq!(0, solution);
        }
    }
}
