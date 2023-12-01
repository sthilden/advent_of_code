use std::{fs, path::Path};

pub fn problem1() {
    println!("problem 1");

    let path = Path::new("resources/day1_input");
    let contents = fs::read_to_string(path).unwrap();
    let mut sum = 0;

    contents.lines().into_iter().for_each(|line| {
        if let Some(first) = line.find(char::is_numeric) {
            if let Some(last) = line.rfind(char::is_numeric) {
                sum += line.chars().nth(first).unwrap().to_digit(10).unwrap() * 10
                    + line.chars().nth(last).unwrap().to_digit(10).unwrap();
            }
        }
    });

    println!("Answer: {}", sum);
}

pub fn problem2() {
    println!("problem 2");

    let path = Path::new("resources/day1_input");
    let contents = fs::read_to_string(path).unwrap();

    let text_numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut sum = 0;

    contents.lines().into_iter().for_each(|line| {
        let mut first = None;
        if let Some(line_inx) = line.find(char::is_numeric) {
            first = Some((
                line_inx,
                line.chars().nth(line_inx).unwrap().to_digit(10).unwrap(),
            ));
        }
        text_numbers
            .iter()
            .enumerate()
            .for_each(|(vec_inx, text_num)| {
                if let Some(line_inx) = line.find(*text_num) {
                    if first.is_none() || line_inx < first.unwrap().0 {
                        first = Some((line_inx, (vec_inx + 1) as u32));
                    }
                }
            });
        let first_num = first.unwrap().1;

        let mut last = None;
        if let Some(line_inx) = line.rfind(char::is_numeric) {
            last = Some((
                line_inx,
                line.chars().nth(line_inx).unwrap().to_digit(10).unwrap(),
            ));
        }
        text_numbers
            .iter()
            .enumerate()
            .for_each(|(vec_inx, text_num)| {
                if let Some(line_inx) = line.rfind(*text_num) {
                    if last.is_none() || line_inx > last.unwrap().0 {
                        last = Some((line_inx, (vec_inx + 1) as u32));
                    }
                }
            });
        let last_num = last.unwrap().1;

        //println!("line: {}, first: {}, last: {}", line, first_num, last_num);

        sum += first_num * 10 + last_num;
    });

    println!("Answer: {}", sum);
}
