use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::{fs, path::Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Item {
    RoundRock,
    SquareRock,
    Nothing,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Platform {
    map: Vec<Vec<Item>>,
}

impl Platform {
    pub fn new() -> Self {
        Platform { map: Vec::new() }
    }
}

fn read_input() -> Platform {
    let path = Path::new("resources/day14_input");
    let contents = fs::read_to_string(path).unwrap();

    let mut platform = Platform::new();

    // Read file into pattern rows
    contents.lines().for_each(|line| {
        if line.is_empty() {
            return;
        }
        platform.map.push(
            line.chars()
                .into_iter()
                .map(|c| match c {
                    '.' => Item::Nothing,
                    '#' => Item::SquareRock,
                    'O' => Item::RoundRock,
                    _ => panic!("Unknown item"),
                })
                .collect::<Vec<Item>>(),
        );
    });

    platform
}

pub fn problem1() {
    println!("problem 1");

    let mut platform = read_input();

    // Tilt platform
    let mut something_moved = true;
    while something_moved {
        something_moved = false;
        for i in 1..platform.map.len() {
            for j in 0..platform.map[i].len() {
                if platform.map[i][j] == Item::RoundRock && platform.map[i - 1][j] == Item::Nothing
                {
                    // Move item up
                    platform.map[i - 1][j] = Item::RoundRock;
                    platform.map[i][j] = Item::Nothing;
                    something_moved = true;
                }
            }
        }
    }

    // Calculate load
    let nrows = platform.map.len();
    let load = platform
        .map
        .iter()
        .enumerate()
        .map(|(inx, row)| {
            row.iter()
                .map(|c| {
                    if *c == Item::RoundRock {
                        nrows - inx
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("Answer: {}", load);
}

fn print_platform(platform: &Platform) {
    platform.map.iter().for_each(|r| {
        r.iter().for_each(|item| {
            print!(
                "{}",
                match item {
                    Item::Nothing => '.',
                    Item::RoundRock => 'O',
                    Item::SquareRock => '#',
                }
            )
        });
        println!("");
    });
    println!("");
}

fn do_cycle(platform: &mut Platform) {
    // Tilt platform North
    let mut something_moved = true;
    while something_moved {
        something_moved = false;
        for i in 1..platform.map.len() {
            for j in 0..platform.map[i].len() {
                if platform.map[i][j] == Item::RoundRock && platform.map[i - 1][j] == Item::Nothing
                {
                    // Move item up
                    platform.map[i - 1][j] = Item::RoundRock;
                    platform.map[i][j] = Item::Nothing;
                    something_moved = true;
                }
            }
        }
    }

    // Tilt platform West
    let mut something_moved = true;
    while something_moved {
        something_moved = false;
        for j in 1..platform.map[0].len() {
            for i in 0..platform.map.len() {
                if platform.map[i][j] == Item::RoundRock && platform.map[i][j - 1] == Item::Nothing
                {
                    // Move item left
                    platform.map[i][j - 1] = Item::RoundRock;
                    platform.map[i][j] = Item::Nothing;
                    something_moved = true;
                }
            }
        }
    }

    // Tilt platform South
    let mut something_moved = true;
    while something_moved {
        something_moved = false;
        for i in (0..platform.map.len() - 1).rev() {
            for j in 0..platform.map[i].len() {
                if platform.map[i][j] == Item::RoundRock && platform.map[i + 1][j] == Item::Nothing
                {
                    // Move item down
                    platform.map[i + 1][j] = Item::RoundRock;
                    platform.map[i][j] = Item::Nothing;
                    something_moved = true;
                }
            }
        }
    }

    // Tilt platform East
    let mut something_moved = true;
    while something_moved {
        something_moved = false;
        for j in (0..platform.map[0].len() - 1).rev() {
            for i in 0..platform.map.len() {
                if platform.map[i][j] == Item::RoundRock && platform.map[i][j + 1] == Item::Nothing
                {
                    // Move item right
                    platform.map[i][j + 1] = Item::RoundRock;
                    platform.map[i][j] = Item::Nothing;
                    something_moved = true;
                }
            }
        }
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn problem2() {
    println!("problem 2");

    let mut platform = read_input();

    let mut map: BTreeMap<u64, usize> = BTreeMap::new();
    map.insert(calculate_hash(&platform), 0);

    let iter_total = 1000000000;
    let mut iter = 0;
    for iter_inx in 0..iter_total {
        do_cycle(&mut platform);
        let hash = calculate_hash(&platform);
        if let Some(inx) = map.get(&hash) {
            println!(
                "Found same platform state after {} cycles as after {} cycles!",
                iter_inx, inx
            );
            let iter_left = iter_total - iter_inx - 1;
            let iter_gap = iter_inx + 1 - inx;
            let times = iter_left / iter_gap;
            iter = (iter_inx + 1) + times * iter_gap;
            break;
        }
        map.insert(hash, iter_inx + 1);
    }

    println!("Jumping to iteration number {}", iter);
    for _ in iter..iter_total {
        do_cycle(&mut platform);
    }

    // println!();
    // print_platform(&platform);

    // Calculate load
    let nrows = platform.map.len();
    let load = platform
        .map
        .iter()
        .enumerate()
        .map(|(inx, row)| {
            row.iter()
                .map(|c| {
                    if *c == Item::RoundRock {
                        nrows - inx
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("Answer: {}", load);
}
