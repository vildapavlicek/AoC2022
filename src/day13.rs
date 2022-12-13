use core::panic;
use std::{cmp::Ordering, iter::Peekable, slice::Iter};

const BYTE_OFFSET: u8 = 48;
const LINE_FEED: u8 = 10;

#[derive(Debug)]
pub enum CompareResult {
    Smaller,
    Equal,
    Bigger,
    InvalidComparison,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Value(u8),
    Array(Vec<Self>),
}

impl Type {
    pub fn parse_value((tens, ones): (u8, u8)) -> Self {
        let n = ((tens - BYTE_OFFSET) * 10) + (ones - BYTE_OFFSET);
        Self::Value(n)
    }

    pub fn parse_array(iter: &mut Peekable<Iter<'_, u8>>) -> Self {
        let mut v = vec![];
        while let Some(c) = iter.next() {
            match *c {
                b',' | b' ' => (),
                b']' | LINE_FEED => return Self::Array(v),
                b'[' => v.push(Self::parse_array(iter)),
                _ => v.push(Self::parse_value(bytes_to_ones_and_tens(
                    *c,
                    **iter.peek().unwrap(),
                ))),
            }
        }
        unreachable!("found unclosed array!")
    }

    pub fn unwrap(self) -> Vec<Self> {
        match self {
            Self::Array(v) => v,
            n => vec![n],
        }
    }

    pub fn to_vec(&self) -> Self {
        match self {
            Self::Array(_) => self.clone(),
            Self::Value(v) => Self::Array(vec![Self::Value(*v)]),
        }
    }

    fn compare_values(&self, rhs: &Type) -> CompareResult {
        let (Self::Value(a), Self::Value(b)) = (self, rhs) else {panic!("can compare only values")};

        match a.cmp(b) {
            Ordering::Equal => CompareResult::Equal,
            Ordering::Greater => CompareResult::Bigger,
            Ordering::Less => CompareResult::Smaller,
        }
    }

    fn compare_lists(&self, rhs: &Type) -> CompareResult {
        let (Self::Array(left_vec), Self::Array(right_vec)) = (self, rhs) else {
            panic!("passed values are not lists");
        };

        let len = left_vec.len().max(right_vec.len());

        for index in 0..len {
            let left = left_vec.get(index);
            let right = right_vec.get(index);

            if left.is_none() && right.is_some() {
                return CompareResult::Smaller;
            }

            if left.is_some() && right.is_none() {
                return CompareResult::Bigger;
            }

            if left.is_none() && right.is_none() {
                return CompareResult::Equal;
            }

            let left = left.unwrap();
            let right = right.unwrap();

            match (left, right) {
                (Self::Value(_), Self::Value(_)) => match left.compare_values(right) {
                    CompareResult::Equal => continue,
                    o => {
                        return o;
                    }
                },
                (Self::Array(_), Self::Array(_)) => match Self::compare_lists(left, right) {
                    CompareResult::Equal => continue,
                    o => {
                        return o;
                    }
                },
                (Self::Value(_), Self::Array(_)) => {
                    match Self::compare_lists(&left.to_vec(), right) {
                        CompareResult::Equal => continue,
                        o => {
                            return o;
                        }
                    }
                }

                (Self::Array(_), Self::Value(_)) => {
                    match Self::compare_lists(left, &right.to_vec()) {
                        CompareResult::Equal => continue,
                        o => {
                            return o;
                        }
                    }
                }
            }
        }

        CompareResult::Equal
    }

    pub fn get_first_number(&self, depth: &mut u8) -> u8 {
        match self {
            Self::Array(v) => {
                if v.is_empty() {
                    return 0;
                }
                *depth += 1;
                v[0].get_first_number(depth)
            }
            Self::Value(v) => *v,
        }
    }
}

fn bytes_to_ones_and_tens(first: u8, second: u8) -> (u8, u8) {
    if second >= 48 && second <= 57 {
        return (first, second);
    }

    (b'0', first)
}

impl From<&str> for Type {
    fn from(line: &str) -> Self {
        let mut iter = line.as_bytes().iter().peekable();
        let mut vec = vec![];

        while let Some(c) = iter.next() {
            match c {
                b'[' => {
                    return Self::parse_array(&mut iter);
                }
                b',' | b' ' => (),
                b']' | &LINE_FEED | 3 => {
                    return Self::Array(vec);
                }
                v => vec.push(Self::parse_value(bytes_to_ones_and_tens(
                    *v,
                    **iter.peek().unwrap_or(&&0),
                ))),
            }
        }

        Self::Array(vec)
    }
}

#[aoc_generator(day13)]
fn parse_input(input: &str) -> Vec<(Type, Type)> {
    input
        .split("\n\n")
        .map(|split| {
            let (first, second) = split.split_once("\n").unwrap();
            (Type::from(first), Type::from(second))
        })
        .collect::<Vec<(Type, Type)>>()
}

#[aoc(day13, part1)]
pub fn part1(input: &[(Type, Type)]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(index, pair)| (index + 1, pair.0.compare_lists(&pair.1)))
        .filter_map(|(index, ordering)| match ordering {
            CompareResult::Smaller => Some(index),
            _ => None,
        })
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &[(Type, Type)]) -> usize {
    let (mut data, mut data2): (Vec<Type>, Vec<Type>) = input.to_vec().into_iter().unzip();

    data.append(&mut data2);

    let two = Type::Array(vec![Type::Array(vec![Type::Value(2)])]);
    let six = Type::Array(vec![Type::Array(vec![Type::Value(6)])]);

    data.push(two.clone());
    data.push(six.clone());

    data.sort_by(|a, b| match a.compare_lists(b) {
        CompareResult::Bigger => Ordering::Greater,
        CompareResult::Smaller => Ordering::Less,
        CompareResult::Equal => Ordering::Equal,
        _ => unreachable!(),
    });

    let index_two = data.iter().position(|t| t == &two);
    let index_six = data.iter().position(|t| t == &six);

    (index_two.unwrap() + 1) * (index_six.unwrap() + 1)
}

#[cfg(test)]
const TEST_INPUT: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

#[test]
fn part1_test() {
    let got = part1(&parse_input(TEST_INPUT));
    assert_eq!(13, got)
}

#[test]
fn part2_test() {
    let got = part2(&parse_input(TEST_INPUT));
    assert_eq!(140, got)
}

#[test]
fn bytes_to_ones_and_tens_tes() {
    let got = bytes_to_ones_and_tens(b'1', b',');
    assert_eq!((b'0', b'1'), got);

    let got = bytes_to_ones_and_tens(b'1', b'0');
    assert_eq!((b'1', b'0'), got);

    let got = bytes_to_ones_and_tens(b'9', b']');
    assert_eq!((b'0', b'9'), got);

    let got = bytes_to_ones_and_tens(b'5', LINE_FEED);
    assert_eq!((b'0', b'5'), got);
}

#[test]
fn parse_array_test() {
    let mut iter = "1, 2, 3]".as_bytes().iter().peekable();
    let got = Type::parse_array(&mut iter);
    assert_eq!(
        Type::Array(vec![Type::Value(1), Type::Value(2), Type::Value(3),]),
        got
    );
}

#[test]
fn from_str_for_type_test() {
    let s = "[1, 2, 3]";
    let got = Type::from(s);
    assert_eq!(
        Type::Array(vec![Type::Value(1), Type::Value(2), Type::Value(3),]),
        got
    );

    let s = "[[1],[2,3,4]]";
    let got = Type::from(s);
    assert_eq!(
        Type::Array(vec![
            Type::Array(vec!(Type::Value(1))),
            Type::Array(vec![Type::Value(2), Type::Value(3), Type::Value(4)])
        ]),
        got
    );

    let s = "[[1], [2, [3, 4, 5], 6]]";
    let got = Type::from(s);
    assert_eq!(
        Type::Array(vec![
            Type::Array(vec!(Type::Value(1))),
            Type::Array(vec![
                Type::Value(2),
                Type::Array(vec![Type::Value(3), Type::Value(4), Type::Value(5)]),
                Type::Value(6)
            ])
        ]),
        got
    );

    let s = "[[7,[]]]";
    let got = Type::from(s);
    assert_eq!(
        Type::Array(vec![Type::Array(vec![Type::Value(7), Type::Array(vec![])])]),
        got
    );
}
