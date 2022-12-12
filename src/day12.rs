use rayon::prelude::*;

type Index = usize;
type Height = u8;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Map {
    inner: Vec<u8>,
    width: usize,
    start: usize,
    end: usize,
}

impl Map {
    fn left_neighbor(&self, index: usize) -> Option<(Index, Height)> {
        if index % self.width == 0 {
            return None;
        }

        Some((index - 1, self.inner[index - 1]))
    }

    fn right_neighbor(&self, index: usize) -> Option<(Index, Height)> {
        if index % self.width == self.width - 1 {
            return None;
        }

        Some((index + 1, self.inner[index + 1]))
    }

    fn top_neighbor(&self, index: usize) -> Option<(Index, Height)> {
        if index < self.width {
            return None;
        }

        Some((index - self.width, self.inner[index - self.width]))
    }

    fn bottom_neighbor(&self, index: usize) -> Option<(Index, Height)> {
        if index + self.width >= self.inner.len() {
            return None;
        }

        Some((index + self.width, self.inner[index + self.width]))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pathfinder {
    current: usize,
    //map: &'m Map,
}

impl Pathfinder {
    pub fn sucessors(&self, map: &Map) -> Vec<Pathfinder> {
        fn resolve_succesor(
            current_height: u8,
            possible_move: Option<(Index, Height)>,
            sucessors: &mut Vec<Pathfinder>,
        ) {
            if let Some((index, height)) = possible_move {
                //println!("current_height={current_height} | height={height}");
                match current_height >= height {
                    true => sucessors.push(Pathfinder { current: index }),

                    false if (height - current_height) < 2 => {
                        sucessors.push(Pathfinder { current: index })
                    }
                    _ => (),
                }
            }
        }

        let current_height = map.inner[self.current];
        let mut sucessors = vec![];
        resolve_succesor(
            current_height,
            map.left_neighbor(self.current),
            &mut sucessors,
        );
        resolve_succesor(
            current_height,
            map.top_neighbor(self.current),
            &mut sucessors,
        );
        resolve_succesor(
            current_height,
            map.right_neighbor(self.current),
            &mut sucessors,
        );
        resolve_succesor(
            current_height,
            map.bottom_neighbor(self.current),
            &mut sucessors,
        );
        sucessors
    }
}

#[aoc_generator(day12)]
pub fn parse_input(input: &str) -> Map {
    let width = input
        .lines()
        .map(|line| line.len())
        .nth(0)
        .expect("expected line width, but got nothing");
    let mut inner = input.replace("\n", "").into_bytes();
    let start = inner
        .iter()
        .position(|b| *b == b'S')
        .expect("no starting position found");
    let end = inner
        .iter()
        .position(|b| *b == b'E')
        .expect("no finish position found");

    inner[start] = 97;
    inner[end] = 122;

    Map {
        inner,
        width,
        start,
        end,
    }
}

#[aoc(day12, part1)]
pub fn part1(input: &Map) -> i32 {
    //dbg!(input);
    let pathfinder = Pathfinder {
        current: input.start,
    };

    let path = pathfinding::prelude::bfs(
        &pathfinder,
        |p| p.sucessors(input),
        |p| input.end == p.current,
    );

    //dbg!(&path);

    let steps = path.map(|x| x.len()).unwrap_or(0);
    steps as i32 - 1
}

#[aoc(day12, part2)]
pub fn part2(input: &Map) -> usize {
    let starts = input
        .inner
        .iter()
        .enumerate()
        .filter(|(_, v)| **v == b'a')
        .map(|(index, _)| index)
        .collect::<Vec<usize>>();

    starts
        .par_iter()
        .filter_map(|start| {
            let pathfinder = Pathfinder { current: *start };
            pathfinding::prelude::bfs(
                &pathfinder,
                |p| p.sucessors(input),
                |p| input.end == p.current,
            )
        })
        .map(|path| path.len())
        .min()
        .unwrap()
        - 1
}

#[test]
fn parse_input_test() {
    parse_input(
        r#"abcccccaaaaaacccaaaccaaaaaaaacccaaaaaaccccccccccccccccccccccccccccaaaaaaaaaaaaaacacccccccccccccccccccccccccccccccaaaaaaaacccccccccccccccccccccccccccccccccccccccccccccaaaaa
    abcccccaaaaaaaacaaaaccaaaaaaccccaaaaaaccccccccccaaacccccccccccccccaaaaaaaaaaaaaaaacccccccccccccccccccccccccccccccaaaaaaaaaccccccaaaccccccccccccccccccccccccccccccccccaaaaaa
    abccccaaaaaaaaacaaaaccaaaaaaccccaaaaaaaaccccccccaaaccccccccccccccccaaaaaaaaaaaaaaccccaaaccccccccccccccccccccccccccaaaaaaaaccccacaaaccccccccccccccccaaccccccccccccccccaaaaaa"#,
    );
}

#[cfg(test)]
const TEST_INPUT: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

#[test]
pub fn part1_test() {
    let got = part1(&parse_input(TEST_INPUT));
    assert_eq!(31, got)
}

#[test]
pub fn part2_test() {
    let got = part2(&parse_input(TEST_INPUT));
    assert_eq!(29, got)
}

#[cfg(test)]
const TEST_GRID: [u8; 20] = [
    //0  1  3  4  5
    0, 1, 2, 3, 4, // 1
    5, 6, 7, 8, 9, // 2
    10, 11, 12, 13, 14, // 3
    15, 16, 17, 18, 19, // 4
];

#[test]
fn test_neighbors() {
    let map = Map {
        inner: vec![
            //0  1  3  4  5
            0, 1, 2, 3, 4, // 1
            5, 6, 7, 8, 9, // 2
            10, 11, 12, 13, 14, // 3
            15, 16, 17, 18, 19, // 4
        ],
        width: 5,
        start: 0,
        end: 0,
    };

    assert_eq!(2, map.top_neighbor(7).unwrap().1);
    assert_eq!(None, map.top_neighbor(2));

    assert_eq!(None, map.left_neighbor(5));
    assert_eq!(5, map.left_neighbor(6).unwrap().1);

    assert_eq!(6, map.right_neighbor(5).unwrap().1);
    assert_eq!(None, map.right_neighbor(9));

    assert_eq!(12, map.bottom_neighbor(7).unwrap().1);
    assert_eq!(None, map.bottom_neighbor(17));
}

#[test]
fn possible_neighbors_test() {
    // all neighbors around 7 should return
    let map = Map {
        inner: vec![
            //0  1  3  4  5
            0, 1, 1, 3, 4, // 1
            5, 1, 7, 1, 9, // 2
            10, 11, 1, 13, 14, // 3
            15, 16, 13, 18, 19, // 4
        ],
        width: 5,
        start: 0,
        end: 0,
    };

    let pathfinder = Pathfinder { current: 7 };

    let got = pathfinder.sucessors(&map);
    let want = vec![
        Pathfinder { current: 6 },
        Pathfinder { current: 2 },
        Pathfinder { current: 8 },
        Pathfinder { current: 12 },
    ];

    assert_eq!(want, got);

    // no neighbor arount 7 should return
    let map = Map {
        inner: vec![
            //0  1  3  4  5
            0, 1, 10, 3, 4, // 1
            5, 10, 7, 10, 9, // 2
            10, 11, 10, 13, 14, // 3
            15, 16, 13, 18, 19, // 4
        ],
        width: 5,
        start: 0,
        end: 0,
    };

    let pathfinder = Pathfinder { current: 7 };

    let got = pathfinder.sucessors(&map);
    let want: Vec<Pathfinder> = vec![];
    assert_eq!(want, got);

    // all neighbor around 7 should return
    let map = Map {
        inner: vec![
            //0  1  3  4  5
            0, 1, 8, 3, 4, // 1
            5, 8, 7, 8, 9, // 2
            10, 11, 8, 13, 14, // 3
            15, 16, 13, 18, 19, // 4
        ],
        width: 5,
        start: 0,
        end: 0,
    };

    let pathfinder = Pathfinder { current: 7 };

    let got = pathfinder.sucessors(&map);
    let want = vec![
        Pathfinder { current: 6 },
        Pathfinder { current: 2 },
        Pathfinder { current: 8 },
        Pathfinder { current: 12 },
    ];
    assert_eq!(want, got);

    // no neighbor around 7 should return
    let map = Map {
        inner: vec![
            //0  1  3  4  5
            0, 1, 9, 3, 4, // 1
            5, 9, 7, 9, 9, // 2
            10, 11, 9, 13, 14, // 3
            15, 16, 13, 18, 19, // 4
        ],
        width: 5,
        start: 0,
        end: 0,
    };

    let pathfinder = Pathfinder { current: 7 };

    let got = pathfinder.sucessors(&map);
    let want: Vec<Pathfinder> = vec![];
    assert_eq!(want, got);

    dbg!(got);
}
