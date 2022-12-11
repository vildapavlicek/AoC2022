use std::{cell::RefCell, collections::VecDeque, str::Lines};

type MonkeyID = usize;

pub struct Monkey {
    items: RefCell<VecDeque<u32>>,
    operation: Box<dyn Fn(u32) -> u32>,
    test: Box<dyn Fn(usize) -> usize>,
    inspect_count: RefCell<u32>,
    div: usize,
}

impl Monkey {
    pub fn throw(&self, stress_reduction: bool) -> Option<(MonkeyID, u32)> {
        let Some(item) = self.items.borrow_mut().pop_front() else {return None;};

        self.inspect_count.replace_with(|&mut old| old + 1);

        let new_item = if stress_reduction {
            (self.operation)(item) / 3
        } else {
            (self.operation)(item)
        };

        let monkey_id = (self.test)(new_item as usize);
        println!("item={item}, new_item={new_item}, monkey_id={monkey_id}");
        Some((monkey_id, new_item))
    }

    pub fn catch(&self, item: u32) {
        self.items.borrow_mut().push_back(item);
    }
}

fn parse_starting_items(line: &str) -> VecDeque<u32> {
    let mut ns = vec![];
    line.split(":")
        .skip(1)
        .map(|n| {
            n.split(",")
                .map(|n| n.trim().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .for_each(|v| v.iter().for_each(|n| ns.push(*n)));

    ns.into()
}

fn parse_operation(input: &str) -> Box<dyn Fn(u32) -> u32> {
    let mut split = input.trim().split(" ").skip(4);
    let operator = split.next().expect("no operator for operation");
    let value = split
        .next()
        .expect("missing value for operation (number or 'old')");

    if operator.trim() == "+" {
        return match value {
            "old" => Box::new(|x| x + x),
            _ => {
                let value = value.parse::<u32>().unwrap();
                Box::new(move |x| x + value)
            }
        };
    }

    match value {
        "old" => Box::new(|x| x.overflowing_mul(x).0),
        _ => {
            let value = value.parse::<u32>().unwrap();
            Box::new(move |x| x.overflowing_mul(value).0)
        }
    }
}

fn parse_test(lines: &mut Lines) -> (Box<dyn Fn(usize) -> usize>, usize) {
    fn get_last(line: &str) -> usize {
        line.split(" ").last().unwrap().parse::<usize>().unwrap()
    }

    let divisible_num = get_last(lines.next().expect("missing 'divisible by' line"));
    let true_cond = get_last(
        lines
            .next()
            .expect("missing true condition description line"),
    );
    let false_cond = get_last(
        lines
            .next()
            .expect("missing false condition description line"),
    );

    (
        Box::new(move |x| {
            if x % divisible_num == 0 {
                true_cond
            } else {
                false_cond
            }
        }),
        divisible_num,
    )
}

#[aoc_generator(day11)]
pub fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|monkey_description| {
            let mut lines = monkey_description.lines();
            lines.next(); // monkey number, we can skip
            let items = parse_starting_items(lines.next().unwrap());
            let operation = parse_operation(lines.next().unwrap());
            let (test, div) = parse_test(&mut lines);

            Monkey {
                items: RefCell::new(items),
                operation,
                test,
                inspect_count: RefCell::new(0),
                div,
            }
        })
        .collect::<Vec<Monkey>>()
}

#[aoc(day11, part1)]
pub fn part1(input: &[Monkey]) -> u32 {
    for _ in 0..20 {
        input.iter().for_each(|monkey| {
            while let Some((monkey_id, item)) = monkey.throw(true) {
                input[monkey_id].catch(item);
            }
        })
    }

    let mut inspection_counts = input
        .iter()
        .map(|monkey| *monkey.inspect_count.borrow())
        .collect::<Vec<u32>>();

    inspection_counts.sort();
    let last = inspection_counts.len();
    inspection_counts[last - 1] * inspection_counts[last - 2]
}

#[aoc(day11, part2)]
pub fn part2(input: &[Monkey]) -> u32 {
    let modulo = input.iter().map(|monkey| monkey.div).product::<usize>();
    println!("modulo to destress is : {modulo}");
    for i in 0..1000 {
        input.iter().enumerate().for_each(|(index, monkey)| {
            println!("ROUND: {}, monkey{index}: ", i + 1);
            while let Some((monkey_id, item)) = monkey.throw(false) {
                input[monkey_id].catch(item % modulo as u32);
            }
        });

        if
        /* (i + 1) % 1000 == 0 */
        i + 1 == 20 || i + 1 == 1000 || i + 1 == 1 {
            println!("--- ROUND {}", i + 1);
            for (i2, monkey) in input.iter().enumerate() {
                println!(
                    "monkey={i2}, inspected items={}",
                    monkey.inspect_count.borrow()
                );
            }
        }
    }

    let mut inspection_counts = input
        .iter()
        .map(|monkey| *monkey.inspect_count.borrow())
        .collect::<Vec<u32>>();

    inspection_counts.sort();
    let last = inspection_counts.len();
    inspection_counts[last - 1] * inspection_counts[last - 2]
}

#[test]
pub fn part1_test() {
    let got = part1(&parse_input(TEST_INPUT));
    assert_eq!(10605, got);
}

#[test]
pub fn part2_test() {
    let got = part2(&parse_input(TEST_INPUT));
    assert_eq!(2713310158, got)
}

#[cfg(test)]
const TEST_INPUT: &str = r#"Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
  If true: throw to monkey 2
  If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
  If true: throw to monkey 2
  If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
  If true: throw to monkey 1
  If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
  If true: throw to monkey 0
  If false: throw to monkey 1"#;
