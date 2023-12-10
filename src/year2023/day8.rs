use std::{collections::HashMap, fs, path::Path};

use itertools::Itertools;
use num::integer::lcm;

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

    // Assuming all paths have the same jump repeating between every time it
    // reaches the Z positions.
    let mut jumps: Vec<u64> = Vec::new();
    for start_pos in positions {
        let mut pos = start_pos;
        let mut steps = 0;
        loop {
            for dir in &instructions {
                match dir {
                    Direction::L => pos = map.get(pos).unwrap().0,
                    Direction::R => pos = map.get(pos).unwrap().1,
                }
                steps += 1;
            }
            if pos.ends_with('Z') {
                jumps.push(steps);
                break;
            }
        }
    }
    println!("jumps: {:?}", jumps);
    // jumps: [21251, 19637, 19099, 12643, 15871, 11567]
    // LCM: 13133452426987 <-- correct answer
    // Used online tool to compute LCM for all jumps

    // let mut steps = jumps.clone();
    // for inx in 0..steps.len() - 2 {
    //     let lcm = lcm(steps[inx], steps[inx + 1]);
    //     println!("lcm: {}", lcm);
    // }

    // println!("Answer: {}", result);

    // TODO: Actually solve this problem here in code
}
