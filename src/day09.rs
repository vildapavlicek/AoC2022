use glam::IVec2;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Left(u8),
    Up(u8),
    Right(u8),
    Down(u8),
}

impl From<(&str, &str)> for Instruction {
    fn from((instruction, count): (&str, &str)) -> Self {
        match instruction {
            "L" => Instruction::Left(count.parse::<u8>().unwrap()),
            "U" => Instruction::Up(count.parse::<u8>().unwrap()),
            "R" => Instruction::Right(count.parse::<u8>().unwrap()),
            "D" => Instruction::Down(count.parse::<u8>().unwrap()),
            _ => unreachable!("unexpected char in input: '{instruction}'"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Tail {
    pos: IVec2,
    visited: HashSet<IVec2>,
}

impl Tail {
    pub fn new() -> Tail {
        let mut visited = HashSet::default();
        visited.insert(IVec2::ZERO);
        Tail {
            pos: IVec2::ZERO,
            visited,
        }
    }

    pub fn at_pos(pos: IVec2) -> Tail {
        let mut visited = HashSet::default();
        visited.insert(pos);
        Tail { pos, visited }
    }

    pub fn r#move(&mut self, parent_position: IVec2) {
        if is_touching(self.pos, parent_position) {
            return;
        }

        if parent_position.x > self.pos.x {
            self.pos.x += 1
        }

        if parent_position.x < self.pos.x {
            self.pos.x -= 1;
        }

        if parent_position.y > self.pos.y {
            self.pos.y += 1
        }

        if parent_position.y < self.pos.y {
            self.pos.y -= 1
        }

        self.visited.insert(self.pos);
    }
}

#[derive(Debug, Default)]
pub struct Snake {
    head_pos: IVec2,
    head_prev_pos: IVec2,
    tail: Vec<Tail>,
}

impl Snake {
    pub fn with_single_part() -> Self {
        Snake {
            head_pos: IVec2::ZERO,
            head_prev_pos: IVec2::ZERO,
            tail: vec![Tail::new()],
        }
    }

    pub fn new(tail_size: usize) -> Self {
        let mut v = vec![];
        for _ in 0..tail_size {
            v.push(Tail::new())
        }
        Snake {
            head_pos: IVec2::ZERO,
            head_prev_pos: IVec2::ZERO,
            tail: v,
        }
    }

    pub fn move_snake(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Left(steps) => {
                for _ in 0..(*steps) {
                    self.move_left();
                    self.move_tail();
                }
            }
            Instruction::Up(steps) => {
                for _ in 0..(*steps) {
                    self.move_up();
                    self.move_tail();
                }
            }
            Instruction::Right(steps) => {
                for _ in 0..(*steps) {
                    self.move_right();
                    self.move_tail();
                }
            }
            Instruction::Down(steps) => {
                for _ in 0..(*steps) {
                    self.move_down();
                    self.move_tail();
                }
            }
        }
    }

    fn move_right(&mut self) {
        self.head_prev_pos = self.head_pos;
        self.head_pos.x += 1
    }

    fn move_left(&mut self) {
        self.head_prev_pos = self.head_pos;
        self.head_pos.x -= 1
    }

    fn move_up(&mut self) {
        self.head_prev_pos = self.head_pos;
        self.head_pos.y += 1
    }

    fn move_down(&mut self) {
        self.head_prev_pos = self.head_pos;
        self.head_pos.y -= 1
    }

    #[cfg(test)]
    fn is_head_touching_tail(&self) -> bool {
        is_touching(self.head_pos, self.tail[0].pos)
    }

    fn move_tail(&mut self) {
        let mut parent_pos = self.head_pos;
        for tail in self.tail.iter_mut() {
            tail.r#move(parent_pos);
            parent_pos = tail.pos;
        }
    }
}

// 1, 2
fn is_touching(a: IVec2, b: IVec2) -> bool {
    let diff = a - b;
    diff.x.abs() < 2 && diff.y.abs() < 2
}

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(Instruction::from)
        .collect::<Vec<Instruction>>()
}

#[aoc(day9, part1)]
pub fn part1(input: &[Instruction]) -> usize {
    let mut snake = Snake::with_single_part();
    input
        .iter()
        .for_each(|instruction| snake.move_snake(instruction));
    snake.tail[0].visited.iter().count()
}

#[aoc(day9, part2)]
pub fn part2(input: &[Instruction]) -> usize {
    let mut snake = Snake::new(9);
    input
        .iter()
        .for_each(|instruction| snake.move_snake(instruction));

    snake.tail.last().unwrap().visited.iter().count()
}

#[cfg(test)]
const TEST_INPUT: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

#[cfg(test)]
const TEST_INPUT_2: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

#[test]
fn parse_input_test() {
    let got = parse_input(TEST_INPUT);
    let want = vec![
        Instruction::Right(4),
        Instruction::Up(4),
        Instruction::Left(3),
        Instruction::Down(1),
        Instruction::Right(4),
        Instruction::Down(1),
        Instruction::Left(5),
        Instruction::Right(2),
    ];
    assert_eq!(want, got)
}

#[test]
pub fn part1_test() {
    let got = part1(&parse_input(TEST_INPUT));
    assert_eq!(13, got)
}

#[test]
pub fn part2_test() {
    let got = part2(&parse_input(TEST_INPUT_2));
    assert_eq!(36, got)
}

#[test]
fn is_head_touching_tail_test() {
    assert_eq!(
        true,
        Snake {
            head_pos: IVec2::ZERO,
            head_prev_pos: IVec2::ZERO,
            tail: vec![Tail::at_pos(IVec2::new(0, 0))],
        }
        .is_head_touching_tail()
    );
    assert_eq!(
        true,
        Snake {
            head_pos: IVec2::ZERO,
            head_prev_pos: IVec2::ZERO,
            tail: vec![Tail::at_pos(IVec2::new(-1, 0))],
        }
        .is_head_touching_tail()
    );
    assert_eq!(
        true,
        Snake {
            head_pos: IVec2::ZERO,
            head_prev_pos: IVec2::ZERO,
            tail: vec![Tail::at_pos(IVec2::new(-1, -1))],
        }
        .is_head_touching_tail()
    );
    assert_eq!(
        true,
        Snake {
            head_pos: IVec2::ZERO,
            head_prev_pos: IVec2::ZERO,
            tail: vec![Tail::at_pos(IVec2::new(0, -1))],
        }
        .is_head_touching_tail()
    );
    assert_eq!(
        true,
        Snake {
            head_pos: IVec2::ZERO,
            head_prev_pos: IVec2::ZERO,
            tail: vec![Tail::at_pos(IVec2::new(1, -1))],
        }
        .is_head_touching_tail()
    );
    assert_eq!(
        true,
        Snake {
            head_pos: IVec2::ZERO,
            head_prev_pos: IVec2::ZERO,
            tail: vec![Tail::at_pos(IVec2::new(1, 0))],
        }
        .is_head_touching_tail()
    );
    assert_eq!(
        true,
        Snake {
            head_pos: IVec2::ZERO,
            head_prev_pos: IVec2::ZERO,
            tail: vec![Tail::at_pos(IVec2::new(1, 1))],
        }
        .is_head_touching_tail()
    );
    assert_eq!(
        true,
        Snake {
            head_pos: IVec2::ZERO,
            head_prev_pos: IVec2::ZERO,
            tail: vec![Tail::at_pos(IVec2::new(0, 1))],
        }
        .is_head_touching_tail()
    );
    assert_eq!(
        true,
        Snake {
            head_pos: IVec2::ZERO,
            head_prev_pos: IVec2::ZERO,
            tail: vec![Tail::at_pos(IVec2::new(-1, 1))],
        }
        .is_head_touching_tail()
    );

    //
    assert_eq!(
        false,
        Snake {
            head_pos: IVec2::ZERO,
            head_prev_pos: IVec2::ZERO,
            tail: vec![Tail::at_pos(IVec2::new(0, 2))],
        }
        .is_head_touching_tail()
    );
    assert_eq!(
        false,
        Snake {
            head_pos: IVec2::ZERO,
            head_prev_pos: IVec2::ZERO,
            tail: vec![Tail::at_pos(IVec2::new(0, -2))],
        }
        .is_head_touching_tail()
    );
    assert_eq!(
        false,
        Snake {
            head_pos: IVec2::ZERO,
            head_prev_pos: IVec2::ZERO,
            tail: vec![Tail::at_pos(IVec2::new(-2, -2))],
        }
        .is_head_touching_tail()
    );
    assert_eq!(
        false,
        Snake {
            head_pos: IVec2::ZERO,
            head_prev_pos: IVec2::ZERO,
            tail: vec![Tail::at_pos(IVec2::new(-2, 0))],
        }
        .is_head_touching_tail()
    );
}

#[test]
fn is_touching_test() {
    assert_eq!(false, is_touching(IVec2::new(2, 0), IVec2::ZERO));
    assert_eq!(false, is_touching(IVec2::ZERO, IVec2::new(2, 0)));
    assert_eq!(false, is_touching(IVec2::ZERO, IVec2::new(2, 1)));
    assert_eq!(false, is_touching(IVec2::ZERO, IVec2::new(2, 2)));
    assert_eq!(false, is_touching(IVec2::ZERO, IVec2::new(1, 2)));
    assert_eq!(false, is_touching(IVec2::ZERO, IVec2::new(0, 2)));
    assert_eq!(false, is_touching(IVec2::ZERO, IVec2::new(-1, 2)));
    assert_eq!(false, is_touching(IVec2::ZERO, IVec2::new(-2, 2)));
    assert_eq!(false, is_touching(IVec2::ZERO, IVec2::new(-2, 1)));
    assert_eq!(false, is_touching(IVec2::ZERO, IVec2::new(-2, 0)));
    assert_eq!(false, is_touching(IVec2::ZERO, IVec2::new(-2, -1)));
    assert_eq!(false, is_touching(IVec2::ZERO, IVec2::new(-2, -2)));
    assert_eq!(false, is_touching(IVec2::ZERO, IVec2::new(-1, -2)));
    assert_eq!(false, is_touching(IVec2::ZERO, IVec2::new(0, -2)));
    assert_eq!(false, is_touching(IVec2::ZERO, IVec2::new(1, -2)));
    assert_eq!(false, is_touching(IVec2::ZERO, IVec2::new(2, -2)));
    assert_eq!(false, is_touching(IVec2::ZERO, IVec2::new(2, -1)));
    assert_eq!(false, is_touching(IVec2::ZERO, IVec2::new(2, 0)));
    //
    assert_eq!(true, is_touching(IVec2::ZERO, IVec2::ZERO));
    assert_eq!(true, is_touching(IVec2::ZERO, IVec2::new(1, 0)));
    assert_eq!(true, is_touching(IVec2::ZERO, IVec2::new(1, 1)));
    assert_eq!(true, is_touching(IVec2::ZERO, IVec2::new(0, 1)));
    assert_eq!(true, is_touching(IVec2::ZERO, IVec2::new(-1, 1)));
    assert_eq!(true, is_touching(IVec2::ZERO, IVec2::new(-1, 0)));
    assert_eq!(true, is_touching(IVec2::ZERO, IVec2::new(-1, -1)));
    assert_eq!(true, is_touching(IVec2::ZERO, IVec2::new(0, -1)));
    assert_eq!(true, is_touching(IVec2::ZERO, IVec2::new(1, -1)));
}
