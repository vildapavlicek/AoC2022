pub enum CraneModel {
    NineThousand,
    NineThousandAndOne,
}

type Crate = char;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Stacks(Vec<Vec<Crate>>);

impl Stacks {
    pub fn push(&mut self, i: usize, c: Crate) {
        match self.0.get_mut(i) {
            Some(stack) if !c.is_whitespace() => stack.push(c),
            None if !c.is_whitespace() => self.0.push(vec![c]),
            _ => (),
        }
    }

    pub fn process_instruction(&mut self, instruction: &Instruction, crane_model: CraneModel) {
        let mut to_move = self.crates_to_move(instruction);

        match crane_model {
            CraneModel::NineThousandAndOne if instruction.count > 1 => {
                to_move = to_move.into_iter().rev().collect()
            }
            _ => (),
        }

        match self.0.get_mut(instruction.to - 1) {
            Some(stack) => stack.append(&mut to_move),
            None => (),
        }
    }

    pub fn get_top_crates(&self) -> Vec<Crate> {
        let mut result = vec![];
        self.0.iter().for_each(|stack| match stack.last() {
            Some(c) => result.push(*c),
            None => (),
        });

        result
    }

    fn crates_to_move(&mut self, instruction: &Instruction) -> Vec<Crate> {
        let mut to_move = vec![];

        let stack = self.0.get_mut(instruction.from - 1).unwrap();
        for _ in 0..instruction.count {
            to_move.push(stack.pop().expect("trying to pop from empty stack"))
        }

        to_move
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    count: u32,
    from: usize,
    to: usize,
}

impl Instruction {
    pub fn new(count: u32, from: usize, to: usize) -> Self {
        Instruction { count, from, to }
    }
}

#[aoc_generator(day5)]
pub fn generate(input: &str) -> (Stacks, Vec<Instruction>) {
    let split = input.split("\n\n").collect::<Vec<&str>>();
    let (raw_stack, raw_instructions) = (split[0], split[1]);

    let mut stacks = Stacks::default();

    raw_stack
        .lines()
        .rev()
        .skip(1)
        .map(|line| line.chars().skip(1).step_by(4).collect::<Vec<char>>())
        .for_each(|row| row.iter().enumerate().for_each(|(i, c)| stacks.push(i, *c)));

    let instructions = raw_instructions
        .lines()
        .map(|line| {
            let raw_instruction = line.split(' ').collect::<Vec<&str>>();
            Instruction {
                count: raw_instruction[1].parse::<u32>().unwrap(),
                from: raw_instruction[3].parse::<usize>().unwrap(),
                to: raw_instruction[5].parse::<usize>().unwrap(),
            }
        })
        .collect();

    (stacks, instructions)
}

#[aoc(day5, part1)]
pub fn part1((stack, instructions): &(Stacks, Vec<Instruction>)) -> String {
    let mut stack = stack.clone();
    instructions
        .into_iter()
        .for_each(|instruction| stack.process_instruction(instruction, CraneModel::NineThousand));

    stack.get_top_crates().into_iter().collect::<String>()
}

#[aoc(day5, part2)]
pub fn part2((stack, instructions): &(Stacks, Vec<Instruction>)) -> String {
    let mut stack = stack.clone();
    instructions.into_iter().for_each(|instruction| {
        stack.process_instruction(instruction, CraneModel::NineThousandAndOne)
    });

    stack.get_top_crates().into_iter().collect::<String>()
}

#[cfg(test)]
const TEST_INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

#[test]
pub fn generator_test() {
    let got = generate(TEST_INPUT);
    let want = (
        Stacks(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]),
        vec![
            Instruction::new(1, 2, 1),
            Instruction::new(3, 1, 3),
            Instruction::new(2, 2, 1),
            Instruction::new(1, 1, 2),
        ],
    );

    assert_eq!(got, want)
}

#[test]
pub fn part1_test() {
    let got = part1(&generate(TEST_INPUT));
    let want = "CMZ".to_string();
    assert_eq!(got, want)
}

#[test]
pub fn part2_test() {
    let got = part2(&generate(TEST_INPUT));
    let want = "MCD".to_string();
    assert_eq!(got, want)
}
