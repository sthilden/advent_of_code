use std::{collections::HashMap, fs, path::Path};

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    L,
    R,
}

pub fn problem1() {
    println!("problem 1");

    let path = Path::new("resources/day8_input");
    let contents = fs::read_to_string(path).unwrap();

    let mut instructions: Vec<Direction> = Vec::new();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    // Parse input
    contents
        .lines()
        .into_iter()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            if instructions.is_empty() {
                instructions = line
                    .chars()
                    .into_iter()
                    .map(|c| match c {
                        'L' => Some(Direction::L),
                        'R' => Some(Direction::R),
                        _ => None,
                    })
                    .flatten()
                    .collect();
            } else {
                // AAA = (BBB, CCC)
                let (from, left_right) = line.split_once('=').unwrap();
                let (left, right) = rem_first_and_last(left_right.trim())
                    .split_once(',')
                    .unwrap();
                map.insert(from.trim(), (left.trim(), right.trim()));
            }
        });

    // Navigate map
    let mut steps = 0;
    let mut pos = "AAA";
    while pos != "ZZZ" {
        for dir in &instructions {
            if pos == "ZZZ" {
                break;
            }
            match dir {
                Direction::L => pos = map.get(pos).unwrap().0,
                Direction::R => pos = map.get(pos).unwrap().1,
            }
            steps += 1;
        }
    }

    println!("Answer: {}", steps);
}

pub fn problem2() {
    println!("problem 2");

    let path = Path::new("resources/day8_input");
    let contents = fs::read_to_string(path).unwrap();

    let mut instructions: Vec<Direction> = Vec::new();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    // Parse input
    contents
        .lines()
        .into_iter()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            if instructions.is_empty() {
                instructions = line
                    .chars()
                    .into_iter()
                    .map(|c| match c {
                        'L' => Some(Direction::L),
                        'R' => Some(Direction::R),
                        _ => None,
                    })
                    .flatten()
                    .collect();
            } else {
                // AAA = (BBB, CCC)
                let (from, left_right) = line.split_once('=').unwrap();
                let (left, right) = rem_first_and_last(left_right.trim())
                    .split_once(',')
                    .unwrap();
                map.insert(from.trim(), (left.trim(), right.trim()));
            }
        });

    // Find starting positions
    let mut positions: Vec<&str> = map
        .iter()
        .filter(|(from, _)| from.ends_with('A'))
        .map(|(from, _)| *from)
        .collect();

    // Navigate map
    let mut steps = 0;
    let mut done = false;
    while !done {
        for dir in &instructions {
            positions.iter_mut().for_each(|pos| match dir {
                Direction::L => *pos = map.get(pos).unwrap().0,
                Direction::R => *pos = map.get(pos).unwrap().1,
            });
            done = positions.iter().all(|pos| pos.ends_with('Z'));
            steps += 1;
            if done {
                break;
            }
        }
    }

    println!("Answer: {}", steps);
}
