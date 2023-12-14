use std::{fs, path::Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Item {
    Ash,
    Rock,
}

#[derive(Debug, Clone)]
struct Pattern {
    rows: Vec<Vec<Item>>,
    columns: Vec<Vec<Item>>,
}

impl Pattern {
    pub fn new() -> Self {
        Pattern {
            rows: Vec::new(),
            columns: Vec::new(),
        }
    }
}

fn find_symmtery_one_dimension(items: &Vec<Vec<Item>>, avoid: Option<usize>) -> usize {
    let mut split_size = 0;
    for split_inx in 0..items.len() - 1 {
        if let Some(avoid_inx) = avoid {
            if avoid_inx == split_inx {
                continue;
            }
        }

        // Split between split_inx and split_inx + 1
        let mut left_inx = split_inx;
        let mut right_inx = split_inx + 1;
        loop {
            if items[left_inx] != items[right_inx] {
                break;
            }
            if left_inx == 0 || right_inx == items.len() - 1 {
                split_size = std::cmp::max(split_size, split_inx + 1);
                break;
            }
            left_inx -= 1;
            right_inx += 1;
        }
    }
    split_size
}

fn find_symmetry(pattern: &Pattern) -> usize {
    let row_split_size = find_symmtery_one_dimension(&pattern.rows, None);
    if row_split_size > 0 {
        return 100 * row_split_size;
    }
    let col_split_size = find_symmtery_one_dimension(&pattern.columns, None);
    if col_split_size > 0 {
        return col_split_size;
    }
    0
}

fn swapItem(item: Item) -> Item {
    match item {
        Item::Ash => Item::Rock,
        Item::Rock => Item::Ash,
    }
}

fn find_smudged_symmetry(pattern: &Pattern) -> usize {
    let original_row_split_size = find_symmtery_one_dimension(&pattern.rows, None);
    let original_col_split_size = find_symmtery_one_dimension(&pattern.columns, None);

    let row_avoid = if original_row_split_size > 0 {
        Some(original_row_split_size - 1)
    } else {
        None
    };
    let col_avoid = if original_col_split_size > 0 {
        Some(original_col_split_size - 1)
    } else {
        None
    };

    for i in 0..pattern.rows.len() {
        for j in 0..pattern.columns.len() {
            let mut smudged_pattern: Pattern = pattern.clone();
            smudged_pattern.rows[i][j] = swapItem(smudged_pattern.rows[i][j]);
            smudged_pattern.columns[j][i] = swapItem(smudged_pattern.columns[j][i]);

            assert_eq!(smudged_pattern.rows[i][j], smudged_pattern.columns[j][i]);

            let row_split_size = find_symmtery_one_dimension(&smudged_pattern.rows, row_avoid);
            if row_split_size > 0 {
                return 100 * row_split_size;
            }
            let col_split_size = find_symmtery_one_dimension(&smudged_pattern.columns, col_avoid);
            if col_split_size > 0 {
                return col_split_size;
            }
        }
    }
    0
}

fn read_input() -> Vec<Pattern> {
    let path = Path::new("resources/day13_input");
    let contents = fs::read_to_string(path).unwrap();

    let mut patterns: Vec<Pattern> = Vec::new();
    let mut current_pattern: Option<Pattern> = None;

    // Read file into pattern rows
    contents.lines().for_each(|line| {
        if line.is_empty() {
            if let Some(pattern) = current_pattern.take() {
                patterns.push(pattern);
            }
        } else {
            if current_pattern.is_none() {
                current_pattern = Some(Pattern::new());
            }
            if let Some(pattern) = &mut current_pattern {
                pattern.rows.push(
                    line.chars()
                        .into_iter()
                        .map(|c| match c {
                            '.' => Item::Ash,
                            '#' => Item::Rock,
                            _ => panic!("Unknown item"),
                        })
                        .collect::<Vec<Item>>(),
                );
            }
        }
    });

    // Add last pattern row
    if let Some(pattern) = current_pattern.take() {
        patterns.push(pattern);
    }

    // Make columns
    patterns.iter_mut().for_each(|pattern| {
        if pattern.rows.is_empty() {
            return;
        }
        let nrows = pattern.rows.len();
        let ncols = pattern.rows.first().unwrap().len();
        for j in 0..ncols {
            let mut column = Vec::new();
            for i in 0..nrows {
                column.push(pattern.rows[i][j]);
            }
            pattern.columns.push(column);
        }
    });

    patterns
}

pub fn problem1() {
    println!("problem 1");

    let patterns = read_input();
    let sum: usize = patterns.iter().map(|p| find_symmetry(p)).sum();

    println!("Answer: {}", sum);
}

pub fn problem2() {
    println!("problem 2");

    let patterns = read_input();
    let sum: usize = patterns.iter().map(|p| find_smudged_symmetry(p)).sum();

    println!("Answer: {}", sum);
}
