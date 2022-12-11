#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Noop,
    Add(i32),
}

impl Operation {
    pub fn cycles_to_process(&self) -> u8 {
        match self {
            Operation::Noop => 1,
            Operation::Add(_) => 2,
        }
    }
}

pub struct CPU<const N: usize> {
    cycle: usize,
    register_x: i32,
    history: [i32; N],
}

impl<const N: usize> CPU<N> {
    pub fn new() -> Self {
        CPU {
            cycle: 0,
            register_x: 1,
            history: [0; N],
        }
    }

    pub fn process_operation(&mut self, operation: &Operation) {
        match operation {
            Operation::Noop => self.step(),
            Operation::Add(x) => {
                for _ in 0..operation.cycles_to_process() {
                    self.step()
                }

                self.register_x += x;
            }
        }
    }

    pub fn step(&mut self) {
        self.history[self.cycle] = self.register_x;
        self.cycle += 1;
    }

    pub fn get_signal_strenght(&self, indices: &[usize]) -> Vec<i32> {
        let mut results: Vec<i32> = Vec::new();
        for index in indices {
            results.push(self.history[index - 1] * *index as i32)
        }

        results
    }
}

struct CRT {
    index: usize,
    drawing: [char; 240],
}

impl CRT {
    pub fn new() -> Self {
        CRT {
            index: 0,
            drawing: ['.'; 240], //vec![],
        }
    }

    pub fn render(&mut self, sprite_pos: i32) {
        let render_pos = self.index % 40;

        if render_pos == (sprite_pos - 1) as usize
            || render_pos == sprite_pos as usize
            || render_pos == (sprite_pos + 1) as usize
        {
            self.drawing[self.index] = '#';
        };
        self.index += 1;
    }

    pub fn print(self) {
        for n in 0..6 {
            let start = n * 40;
            println!("{:?}", &self.drawing[start..start + 40]);
        }
    }
}

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<Operation> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            match split.next() {
                Some("noop") => Operation::Noop,
                Some("addx") => Operation::Add(split.next().unwrap().parse::<i32>().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<Operation>>()
}

#[aoc(day10, part1)]
pub fn part1(input: &[Operation]) -> i32 {
    let mut cpu = CPU::<240>::new();
    input
        .into_iter()
        .for_each(|operation| cpu.process_operation(operation));
    cpu.get_signal_strenght(&[20, 60, 100, 140, 180, 220])
        .into_iter()
        .sum()
}

#[aoc(day10, part2)]
pub fn part2(input: &[Operation]) -> usize {
    let mut cpu = CPU::<240>::new();
    let mut crt = CRT::new();
    input
        .into_iter()
        .for_each(|operation| cpu.process_operation(operation));

    cpu.history[0..240].iter().for_each(|x| crt.render(*x));
    crt.print();
    0
}

#[test]
pub fn part1_test() {
    let got = part1(&parse_input(TEST_INPUT));
    assert_eq!(13140, got)
}

#[test]
pub fn part2_test() {
    let _ = part2(&parse_input(TEST_INPUT));
}

#[test]
fn parse_input_test() {
    let got = parse_input(TEST_INPUT);
    let want = vec![
        Operation::Add(15),
        Operation::Add(-11),
        Operation::Add(6),
        Operation::Add(-3),
        Operation::Add(5),
        Operation::Add(-1),
        Operation::Add(-8),
        Operation::Add(13),
        Operation::Add(4),
        Operation::Noop,
        Operation::Add(-1),
        Operation::Add(5),
        Operation::Add(-1),
        Operation::Add(5),
        Operation::Add(-1),
        Operation::Add(5),
        Operation::Add(-1),
        Operation::Add(5),
        Operation::Add(-1),
        Operation::Add(-35),
        Operation::Add(1),
        Operation::Add(24),
        Operation::Add(-19),
        Operation::Add(1),
        Operation::Add(16),
        Operation::Add(-11),
        Operation::Noop,
        Operation::Noop,
        Operation::Add(21),
        Operation::Add(-15),
        Operation::Noop,
        Operation::Noop,
        Operation::Add(-3),
        Operation::Add(9),
        Operation::Add(1),
        Operation::Add(-3),
        Operation::Add(8),
        Operation::Add(1),
        Operation::Add(5),
        Operation::Noop,
        Operation::Noop,
        Operation::Noop,
        Operation::Noop,
        Operation::Noop,
        Operation::Add(-36),
        Operation::Noop,
        Operation::Add(1),
        Operation::Add(7),
        Operation::Noop,
        Operation::Noop,
        Operation::Noop,
        Operation::Add(2),
        Operation::Add(6),
        Operation::Noop,
        Operation::Noop,
        Operation::Noop,
        Operation::Noop,
        Operation::Noop,
        Operation::Add(1),
        Operation::Noop,
        Operation::Noop,
        Operation::Add(7),
        Operation::Add(1),
        Operation::Noop,
        Operation::Add(-13),
        Operation::Add(13),
        Operation::Add(7),
        Operation::Noop,
        Operation::Add(1),
        Operation::Add(-33),
        Operation::Noop,
        Operation::Noop,
        Operation::Noop,
        Operation::Add(2),
        Operation::Noop,
        Operation::Noop,
        Operation::Noop,
        Operation::Add(8),
        Operation::Noop,
        Operation::Add(-1),
        Operation::Add(2),
        Operation::Add(1),
        Operation::Noop,
        Operation::Add(17),
        Operation::Add(-9),
        Operation::Add(1),
        Operation::Add(1),
        Operation::Add(-3),
        Operation::Add(11),
        Operation::Noop,
        Operation::Noop,
        Operation::Add(1),
        Operation::Noop,
        Operation::Add(1),
        Operation::Noop,
        Operation::Noop,
        Operation::Add(-13),
        Operation::Add(-19),
        Operation::Add(1),
        Operation::Add(3),
        Operation::Add(26),
        Operation::Add(-30),
        Operation::Add(12),
        Operation::Add(-1),
        Operation::Add(3),
        Operation::Add(1),
        Operation::Noop,
        Operation::Noop,
        Operation::Noop,
        Operation::Add(-9),
        Operation::Add(18),
        Operation::Add(1),
        Operation::Add(2),
        Operation::Noop,
        Operation::Noop,
        Operation::Add(9),
        Operation::Noop,
        Operation::Noop,
        Operation::Noop,
        Operation::Add(-1),
        Operation::Add(2),
        Operation::Add(-37),
        Operation::Add(1),
        Operation::Add(3),
        Operation::Noop,
        Operation::Add(15),
        Operation::Add(-21),
        Operation::Add(22),
        Operation::Add(-6),
        Operation::Add(1),
        Operation::Noop,
        Operation::Add(2),
        Operation::Add(1),
        Operation::Noop,
        Operation::Add(-10),
        Operation::Noop,
        Operation::Noop,
        Operation::Add(20),
        Operation::Add(1),
        Operation::Add(2),
        Operation::Add(2),
        Operation::Add(-6),
        Operation::Add(-11),
        Operation::Noop,
        Operation::Noop,
        Operation::Noop,
    ];
    assert_eq!(want, got)
}

#[cfg(test)]
const TEST_INPUT: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;
