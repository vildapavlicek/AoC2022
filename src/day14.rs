use glam::IVec2;

struct Cave {
    inner: Vec<[u8; 600]>,
}

impl Cave {
    pub fn new(stones: &[Vec<IVec2>], depth: usize) -> Self {
        let mut inner = Vec::with_capacity(50);

        for _ in 0..=(depth + 1) {
            inner.push([46u8; 600])
        }

        for stone_coords in stones {
            let mut coords_iter = stone_coords.iter().peekable();

            while let Some(start) = coords_iter.next() {
                let Some(end) = coords_iter.peek() else {continue;};
                //println!("-----------------------------------------------------------------------------------------------------------------------------------------");
                add_stones(&mut inner, *start, **end);

                /* inner.iter().for_each(|line| {
                    line.iter()
                        .skip(430)
                        .map(|b| char::from(*b))
                        .for_each(|c| print!("{c}"));
                    println!("");
                });
                println!("-----------------------------------------------------------------------------------------------------------------------------------------"); */
            }
        }

        Cave { inner }
    }

    pub fn add_floor(mut self) -> Self {
        self.inner.push([46u8; 600]);
        self.inner.push([35; 600]);
        self
    }

    fn simulate_grain_of_sand(&mut self, origin: IVec2, check_for_abyss: bool) -> Option<IVec2> {
        //println!("simulatin falling grain, origin={origin}");
        // if we are about to fall into abyss, return None
        if check_for_abyss {
            if origin.y >= self.inner.len() as i32 - 1 {
                return None;
            }
        }

        if origin.x <= 1 || origin.x >= 599 {
            return Some(origin);
        }

        if self.can_fall(origin) {
            let next = IVec2::new(origin.x, origin.y + 1);
            return self.simulate_grain_of_sand(next, check_for_abyss);
        }

        if self.can_fall_left(origin) {
            let next = IVec2::new(origin.x - 1, origin.y + 1);
            return self.simulate_grain_of_sand(next, check_for_abyss);
        }

        if self.can_fall_right(origin) {
            let next = IVec2::new(origin.x + 1, origin.y + 1);
            return self.simulate_grain_of_sand(next, check_for_abyss);
        }

        // we can't fall anymore because we got stuck in a position
        Some(origin)
    }

    fn can_fall(&self, current_pos: IVec2) -> bool {
        self.inner[(current_pos.y + 1) as usize][current_pos.x as usize] == 46
    }
    fn can_fall_right(&self, current_pos: IVec2) -> bool {
        self.inner[(current_pos.y + 1) as usize][current_pos.x as usize + 1] == 46
    }
    fn can_fall_left(&self, current_pos: IVec2) -> bool {
        self.inner[(current_pos.y + 1) as usize][current_pos.x as usize - 1] == 46
    }

    fn print(&self) {
        self.inner.iter().for_each(|line| {
            line.iter()
                .skip(430)
                .map(|b| char::from(*b))
                .for_each(|c| print!("{c}"));
            println!("")
        });
        //println!("{:?}", &self.inner.iter().skip(400).map(|b| char::from(b)));
    }
}

fn add_stones(cave: &mut Vec<[u8; 600]>, start: IVec2, end: IVec2) {
    //println!("start={start}, end={end}");
    let diff = start - end;
    //println!("diff={diff}");

    if start.x != end.x {
        let mut current = if !diff.x.is_negative() { end } else { start };

        //println!("current={current}");
        for _ in 0..=(diff.x.abs()) {
            //  println!("{i} - X - current={current}");
            cave[current.y as usize][current.x as usize] = b'#';
            current.x += 1;
        }
    }
    // we are drawing from to right so if we are supposed to draw to the left,
    // we have to go back and draw to right

    if start.y != end.y {
        // we draw from top to bottom, so if coords are from bottom to top, we have to offset
        let mut current = if !diff.y.is_negative() { end } else { start };

        //println!("current={current}");
        for _ in 0..=(diff.y.abs()) {
            // println!("{i} - Y - current={current}");
            cave[current.y as usize][current.x as usize] = b'#';
            current.y += 1;
        }
    }
}

#[aoc_generator(day14)]
pub fn parse_input(input: &str) -> (Vec<Vec<IVec2>>, i32) {
    let mut depth = 0;
    let lines = input
        .lines()
        .map(|line| {
            line.split("->")
                .filter_map(|splitted| {
                    splitted.split_once(",").map(|(x, y)| {
                        let (x, y) = (
                            x.trim().parse::<i32>().unwrap(),
                            y.trim().parse::<i32>().unwrap(),
                        );

                        if y > depth {
                            depth = y
                        }

                        IVec2::new(x, y)
                    })
                })
                .collect::<Vec<IVec2>>()
        })
        .collect::<Vec<Vec<IVec2>>>();

    (lines, depth)
}

#[aoc(day14, part1)]
pub fn part1((input, depth): &(Vec<Vec<IVec2>>, i32)) -> usize {
    //println!("running part1");
    let mut sand_counter = 0;
    let mut cave = Cave::new(input, *depth as usize);
    while let Some(sand_stuck_at) = cave.simulate_grain_of_sand(IVec2::new(500, 0), true) {
        //println!("sand_stuck_at={sand_stuck_at}");
        cave.inner[sand_stuck_at.y as usize][sand_stuck_at.x as usize] = b'o';
        /* println!(
            "stuck_at_position={}",
            cave.inner[sand_stuck_at.y as usize][sand_stuck_at.x as usize]
        ); */
        sand_counter += 1;
    }

    //cave.print();

    sand_counter
}

#[aoc(day14, part2)]
pub fn part2((input, depth): &(Vec<Vec<IVec2>>, i32)) -> usize {
    todo!("can't use array as it creates fictive bounday")
    /* let mut sand_counter = 0;
    let mut cave = Cave::new(input, *depth as usize).add_floor();
    while let Some(sand_stuck_at) = cave.simulate_grain_of_sand(IVec2::new(500, 0), false) {
        //println!("sand_stuck_at={sand_stuck_at}");
        cave.inner[sand_stuck_at.y as usize][sand_stuck_at.x as usize] = b'o';
        /* println!(
            "stuck_at_position={}",
            cave.inner[sand_stuck_at.y as usize][sand_stuck_at.x as usize]
        ); */
        sand_counter += 1;
        println!("sand_stuck_at={sand_stuck_at}");
        if sand_stuck_at.x == 500 && sand_stuck_at.y == 0 {
            cave.print();
            return sand_counter;
        }

        if sand_counter % 500 == 0 {
            let now = chrono::Local::now();
            println!("{} still working", now.format("HH:MM:SS"))
        }

        if sand_counter % 1000 == 0 {
            println!("DEBUG CAVE ====================================================================================================================");
            cave.print();
            println!("===============================================================================================================================");
        }
    }

    cave.print();

    sand_counter */
}

const TEST_INPUT: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

#[test]
fn fill_with_stones_test() {
    let (stones, depth) = parse_input(TEST_INPUT);
    let mut cave = Cave::new(&stones, depth as usize);
    cave.print();
    panic!()
}

#[test]
fn parse_input_test() {
    let got = parse_input(TEST_INPUT);
    let want = vec![
        vec![IVec2::new(498, 4), IVec2::new(498, 6), IVec2::new(496, 6)],
        vec![
            IVec2::new(503, 4),
            IVec2::new(502, 4),
            IVec2::new(502, 9),
            IVec2::new(494, 9),
        ],
    ];
}

#[test]
pub fn part1_test() {
    let got = part1(&parse_input(TEST_INPUT));
    assert_eq!(24, got)
}

#[test]
pub fn part2_test() {}
