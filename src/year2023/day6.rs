use std::{fs, path::Path};

fn compute(time: &Vec<u64>, dist: &Vec<u64>) -> u64 {
    // Let
    //   t:  time race lasts
    //   d:  record distance
    //   tr: time button is pressed in race
    //   dr: distance travelled in race
    // then
    //   dr = (t - tr) * tr
    // and so we are equal to the record if
    //   d  = (t - tr) * tr

    assert_eq!(time.len(), dist.len());
    let mut product = 1;

    for (t, d) in time.iter().zip(dist.iter()) {
        let s = ((*t as f64).powf(2.) - 4. * (*d as f64)).sqrt();
        let t0 = (0.5 * (*t as f64) - 0.5 * s).ceil() as u64;
        let t1 = (0.5 * (*t as f64) + 0.5 * s).floor() as u64;
        let count = t1 - t0 + 1;
        product *= count;
    }

    product
}

pub fn problem1() {
    println!("problem 1");

    let path = Path::new("resources/day6_input");
    let contents = fs::read_to_string(path).unwrap();

    let mut time: Vec<u64> = Vec::new();
    let mut dist: Vec<u64> = Vec::new();

    contents.lines().for_each(|line| {
        let values = line
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split(char::is_whitespace)
            .into_iter()
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<u64>().expect(format!("n=|{}|", n).as_str()))
            .collect::<Vec<u64>>();
        if line.starts_with("Time") {
            time = values;
        } else if line.starts_with("Distance") {
            dist = values;
        }
    });

    let product = compute(&time, &dist);

    println!("Answer: {}", product);
}

pub fn problem2() {
    println!("problem 2");

    let path = Path::new("resources/day6_input");
    let contents = fs::read_to_string(path).unwrap();

    let mut time: Vec<u64> = Vec::new();
    let mut dist: Vec<u64> = Vec::new();

    contents.lines().for_each(|line| {
        let number = line
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .collect::<String>()
            .parse::<u64>()
            .unwrap();

        if line.starts_with("Time") {
            time.push(number);
        } else if line.starts_with("Distance") {
            dist.push(number);
        }
    });

    let product = compute(&time, &dist);

    println!("Answer: {}", product);
}
