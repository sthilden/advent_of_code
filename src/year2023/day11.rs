use std::{fs, path::Path};

fn distance(g1: (usize, usize), g2: (usize, usize)) -> usize {
    std::cmp::max(g1.0, g2.0) - std::cmp::min(g1.0, g2.0) + std::cmp::max(g1.1, g2.1)
        - std::cmp::min(g1.1, g2.1)
}

fn solve(expansion_factor: usize) -> usize {
    let path = Path::new("resources/day11_input");
    let contents = fs::read_to_string(path).unwrap();

    // Parse input
    let mut map: Vec<Vec<char>> = Vec::new();
    contents.lines().for_each(|line| {
        map.push(line.chars().collect());
    });

    // Print map
    // map.iter().for_each(|r| {
    //     r.iter().for_each(|c| print!("{}", c));
    //     println!("");
    // });

    // Row expansion
    let mut row_expansion = vec![0; map.len()];
    let mut expansion = 0;
    for i in 0..map.len() {
        let mut do_expansion = true;
        for j in 0..map.len() {
            if map[i][j] != '.' {
                do_expansion = false;
                break;
            }
        }
        if do_expansion {
            expansion += 1;
        }
        row_expansion[i] = expansion;
    }

    // Column expansion
    let mut col_expansion = vec![0; map[0].len()];
    let mut expansion = 0;
    for j in 0..map.len() {
        let mut do_expansion = true;
        for i in 0..map.len() {
            if map[i][j] != '.' {
                do_expansion = false;
                break;
            }
        }
        if do_expansion {
            expansion += 1;
        }
        col_expansion[j] = expansion;
    }

    // Find all galaxies
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for i in 0..map.len() {
        for j in 0..map.len() {
            if map[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }

    // Apply expansion
    for i in 0..galaxies.len() {
        galaxies[i].0 += (expansion_factor - 1) * row_expansion[galaxies[i].0];
        galaxies[i].1 += (expansion_factor - 1) * col_expansion[galaxies[i].1];
    }

    // Find distance between all pairs
    let mut sum = 0;
    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            sum += distance(galaxies[i], galaxies[j]);
        }
    }

    sum
}

pub fn problem1() {
    println!("problem 1");

    println!("Answer={}", solve(2));
}

pub fn problem2() {
    println!("problem 2");

    println!("Answer={}", solve(1000000));
}
