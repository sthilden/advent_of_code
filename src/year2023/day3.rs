use std::{collections::BTreeMap, fs, path::Path, vec};

fn is_symbol(schematics: &Vec<Vec<char>>, row_inx: usize, col_inx: usize) -> bool {
    let c = schematics[row_inx][col_inx];
    return !c.is_alphanumeric() && c != '.';
}

fn is_part_number(
    schematics: &Vec<Vec<char>>,
    row_inx: usize,
    col_inx_first: usize,
    col_inx_last: usize,
) -> bool {
    // Check row above number
    if row_inx > 0 {
        for col_inx in (col_inx_first as i32 - 1)..(col_inx_last as i32 + 2) {
            if 0 <= col_inx && (col_inx as usize) < schematics[row_inx - 1].len() {
                if is_symbol(&schematics, row_inx - 1, col_inx as usize) {
                    return true;
                }
            }
        }
    }
    // Check row below number
    if row_inx < schematics.len() - 1 {
        for col_inx in (col_inx_first as i32 - 1)..(col_inx_last as i32 + 2) {
            if 0 <= col_inx && (col_inx as usize) < schematics[row_inx + 1].len() {
                if is_symbol(&schematics, row_inx + 1, col_inx as usize) {
                    return true;
                }
            }
        }
    }
    // Check sides
    if col_inx_first > 0 {
        if is_symbol(&schematics, row_inx, col_inx_first - 1) {
            return true;
        }
    }
    if col_inx_last < schematics[row_inx].len() - 1 {
        if is_symbol(&schematics, row_inx, col_inx_last + 1) {
            return true;
        }
    }

    // No symbol found. This is not a part number.
    false
}

fn get_part_number(
    schematics: &Vec<Vec<char>>,
    row_inx: usize,
    col_inx_first: usize,
    col_inx_last: usize,
) -> Option<u32> {
    if is_part_number(&schematics, row_inx, col_inx_first, col_inx_last) {
        let mut s = String::new();
        for col_inx in col_inx_first..col_inx_last + 1 {
            s.push(schematics[row_inx][col_inx]);
        }
        return Some(
            s.parse::<u32>()
                .expect(format!("not a number!? {}", s).as_str()),
        );
    }
    None
}

pub fn problem1() {
    println!("problem 1");

    let path = Path::new("resources/day3_input");
    let contents = fs::read_to_string(path).unwrap();

    let mut sum = 0;

    // Turn into character matrix
    let mut schematics: Vec<Vec<char>> = Vec::new();
    contents
        .lines()
        .into_iter()
        .for_each(|line| schematics.push(line.chars().collect()));

    // Go over matrix
    schematics.iter().enumerate().for_each(|(row_inx, row)| {
        let mut first_inx = None;
        row.into_iter().enumerate().for_each(|(col_inx, c)| {
            // Beginning of number
            if first_inx.is_none() && c.is_alphanumeric() {
                first_inx = Some(col_inx);
            }

            // End of number
            if first_inx.is_some()
                && (!c.is_alphanumeric() || col_inx == schematics[row_inx].len() - 1)
            {
                let mut last_inx = col_inx - 1;
                if c.is_alphanumeric() && col_inx == schematics[row_inx].len() - 1 {
                    last_inx = col_inx;
                }

                // Check if this number is a part number, and add to total if it is
                if let Some(part_number) =
                    get_part_number(&schematics, row_inx, first_inx.unwrap(), last_inx)
                {
                    sum += part_number
                }
                first_inx = None;
            }
        });
    });

    println!("Answer: {}", sum);
}

fn check_for_gear(
    number: u32,
    gear_map: &mut BTreeMap<(usize, usize), (usize, u32)>,
    schematics: &Vec<Vec<char>>,
    row_inx: usize,
    col_inx: usize,
) {
    if schematics[row_inx][col_inx] == '*' {
        // is gear
        let key = (row_inx, col_inx);
        if let Some(val) = gear_map.get_mut(&key) {
            (*val).0 += 1; // count
            (*val).1 *= number; // product
        } else {
            gear_map.insert(key, (1, number));
        }
    }
}

fn check_for_gears(
    gear_map: &mut BTreeMap<(usize, usize), (usize, u32)>,
    schematics: &Vec<Vec<char>>,
    row_inx: usize,
    col_inx_first: usize,
    col_inx_last: usize,
) -> bool {
    // Fetch number
    let mut s = String::new();
    for col_inx in col_inx_first..col_inx_last + 1 {
        s.push(schematics[row_inx][col_inx]);
    }
    let number = s
        .parse::<u32>()
        .expect(format!("not a number!? {}", s).as_str());

    // Check row above number
    if row_inx > 0 {
        for col_inx in (col_inx_first as i32 - 1)..(col_inx_last as i32 + 2) {
            if 0 <= col_inx && (col_inx as usize) < schematics[row_inx - 1].len() {
                check_for_gear(number, gear_map, &schematics, row_inx - 1, col_inx as usize);
            }
        }
    }
    // Check row below number
    if row_inx < schematics.len() - 1 {
        for col_inx in (col_inx_first as i32 - 1)..(col_inx_last as i32 + 2) {
            if 0 <= col_inx && (col_inx as usize) < schematics[row_inx + 1].len() {
                check_for_gear(number, gear_map, &schematics, row_inx + 1, col_inx as usize);
            }
        }
    }
    // Check sides
    if col_inx_first > 0 {
        check_for_gear(number, gear_map, &schematics, row_inx, col_inx_first - 1);
    }
    if col_inx_last < schematics[row_inx].len() - 1 {
        check_for_gear(number, gear_map, &schematics, row_inx, col_inx_last + 1);
    }

    // No symbol found. This is not a part number.
    false
}

pub fn problem2() {
    println!("problem 2");

    let path = Path::new("resources/day3_input");
    let contents = fs::read_to_string(path).unwrap();

    let mut gear_map: BTreeMap<(usize, usize), (usize, u32)> = BTreeMap::new();

    // Turn into character matrix
    let mut schematics: Vec<Vec<char>> = Vec::new();
    contents
        .lines()
        .into_iter()
        .for_each(|line| schematics.push(line.chars().collect()));

    // Go over matrix
    schematics.iter().enumerate().for_each(|(row_inx, row)| {
        let mut first_inx = None;
        row.into_iter().enumerate().for_each(|(col_inx, c)| {
            // Beginning of number
            if first_inx.is_none() && c.is_alphanumeric() {
                first_inx = Some(col_inx);
            }

            // End of number
            if first_inx.is_some()
                && (!c.is_alphanumeric() || col_inx == schematics[row_inx].len() - 1)
            {
                let mut last_inx = col_inx - 1;
                if c.is_alphanumeric() && col_inx == schematics[row_inx].len() - 1 {
                    last_inx = col_inx;
                }

                // Check if this number has an adjacent gear
                check_for_gears(
                    &mut gear_map,
                    &schematics,
                    row_inx,
                    first_inx.unwrap(),
                    last_inx,
                );

                first_inx = None;
            }
        });
    });

    // Go over gear_map
    let mut sum = 0;
    gear_map.into_iter().for_each(|(_, val)| {
        if val.0 == 2 {
            sum += val.1;
        }
    });

    println!("Answer: {}", sum);
}
