use std::{fs, path::Path};

pub fn problem1() {
    println!("problem 1");

    let path = Path::new("resources/day4_input");
    let contents = fs::read_to_string(path).unwrap();

    let mut sum = 0;

    contents.lines().into_iter().for_each(|line| {
        let (_, numbers) = line.split_once(':').unwrap();
        let (winning_str, our_str) = numbers.split_once('|').unwrap();

        let winning_vec = winning_str
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let count = our_str
            .split_whitespace()
            .filter(|n| winning_vec.contains(&n.parse::<u32>().unwrap()))
            .count();

        if count > 0 {
            sum += (2 as u32).pow((count - 1) as u32);
        }
    });

    println!("Answer: {}", sum);
}

pub fn problem2() {
    println!("problem 2");

    let path = Path::new("resources/day4_input");
    let contents = fs::read_to_string(path).unwrap();

    let num_cards = contents.lines().into_iter().count();
    let mut copies: Vec<usize> = vec![1; num_cards];

    contents
        .lines()
        .into_iter()
        .enumerate()
        .for_each(|(line_inx, line)| {
            let (_, numbers) = line.split_once(':').unwrap();
            let (winning_str, our_str) = numbers.split_once('|').unwrap();

            let winning_vec = winning_str
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let count = our_str
                .split_whitespace()
                .filter(|n| winning_vec.contains(&n.parse::<u32>().unwrap()))
                .count();

            let copies_of_current = copies[line_inx];
            for inx in line_inx + 1..line_inx + 1 + count {
                if inx > num_cards - 1 {
                    break;
                }
                copies[inx] += copies_of_current;
            }
        });

    let sum: usize = copies.into_iter().sum();
    println!("Answer: {}", sum);
}
