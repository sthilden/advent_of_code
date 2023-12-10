use std::{fs, path::Path};

fn solve(contents: &str, reverse: bool) -> i64 {
    let mut sum = 0;

    contents.lines().for_each(|line| {
        let mut numbers = line
            .split_whitespace()
            .into_iter()
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        if reverse {
            numbers.reverse();
        }

        for outer_inx in (1..numbers.len()).rev() {
            let mut num_inx = numbers[outer_inx];
            let mut all_zeros = true;
            for inx in (1..outer_inx + 1).rev() {
                let next_num_inx = numbers[inx - 1];
                numbers[inx - 1] = num_inx - numbers[inx - 1];
                if numbers[inx - 1] != 0 {
                    all_zeros = false;
                }
                num_inx = next_num_inx;
            }

            if all_zeros {
                let mut x = numbers[outer_inx - 1];
                for inx in outer_inx..numbers.len() {
                    // x - numbers[inx] = numbers[inx - 1]
                    // => x = numbers[inx - 1] + numbers[inx]
                    x += numbers[inx];
                }
                sum += x;
                break;
            }
        }
    });

    sum
}

pub fn problem1() {
    println!("problem 1");

    let path = Path::new("resources/day9_input");
    let contents = fs::read_to_string(path).unwrap();

    let sum = solve(&contents, false);

    println!("Answer: {}", sum);
}

pub fn problem2() {
    println!("problem 2");

    let path = Path::new("resources/day9_input");
    let contents = fs::read_to_string(path).unwrap();

    let sum = solve(&contents, true);

    println!("Answer: {}", sum);
}
