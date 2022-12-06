#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    count_callories(input).into_iter().max().unwrap_or(0)
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut callories = count_callories(input);

    callories.sort();
    let x = callories.into_iter().rev().take(3).collect::<Vec<usize>>();

    x.into_iter().sum()
}

fn count_callories(input: &str) -> Vec<usize> {
    let mut callories = vec![];
    let mut counter = 0usize;

    input.lines().for_each(|line| match line.is_empty() {
        true => {
            callories.push(counter);
            counter = 0;
        }
        false => counter += line.parse::<usize>().unwrap_or(0),
    });
    callories.push(counter);
    callories
}

#[cfg(test)]
const TEST_INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

#[test]
fn part1_test() {
    let want = 24000;
    let got = part1(TEST_INPUT);

    assert_eq!(want, got)
}

#[test]
fn part2_test() {
    let want = 45000;
    let got = part2(TEST_INPUT);
    assert_eq!(want, got)
}
