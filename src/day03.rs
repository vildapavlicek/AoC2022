use itertools;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
pub struct Compartment(String);

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<(Compartment, Compartment)> {
    input
        .lines()
        .map(|line| {
            let split = line.split_at(line.len() / 2);
            (
                Compartment(split.0.to_owned()),
                Compartment(split.1.to_owned()),
            )
        })
        .collect::<Vec<(Compartment, Compartment)>>()
}

#[aoc(day3, part1)]
pub fn part1(input: &[(Compartment, Compartment)]) -> u32 {
    find_common_character(input)
        .into_iter()
        .fold(0, |acc, c| acc + to_score(c))
}

#[aoc(day3, part2)]
pub fn part2(input: &[(Compartment, Compartment)]) -> u32 {
    input
        .into_iter()
        .map(|(c1, c2)| c1.0.chars().join(&c2.0).chars().collect::<HashSet<char>>())
        .tuples::<(HashSet<char>, HashSet<char>, HashSet<char>)>()
        .map(|(set1, set2, set3)| {
            set1.into_iter()
                .find(|c| set2.contains(c) && set3.contains(c))
                .unwrap()
        })
        .fold(0, |acc, c| acc + to_score(c))
}

fn find_common_character(input: &[(Compartment, Compartment)]) -> Vec<char> {
    input
        .into_iter()
        .map(|data| {
            let set = data.1 .0.chars().collect::<HashSet<char>>();
            data.0 .0.chars().find(|c| set.contains(c)).unwrap()
        })
        .collect()
}

fn to_score(c: char) -> u32 {
    match c.is_uppercase() {
        true => c as u32 - 38,
        false => c as u32 - 96,
    }
}

#[cfg(test)]
const TEST_INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

#[test]
pub fn generator_test() {
    let got = generator(TEST_INPUT);
    let want = vec![
        (
            Compartment("vJrwpWtwJgWr".into()),
            Compartment("hcsFMMfFFhFp".into()),
        ),
        (
            Compartment("jqHRNqRjqzjGDLGL".into()),
            Compartment("rsFMfFZSrLrFZsSL".into()),
        ),
        (
            Compartment("PmmdzqPrV".into()),
            Compartment("vPwwTWBwg".into()),
        ),
        (
            Compartment("wMqvLMZHhHMvwLH".into()),
            Compartment("jbvcjnnSBnvTQFn".into()),
        ),
        (
            Compartment("ttgJtRGJ".into()),
            Compartment("QctTZtZT".into()),
        ),
        (
            Compartment("CrZsJsPPZsGz".into()),
            Compartment("wwsLwLmpwMDw".into()),
        ),
    ];
    assert_eq!(got, want);
}

#[test]
fn to_score_test() {
    let input = vec!['p', 'L', 'P', 'v', 't', 's'];
    let want = vec![16, 38, 42, 22, 20, 19];

    let got = input.into_iter().map(to_score).collect::<Vec<u32>>();
    assert_eq!(got, want)
}

#[test]
fn find_common_character_test() {
    let got = find_common_character(&generator(TEST_INPUT));
    let want = vec!['p', 'L', 'P', 'v', 't', 's'];
    assert_eq!(got, want)
}

#[test]
pub fn part1_test() {
    let want = 157;
    let got = part1(&generator(TEST_INPUT));
    assert_eq!(got, want)
}

#[test]
pub fn part2_test() {
    let want = 70;
    let got = part2(&generator(TEST_INPUT));
    assert_eq!(got, want)
}
