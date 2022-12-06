use itertools;
use itertools::Itertools;

use crate::day02::RoundResult::{Defeat, Draw, Win};
use crate::day02::Shape::{Paper, Rock, Scissors};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn resolve_battle(&self, rhs: Shape) -> RoundResult {
        match (self, rhs) {
            (Paper, Rock) | (Scissors, Paper) | (Rock, Scissors) => Win,
            (Paper, Scissors) | (Rock, Paper) | (Scissors, Rock) => Defeat,
            _ if self.eq(&rhs) => Draw,
            _ => unreachable!("should not reach this point"),
        }
    }

    fn looses_against(&self) -> Shape {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn wins_against(&self) -> Shape {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    pub fn resolve_shape_for_outcome(&self, wanted_outcome: RoundResult) -> Shape {
        match wanted_outcome {
            Draw => *self,
            Win => self.looses_against(),
            Defeat => self.wins_against(),
        }
    }
}

impl Into<usize> for Shape {
    fn into(self) -> usize {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl TryFrom<char> for Shape {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Rock),
            'B' | 'Y' => Ok(Paper),
            'C' | 'Z' => Ok(Scissors),
            _ => Err(format!("Invalid char provided '{}'", value)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RoundResult {
    Win,
    Draw,
    Defeat,
}

impl Into<usize> for RoundResult {
    fn into(self) -> usize {
        match self {
            Win => 6,
            Draw => 3,
            Defeat => 0,
        }
    }
}

impl TryFrom<char> for RoundResult {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Defeat),
            'Y' => Ok(Draw),
            'Z' => Ok(Win),
            _ => Err(format!("Invalid char provided '{}'", value)),
        }
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let input = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| Shape::try_from(c).ok())
                .collect_tuple::<(Shape, Shape)>()
                .unwrap()
        })
        .collect::<Vec<(Shape, Shape)>>();

    count_score(&input).into_iter().sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    let mut opponent_moves = vec![];
    let mut should_end_with = vec![];

    input.lines().for_each(|line| {
        line.chars()
            .tuples::<(char, char, char)>()
            .for_each(|(shape, _, x)| {
                opponent_moves.push(Shape::try_from(shape).unwrap());
                should_end_with.push(RoundResult::try_from(x).unwrap());
            });
    });

    if opponent_moves.len() != should_end_with.len() {
        panic!("has different number of opponent moves and number of how should battle end")
    }

    let v = opponent_moves
        .into_iter()
        .zip(should_end_with)
        .map(|(shape, should_end)| {
            let my_shape = shape.resolve_shape_for_outcome(should_end);

            (shape, my_shape)
        })
        .collect::<Vec<(Shape, Shape)>>();

    count_score(&v).into_iter().sum()
}

fn count_score(input: &[(Shape, Shape)]) -> Vec<usize> {
    input
        .into_iter()
        .map(|(opponent, me)| {
            <RoundResult as Into<usize>>::into(me.resolve_battle(*opponent))
                + <Shape as Into<usize>>::into(*me)
        })
        .collect::<Vec<usize>>()
}

#[test]
fn count_score_test() {
    let input = &[(Rock, Paper), (Paper, Rock), (Scissors, Scissors)];
    let want = 15;
    let got: usize = count_score(input).into_iter().sum();
    assert_eq!(got, want)
}
