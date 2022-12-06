use itertools::Itertools;
use regex::Regex;
use std::ops::RangeInclusive;

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    let regex = Regex::new("[0-9]*").unwrap();
    input
        .lines()
        .map(|line| {
            regex
                .find_iter(line)
                .map(|r#match| r#match.as_str().parse::<u32>().unwrap())
                .tuples::<(u32, u32)>()
                .map(|(start, end)| RangeInclusive::new(start, end))
                .collect_tuple::<(RangeInclusive<u32>, RangeInclusive<u32>)>()
                .unwrap()
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
    input
        .into_iter()
        .filter(|(range1, range2)| {
            (range1.contains(range2.start()) && range1.contains(range2.end()))
                || (range2.contains(range1.start()) && range2.contains(range1.end()))
        })
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
    input
        .into_iter()
        .filter(|(range1, range2)| range1.clone().any(|x| range2.contains(&x)))
        .count()
}

#[cfg(test)]
const TEST_INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

#[test]
pub fn generator_test() {
    let got = generator(TEST_INPUT);
    let want = vec![
        (2..=4, 6..=8),
        (2..=3, 4..=5),
        (5..=7, 7..=9),
        (2..=8, 3..=7),
        (6..=6, 4..=6),
        (2..=6, 4..=8),
    ];

    assert_eq!(got, want)
}

#[test]
pub fn part1_test() {
    let got = part1(&generator(TEST_INPUT));
    let want = 2;

    assert_eq!(got, want)
}

#[test]
pub fn part2_test() {
    let got = part2(&generator(TEST_INPUT));
    let want = 4;
    assert_eq!(got, want)
}
