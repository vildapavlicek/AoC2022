#[cfg(test)]
const OFFSET: u8 = 48;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Grid {
    width: usize,
    data: Vec<u8>,
}

// to get row: index / width
// position on a row: index % width
impl Grid {
    fn is_on_edge(&self, index: usize) -> bool {
        let index = index + 1;
        // right side
        index % self.width == 0
            // left side
            || index % self.width == 1
            // first row
            || index < self.width
            // last row
            || index > self.data.len() - self.width
    }

    pub fn count_visible(&self) -> u32 {
        let mut counter = 0;

        for (index, tree) in self.data.iter().enumerate() {
            if self.is_on_edge(index) {
                counter += 1;
                continue;
            }

            if self.is_visible_from_left(index, *tree)
                || self.is_visible_from_top(index, *tree)
                || self.is_visible_from_right(index, *tree)
                || self.is_visible_from_bottom(index, *tree)
            {
                counter += 1;
            }
        }

        counter
    }

    pub fn count_scenic_score_for_trees(&self) -> Vec<u32> {
        self.data
            .iter()
            .enumerate()
            .map(|(index, value)| {
                self.scenic_score_left(index, *value)
                    * self.scenic_score_top(index, *value)
                    * self.scenic_score_right(index, *value)
                    * self.scenic_score_bottom(index, *value)
            })
            .collect::<Vec<u32>>()
    }

    // 8, 1 | 23, 9
    fn is_visible_from_left(&self, index: usize, value: u8) -> bool {
        // need to resolve start of the row and then take all until the index
        let skip_to = (index / self.width) * self.width; // ((8 / 5) * 5)= (1 * 5) = 5 | ((23 / 5) * 5) = (4 * 5) = 20
        let to_take = self.width - (self.width - (index % self.width)); // 5 - (5 - (8 % 5)) = 5 - (5 - 3) =  5 - 2 = 3 | 5 - (5 - ( 23 % 5)) = 5 - ( 5 - 3 )) = 5 - 2 = 3
        !self
            .data
            .iter() // 3037325512653323354935390
            .skip(skip_to) // 30373 25512653323354935390 | 30373255126533233549 35390
            .take(to_take) // 30373 255 12.65332.33549.35390 | 30373255126533233549 353 90
            .find(|v| **v >= value) // 2 >= 1 | 3 >= 9, 5 >= 9, 3 >= 9
            .is_some()
    }

    fn scenic_score_left(&self, index: usize, value: u8) -> u32 {
        if index % self.width == 1 {
            return 0;
        }

        let skip_to = (index / self.width) * self.width;
        let to_take = self.width - (self.width - (index % self.width));

        let mut counter = 0;

        for tree in self
            .data
            .iter() // 3037325512653323354935390
            .skip(skip_to) // 30373 25512653323354935390 | 30373255126533233549 35390
            .take(to_take)
        // 30373 255 12.65332.33549.35390 | 30373255126533233549 353 90
        {
            if *tree >= value {
                break;
            }

            counter += 1;
        }
        counter
    }

    // 16, 3 | 23, 9
    fn is_visible_from_top(&self, index: usize, value: u8) -> bool {
        let to_skip = index % self.width;

        !self
            .data
            .iter() // 30373.25512.65332.33549.35390
            .take(index - 1) // 30373.25512.65332.3 3549.35390
            .skip(to_skip) // 0373.25512.65332.3
            .step_by(self.width) // 0373.25512.65332.3
            .find(|v| **v >= value)
            .is_some()
    }

    fn scenic_score_top(&self, index: usize, value: u8) -> u32 {
        if index < self.width {
            return 0;
        }

        let to_skip = index % self.width;
        let mut counter = 0;

        for tree in self
            .data
            .iter() // 30373.25512.65332.33549.35390
            .take(index - 1) // 30373.25512.65332.3 3549.35390
            .skip(to_skip) // 0373.25512.65332.3
            .step_by(self.width)
        {
            if *tree >= value {
                break;
            }

            counter += 1;
        }
        counter
    }

    // 16, 3 | 7, 5
    fn is_visible_from_right(&self, index: usize, value: u8) -> bool {
        let to_take = self.width - (index % self.width) - 1; // (5 - (16 % 5)) - 1 = (5 - 1) - 1 = 4 - 1 = 3 | (5 - (7 % 5)) -1 = (5 - 2) - 1 = 3 - 1 = 2
        !self
            .data
            .iter() // 3037325512653323 3 549 35390 | 30373.25 5 12. 65332.33549.35390
            .skip(index + 1)
            .take(to_take) // 549 | 12
            .find(|v| **v >= value) // 5 >= 3, 4 >= 3, 9 >= 3 | 1 >= 5, 2 >= 5
            .is_some()
    }

    fn scenic_score_right(&self, index: usize, value: u8) -> u32 {
        if index % self.width == 0 {
            return 0;
        }

        let to_take = self.width - (index % self.width) - 1; // (5 - (16 % 5)) - 1 = (5 - 1) - 1 = 4 - 1 = 3 | (5 - (7 % 5)) -1 = (5 - 2) - 1 = 3 - 1 = 2
        let mut counter = 0;

        for tree in self
            .data
            .iter() // 3037325512653323 3 549 35390 | 30373.25 5 12. 65332.33549.35390
            .skip(index + 1)
            .take(to_take)
        // 549 | 12
        {
            if *tree >= value {
                break;
            }

            counter += 1;
        }

        counter
    }

    // 3, 7 | 10, 6
    fn is_visible_from_bottom(&self, index: usize, value: u8) -> bool {
        let to_take = self.data.len() - index;

        !self
            .data
            .iter() // 30373.25512.65332.33549.35390
            .skip(index) //  303 73.25512.65332.33549.35390
            .take(to_take) // 73.25512.65332.33549.35390
            .step_by(self.width) // 73.255 12.653 32.335 49.353 90
            .skip(1) // 12.653 32.335 49.353 90
            .find(|v| **v >= value)
            .is_some()
    }

    fn scenic_score_bottom(&self, index: usize, value: u8) -> u32 {
        if index > self.data.len() - self.width {
            return 0;
        }

        let to_take = self.data.len() - index;
        let mut counter = 0;

        for tree in self
            .data
            .iter() // 30373.25512.65332.33549.35390
            .skip(index) //  303 73.25512.65332.33549.35390
            .take(to_take) // 73.25512.65332.33549.35390
            .step_by(self.width) // 73.255 12.653 32.335 49.353 90
            .skip(1)
        {
            if *tree >= value {
                break;
            }

            counter += 1
        }

        counter
    }
}

#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> Grid {
    let width = input.lines().next().unwrap().chars().count();
    let data = input.replace("\n", "").into_bytes();
    Grid { width, data }
}

#[aoc(day8, part1)]
pub fn part1(input: &Grid) -> u32 {
    input.count_visible()
}

#[aoc(day8, part2)]
pub fn part2(input: &Grid) -> u32 {
    let mut scores = input.count_scenic_score_for_trees();
    scores.sort();
    *scores.last().unwrap()
}

#[cfg(test)]
const TEST_INPUT: &str = r#"30373
25512
65332
33549
35390"#;

#[test]
pub fn parse_input_test() {
    let got = parse_input("123\n456");
    let want = Grid {
        width: 3,
        data: vec![49, 50, 51, 52, 53, 54],
    };

    assert_eq!(want, got)
}

#[test]
pub fn part1_test() {
    let grid = parse_input(TEST_INPUT);
    let got = grid.count_visible();
    assert_eq!(21, got)
}

#[test]
pub fn part2_test() {
    let grid = parse_input(TEST_INPUT);
    let got = part2(&grid);
    assert_eq!(8, got)
}

#[test]
pub fn is_on_edge_test() {
    let grid = parse_input(TEST_INPUT);
    let count = grid
        .data
        .iter()
        .enumerate()
        .filter(|(index, _)| grid.is_on_edge(*index))
        .count();
    let want = 16;

    assert_eq!(want, count);
    assert_eq!(true, grid.is_on_edge(4));
    assert_eq!(false, grid.is_on_edge(6));
    assert_eq!(false, grid.is_on_edge(7));
}

#[test]
pub fn is_visible_from_right_test() {
    let grid = parse_input(TEST_INPUT);
    assert_eq!(true, grid.is_visible_from_right(7, 5 + OFFSET));
    assert_eq!(false, grid.is_visible_from_right(16, 3 + OFFSET));
}

#[test]
pub fn is_visible_from_left_test() {
    let grid = parse_input(TEST_INPUT);
    assert_eq!(false, grid.is_visible_from_left(8, 1 + OFFSET));
    assert_eq!(true, grid.is_visible_from_left(23, 9 + OFFSET));
}

// 30373
// 25512
// 65332
// 33549
// 35390

#[test]
pub fn is_visible_from_top_test() {
    let grid = parse_input(TEST_INPUT);
    assert_eq!(false, grid.is_visible_from_top(16, 3 + OFFSET));
    assert_eq!(true, grid.is_visible_from_top(23, 9 + OFFSET));
}

#[test]
pub fn is_visible_from_bottom_test() {
    let grid = parse_input(TEST_INPUT);
    assert_eq!(false, grid.is_visible_from_bottom(3, 7 + OFFSET));
    assert_eq!(true, grid.is_visible_from_bottom(10, 6 + OFFSET));
}
