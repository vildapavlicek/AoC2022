#[aoc(day6, part1)]
pub fn part1(input: &[u8]) -> usize {
    let pos = input.iter().enumerate().skip(3).position(|(index, b)| {
        let (x, y, z) = (input[index - 3], input[index - 2], input[index - 1]);
        (*b != x && *b != y && *b != z) && (x != y && x != z) && (y != z)
    });

    pos.unwrap_or(0) + 4
}

#[derive(Debug, Eq, PartialEq, Default)]
struct CharsCounters([u8; 26]);

impl CharsCounters {
    pub fn increment(&mut self, c: char) {
        self.0[c as usize - 97] += 1;
    }

    pub fn decrement(&mut self, c: char) {
        if self.0[c as usize - 97] == 0 {
            panic!("trying to decrement char '{c}' count below 0")
        }

        self.0[c as usize - 97] -= 1;
    }

    pub fn count_duplicates(&self) -> usize {
        self.0.iter().filter(|c| **c > 1).count()
    }
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let mut counter = CharsCounters::default();
    input.chars().take(14).for_each(|c| counter.increment(c));

    let pos = input.chars().enumerate().skip(14).position(|(index, c)| {
        counter.increment(c);
        counter.decrement(input.as_bytes()[index - 14] as char);
        counter.count_duplicates() < 1
    });

    pos.unwrap_or(0) + 15
}

#[cfg(test)]
const TEST_INPUT: [&str; 5] = [
    r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#,
    r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#,
    r#"nppdvjthqldpwncqszvftbrmjlhg"#,
    r#"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"#,
    r#"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"#,
];

#[test]
pub fn part1_test() {
    let got = TEST_INPUT
        .into_iter()
        .map(|input| input.as_bytes())
        .map(part1)
        .collect::<Vec<usize>>();
    let want = vec![7usize, 5, 6, 10, 11];
    assert_eq!(got, want)
}

#[test]
pub fn part2_test() {
    let got = TEST_INPUT.into_iter().map(part2).collect::<Vec<usize>>();
    let want = vec![19, 23, 23, 29, 26];

    assert_eq!(got, want)
}

#[test]
pub fn increment_test() {
    let mut counter = CharsCounters::default();
    counter.increment('a');
    counter.increment('a');
    counter.increment('b');
    counter.increment('y');
    counter.increment('z');
    counter.increment('z');

    let mut want = [0u8; 26];
    want[0] = 2;
    want[1] = 1;

    want[24] = 1;
    want[25] = 2;

    assert_eq!(counter.0, want)
}

#[test]
pub fn decrement_test() {
    let mut counter = CharsCounters::default();
    counter.increment('a');
    counter.increment('a');
    counter.decrement('a');
    counter.increment('z');
    counter.increment('z');
    counter.decrement('z');

    let mut want = [0u8; 26];
    want[0] = 1;
    want[25] = 1;

    assert_eq!(counter.0, want)
}

#[test]
pub fn count_duplicates_test() {
    let mut counter = CharsCounters::default();
    counter.increment('a');
    counter.increment('a');
    counter.increment('b');
    counter.increment('y');
    counter.increment('z');
    counter.increment('z');
    counter.increment('z');

    let want = 2;

    assert_eq!(counter.count_duplicates(), want)
}
