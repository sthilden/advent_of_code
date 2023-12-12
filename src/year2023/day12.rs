use std::{fs, path::Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

fn is_valid(list: &Vec<Spring>, groups: &Vec<usize>) -> bool {
    let damaged_count = list.iter().filter(|s| **s == Spring::Damaged).count();
    let expected_count = groups.iter().sum();
    if damaged_count != expected_count {
        return false;
    }

    let mut actual_groups: Vec<usize> = Vec::new();
    let mut group_count = 0;
    list.iter().for_each(|spring| match spring {
        Spring::Damaged => {
            group_count += 1;
        }
        Spring::Operational => {
            if group_count > 0 {
                actual_groups.push(group_count);
                group_count = 0;
            }
        }
        Spring::Unknown => panic!("unknown present in is_valid!"),
    });

    // Final group, if any
    if group_count > 0 {
        actual_groups.push(group_count);
    }

    actual_groups == *groups
}

fn solve(list: &Vec<Spring>, groups: &Vec<usize>) -> usize {
    // Check count first
    let mut damaged_count = 0;
    let mut unknown_count = 0;
    list.iter().for_each(|s| match s {
        Spring::Damaged => damaged_count += 1,
        Spring::Unknown => unknown_count += 1,
        _ => (),
    });
    let expected_damaged_count = groups.iter().sum::<usize>();
    if damaged_count > expected_damaged_count
        || damaged_count + unknown_count < expected_damaged_count
    {
        return 0;
    }

    // If there are no unknowns, then validate solution
    if unknown_count == 0 {
        if is_valid(&list, groups) {
            return 1;
        } else {
            return 0;
        }
    }

    // Recursively check possible solutions
    let mut solutions = 0;
    for inx in 0..list.len() {
        if list[inx] == Spring::Unknown {
            let mut list_clone = list.clone();
            list_clone[inx] = Spring::Operational;
            solutions += solve(&list_clone, &groups);
            list_clone[inx] = Spring::Damaged;
            solutions += solve(&list_clone, &groups);
            break;
        }
    }

    solutions
}

pub fn problem1() {
    println!("problem 1");

    let path = Path::new("resources/day12_input");
    let contents = fs::read_to_string(path).unwrap();

    let mut sum = 0;
    contents.lines().for_each(|line| {
        let (list_str, groups_str) = line.split_once(' ').unwrap();
        let list = list_str
            .chars()
            .into_iter()
            .map(|c| match c {
                '.' => Spring::Operational,
                '#' => Spring::Damaged,
                '?' => Spring::Unknown,
                _ => panic!("unknown character for spring"),
            })
            .collect::<Vec<Spring>>();
        let groups = groups_str
            .split(',')
            .into_iter()
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let solutions = solve(&list, &groups);
        sum += solutions;
    });

    println!("Answer: {}", sum);
}

pub fn problem2() {
    println!("problem 2");

    let path = Path::new("resources/day12_example");
    let contents = fs::read_to_string(path).unwrap();

    let mut sum = 0;
    contents.lines().for_each(|line| {
        let (list_str, groups_str) = line.split_once(' ').unwrap();
        let list = list_str
            .chars()
            .into_iter()
            .map(|c| match c {
                '.' => Spring::Operational,
                '#' => Spring::Damaged,
                '?' => Spring::Unknown,
                _ => panic!("unknown character for spring"),
            })
            .collect::<Vec<Spring>>();
        let groups = groups_str
            .split(',')
            .into_iter()
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut list_unfolded = list.clone();
        let mut groups_unfolded = groups.clone();
        for _ in 0..4 {
            list_unfolded.push(Spring::Unknown);
            list_unfolded.append(&mut list.clone());
            groups_unfolded.append(&mut groups.clone());
        }

        let solutions = solve(&list_unfolded, &groups_unfolded);
        println!("solutions={}", solutions);
        sum += solutions;
    });

    println!("Answer: {}", sum);
}
