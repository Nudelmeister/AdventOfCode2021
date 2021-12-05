pub mod day_01;
pub mod day_02;
pub mod day_03;

pub mod day_04 {
    use std::array;

    pub struct BingoBoard {
        /// [row][col]
        numbers: [[i32; 5]; 5],
        rows: [bool; 5],
        cols: [bool; 5],
    }

    impl BingoBoard {
        fn from_line_iter<'a>(mut input: impl Iterator<Item = &'a str>) -> Option<Self> {
            let mut this = Self {
                numbers: [[0; 5]; 5],
                rows: [false; 5],
                cols: [false; 5],
            };

            for row in 0..5 {
                let mut nums = input.next()?.split_whitespace().map(|l| l.parse().unwrap());
                for col in 0..5 {
                    this.numbers[row][col] = nums.next().unwrap();
                }
            }
            Some(this)
        }
    }

    pub fn parse_input(input: &str) -> (Vec<i32>, Vec<BingoBoard>) {
        let mut lines = input.lines().filter(|l| !l.is_empty());

        let called_numbers = lines
            .next()
            .unwrap()
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect();

            let boards = 

        (called_numbers, todo!())
    }
}
