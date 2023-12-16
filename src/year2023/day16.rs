use std::{
    collections::{BTreeSet, LinkedList},
    fs,
    path::Path,
};

use itertools::Itertools;

#[derive(Debug)]
enum Item {
    Empty,              // Empty space .
    MirrorUpRight,      // Mirror /
    MirrorUpLeft,       // Mirror \
    SplitterVertical,   // Splitter |
    SplitterHorizontal, // Splitter -
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct Ray {
    row: usize,
    col: usize,
    dir: Direction,
}

fn read_map() -> Vec<Vec<Item>> {
    let path = Path::new("resources/day16_input");
    let contents = fs::read_to_string(path).unwrap();

    // Contruct map
    let map = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Item::Empty,
                    '/' => Item::MirrorUpRight,
                    '\\' => Item::MirrorUpLeft,
                    '-' => Item::SplitterHorizontal,
                    '|' => Item::SplitterVertical,
                    _ => panic!("unexpected character: {}", c),
                })
                .collect_vec()
        })
        .collect_vec();

    map
}

fn get_energized_tile_count(map: &Vec<Vec<Item>>, start_ray: Ray) -> usize {
    let num_rows = map.len();
    let num_cols = if let Some(row) = map.first() {
        row.len()
    } else {
        0
    };

    // Ray tracing
    let mut ray_map: Vec<Vec<BTreeSet<Direction>>> =
        vec![vec![BTreeSet::new(); num_cols]; num_rows];
    let mut rays: LinkedList<Ray> = LinkedList::new();
    rays.push_back(start_ray);
    'next_ray: while let Some(mut ray) = rays.pop_front() {
        loop {
            // Check / update ray map
            let ray_set = &mut ray_map[ray.row][ray.col];
            if ray_set.contains(&ray.dir) {
                continue 'next_ray;
            }
            ray_set.insert(ray.dir);

            // Update direction or split ray in two
            match map[ray.row][ray.col] {
                Item::Empty => (),
                Item::MirrorUpRight => match ray.dir {
                    Direction::Left => ray.dir = Direction::Down,
                    Direction::Right => ray.dir = Direction::Up,
                    Direction::Up => ray.dir = Direction::Right,
                    Direction::Down => ray.dir = Direction::Left,
                },
                Item::MirrorUpLeft => match ray.dir {
                    Direction::Left => ray.dir = Direction::Up,
                    Direction::Right => ray.dir = Direction::Down,
                    Direction::Up => ray.dir = Direction::Left,
                    Direction::Down => ray.dir = Direction::Right,
                },
                Item::SplitterVertical => match ray.dir {
                    Direction::Left | Direction::Right => {
                        if ray.row > 0 {
                            rays.push_back(Ray {
                                row: ray.row - 1,
                                col: ray.col,
                                dir: Direction::Up,
                            });
                        }
                        if ray.row < num_rows - 1 {
                            rays.push_back(Ray {
                                row: ray.row + 1,
                                col: ray.col,
                                dir: Direction::Down,
                            });
                        }
                        continue 'next_ray;
                    }
                    _ => (),
                },
                Item::SplitterHorizontal => match ray.dir {
                    Direction::Up | Direction::Down => {
                        if ray.col > 0 {
                            rays.push_back(Ray {
                                row: ray.row,
                                col: ray.col - 1,
                                dir: Direction::Left,
                            });
                        }
                        if ray.col < num_cols - 1 {
                            rays.push_back(Ray {
                                row: ray.row,
                                col: ray.col + 1,
                                dir: Direction::Right,
                            });
                        }
                        continue 'next_ray;
                    }
                    _ => (),
                },
            }

            // Move ray one step in the current direction
            match ray.dir {
                Direction::Left => {
                    if ray.col > 0 {
                        ray.col -= 1;
                    } else {
                        continue 'next_ray;
                    }
                }
                Direction::Right => {
                    if ray.col < num_cols - 1 {
                        ray.col += 1;
                    } else {
                        continue 'next_ray;
                    }
                }
                Direction::Up => {
                    if ray.row > 0 {
                        ray.row -= 1;
                    } else {
                        continue 'next_ray;
                    }
                }
                Direction::Down => {
                    if ray.row < num_rows - 1 {
                        ray.row += 1;
                    } else {
                        continue 'next_ray;
                    }
                }
            }
        }
    }

    // Compute number of energized tiles
    let energized_tile_count = ray_map
        .iter()
        .map(|row| {
            row.iter()
                .map(|set| if set.is_empty() { 0 } else { 1 })
                .sum::<usize>()
        })
        .sum::<usize>();

    energized_tile_count
}

pub fn problem1() {
    println!("problem 1");

    let map = read_map();
    let start_ray = Ray {
        row: 0,
        col: 0,
        dir: Direction::Right,
    };
    let energized_tile_count = get_energized_tile_count(&map, start_ray);

    println!("Answer: {}", energized_tile_count);
}

pub fn problem2() {
    println!("problem 2");

    let map = read_map();

    let num_rows = map.len();
    let num_cols = if let Some(row) = map.first() {
        row.len()
    } else {
        0
    };

    let mut max_energized_tile_count = 0;
    for row in 0..num_rows {
        let start_ray = Ray {
            row,
            col: 0,
            dir: Direction::Right,
        };
        max_energized_tile_count = std::cmp::max(
            max_energized_tile_count,
            get_energized_tile_count(&map, start_ray),
        );
        let start_ray = Ray {
            row,
            col: num_cols - 1,
            dir: Direction::Left,
        };
        max_energized_tile_count = std::cmp::max(
            max_energized_tile_count,
            get_energized_tile_count(&map, start_ray),
        );
    }
    for col in 0..num_cols {
        let start_ray = Ray {
            row: 0,
            col,
            dir: Direction::Down,
        };
        max_energized_tile_count = std::cmp::max(
            max_energized_tile_count,
            get_energized_tile_count(&map, start_ray),
        );
        let start_ray = Ray {
            row: num_rows - 1,
            col,
            dir: Direction::Up,
        };
        max_energized_tile_count = std::cmp::max(
            max_energized_tile_count,
            get_energized_tile_count(&map, start_ray),
        );
    }

    println!("Answer: {}", max_energized_tile_count);
}
