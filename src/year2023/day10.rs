use std::{fs, path::Path};

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn new_direction(c: char, dir: Direction) -> Option<Direction> {
    match c {
        '|' => Some(dir),
        '-' => Some(dir),
        'L' => match dir {
            Direction::Down => Some(Direction::Right),
            Direction::Left => Some(Direction::Up),
            _ => None,
        },
        'J' => match dir {
            Direction::Down => Some(Direction::Left),
            Direction::Right => Some(Direction::Up),
            _ => None,
        },
        '7' => match dir {
            Direction::Up => Some(Direction::Left),
            Direction::Right => Some(Direction::Down),
            _ => None,
        },
        'F' => match dir {
            Direction::Up => Some(Direction::Right),
            Direction::Left => Some(Direction::Down),
            _ => None,
        },
        _ => None,
    }
}

fn new_position(
    pos: (usize, usize),
    dir: Direction,
    map: &Vec<Vec<char>>,
) -> Option<(usize, usize)> {
    if map.is_empty() {
        return None;
    }
    let mut new_pos = pos;
    match dir {
        Direction::Up => {
            if new_pos.0 > 0 {
                new_pos.0 -= 1;
            } else {
                return None;
            }
        }
        Direction::Down => {
            if new_pos.0 < map.len() - 1 {
                new_pos.0 += 1;
            } else {
                return None;
            }
        }
        Direction::Left => {
            if new_pos.1 > 0 {
                new_pos.1 -= 1;
            } else {
                return None;
            }
        }
        Direction::Right => {
            if new_pos.1 < map[0].len() - 1 {
                new_pos.1 += 1;
            } else {
                return None;
            }
        }
    }
    Some(new_pos)
}

pub fn problem1() {
    println!("problem 1");

    let path = Path::new("resources/day10_input");
    let contents = fs::read_to_string(path).unwrap();

    // Parse input
    let mut map: Vec<Vec<char>> = Vec::new();
    contents.lines().for_each(|line| {
        map.push(line.chars().collect());
    });

    // Find starting position
    let mut start_pos = None;
    map.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, c)| {
            if *c == 'S' {
                start_pos = Some((i, j));
            }
        })
    });

    // Check each of the four possible starting directions
    let mut result = None;
    for start_dir in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .into_iter()
    {
        println!("Checking starting direction {:?}", start_dir);

        let mut pos = start_pos.unwrap();
        let mut dir = start_dir;
        if let Some(p) = new_position(pos, dir, &map) {
            pos = p;
        } else {
            continue;
        }
        let mut steps = 1; // already taken 1 step

        loop {
            let current_char = map[pos.0][pos.1];

            if current_char == 'S' {
                break;
            }

            // Update direction and position
            if let Some(d) = new_direction(current_char, dir) {
                dir = d;
                pos = new_position(pos, dir, &map).unwrap();
                steps += 1;
            } else {
                break;
            }
        }

        if steps > 1 {
            result = Some(steps / 2);
        }
    }

    println!("Answer: {}", result.unwrap());
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Color {
    Path,
    Left,
    Right,
    None,
}

fn set_color(map: &mut Vec<Vec<Color>>, i: usize, j: usize, color: Color) {
    if i < map.len() && j < map[i].len() && map[i][j] != Color::Path {
        map[i][j] = color;
    }
}

fn get_left_right(map: &Vec<Vec<Color>>, i: usize, j: usize) -> Option<Color> {
    if i < map.len() - 1 && (map[i + 1][j] == Color::Left || map[i + 1][j] == Color::Right) {
        return Some(map[i + 1][j]);
    }
    if j < map.len() - 1 && (map[i][j + 1] == Color::Left || map[i][j + 1] == Color::Right) {
        return Some(map[i][j + 1]);
    }
    if i > 0 && (map[i - 1][j] == Color::Left || map[i - 1][j] == Color::Right) {
        return Some(map[i - 1][j]);
    }
    if j > 0 && (map[i][j - 1] == Color::Left || map[i][j - 1] == Color::Right) {
        return Some(map[i][j - 1]);
    }
    None
}

fn update_color_map(color_map: &mut Vec<Vec<Color>>, dir: Direction, pos: (usize, usize)) {
    match dir {
        Direction::Up => {
            if pos.1 > 0 {
                set_color(color_map, pos.0, pos.1 - 1, Color::Left);
            }
            set_color(color_map, pos.0, pos.1 + 1, Color::Right);
        }
        Direction::Down => {
            if pos.1 > 0 {
                set_color(color_map, pos.0, pos.1 - 1, Color::Right);
            }
            set_color(color_map, pos.0, pos.1 + 1, Color::Left);
        }
        Direction::Left => {
            if pos.0 > 0 {
                set_color(color_map, pos.0 - 1, pos.1, Color::Right);
            }
            set_color(color_map, pos.0 + 1, pos.1, Color::Left);
        }
        Direction::Right => {
            if pos.0 > 0 {
                set_color(color_map, pos.0 - 1, pos.1, Color::Left);
            }
            set_color(color_map, pos.0 + 1, pos.1, Color::Right);
        }
    }
}

pub fn problem2() {
    println!("problem 2");

    let path = Path::new("resources/day10_input");
    let contents = fs::read_to_string(path).unwrap();

    // Parse input
    let mut map: Vec<Vec<char>> = Vec::new();
    contents.lines().for_each(|line| {
        map.push(line.chars().collect());
    });

    // Find starting position
    let mut start_pos = None;
    map.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, c)| {
            if *c == 'S' {
                start_pos = Some((i, j));
            }
        })
    });

    // Create coloring map
    let mut color_map: Vec<Vec<Color>> = Vec::new();

    // Check each of the four possible starting directions
    let mut done = false;
    for start_dir in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .into_iter()
    {
        // Reset color map
        color_map = Vec::new();
        map.iter().for_each(|v| {
            color_map.push(vec![Color::None; v.len()]);
        });

        let mut pos = start_pos.unwrap();
        let mut dir = start_dir;
        let mut prev_pos = pos;
        if let Some(p) = new_position(pos, dir, &map) {
            prev_pos = pos;
            pos = p;
        } else {
            continue;
        }

        loop {
            let current_char = map[pos.0][pos.1];

            // Update color map
            color_map[pos.0][pos.1] = Color::Path;
            update_color_map(&mut color_map, dir, prev_pos);
            update_color_map(&mut color_map, dir, pos);

            if current_char == 'S' {
                done = true;
                break;
            }

            // Update direction and position
            if let Some(d) = new_direction(current_char, dir) {
                dir = d;
                prev_pos = pos;
                pos = new_position(pos, dir, &map).unwrap();
            } else {
                break;
            }
        }

        if done {
            break;
        }
    }

    // Color rest of map
    let mut done = false;
    while !done {
        done = true;
        let mut no_coloring = true;
        for i in 0..color_map.len() {
            for j in 0..color_map[i].len() {
                if color_map[i][j] == Color::None {
                    if let Some(color) = get_left_right(&color_map, i, j) {
                        color_map[i][j] = color;
                        no_coloring = false;
                    } else {
                        done = false;
                    }
                }
            }
        }
        assert!(!no_coloring);
    }

    // Find what is outside and what is inside
    let outside = (|| {
        for i in [0, color_map.len() - 1].into_iter() {
            for j in [0, color_map[i].len() - 1].into_iter() {
                let c = color_map[i][j];
                if c == Color::Right || c == Color::Left {
                    return Some(c);
                }
            }
        }
        None
    })()
    .unwrap();

    let inside = match outside {
        Color::Right => Some(Color::Left),
        Color::Left => Some(Color::Right),
        _ => None,
    }
    .unwrap();

    // Print color map
    // color_map.iter().for_each(|r| {
    //     r.iter().for_each(|c| {
    //         match *c {
    //             Color::Path => print!("o"),
    //             Color::Left => print!("L"),
    //             Color::Right => print!("R"),
    //             Color::None => print!("."),
    //         };
    //     });
    //     println!("");
    // });

    // Count number of inside occurances
    let mut sum = 0;
    color_map.iter().for_each(|r| {
        r.iter().for_each(|c| {
            if *c == inside {
                sum += 1;
            }
        })
    });

    println!("Answer: {}", sum);
}
