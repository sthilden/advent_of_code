use std::{cmp, fs, path::Path};

pub fn problem1() {
    println!("problem 1");

    // Determine which games would have been possible if the bag had been
    // loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes

    let path = Path::new("resources/day2_input");
    let contents = fs::read_to_string(path).unwrap();

    let mut sum = 0;

    contents.lines().into_iter().for_each(|line| {
        let (game, sets) = line.split_once(':').unwrap();
        let game_num = game.split_once(' ').unwrap().1.parse::<i32>().unwrap();

        let ok = sets.split(';').into_iter().all(|set| {
            set.split(",").into_iter().all(|s| {
                let (num_str, color) = s.trim().split_once(' ').unwrap();
                let num = num_str.parse::<i32>().unwrap();
                match color {
                    "red" => num <= 12,
                    "green" => num <= 13,
                    "blue" => num <= 14,
                    _ => false,
                }
            })
        });

        if ok {
            sum += game_num;
        }
    });

    println!("Answer: {}", sum);
}

pub fn problem2() {
    println!("problem 2");

    let path = Path::new("resources/day2_input");
    let contents = fs::read_to_string(path).unwrap();

    let mut sum = 0;

    contents.lines().into_iter().for_each(|line| {
        let (_, sets) = line.split_once(':').unwrap();

        let mut max_count = (0, 0, 0);
        sets.split(';').into_iter().for_each(|set| {
            set.split(",").into_iter().for_each(|s| {
                let (num_str, color) = s.trim().split_once(' ').unwrap();
                let num = num_str.parse::<i32>().unwrap();
                match color {
                    "red" => max_count.0 = cmp::max(max_count.0, num),
                    "green" => max_count.1 = cmp::max(max_count.1, num),
                    "blue" => max_count.2 = cmp::max(max_count.2, num),
                    _ => {}
                }
            })
        });

        sum += max_count.0 * max_count.1 * max_count.2;
    });

    println!("Answer: {}", sum);
}
