use std::fs::File;
use std::io::{BufReader, BufRead};

fn parse_temp(s: &str) -> i32 {
    if s.ends_with("*") {
        return s[..s.len() - 1].parse::<i32>().unwrap();
    }
    s.parse::<i32>().unwrap()
}

fn is_day_row(row: &Vec<String>) -> bool {
    row.len() > 3 && row[0].parse::<i32>().is_ok()
}

fn process_row(s: String) -> Vec<String> {
    s.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()
}

fn part1() {
    let f = File::open("weather.dat").unwrap();
    let f = BufReader::new(f);
    let mut minimal_spread_day = -1;
    let mut minimal_spread = std::i32::MAX;

    for line in f.lines().map(|l| process_row(l.unwrap())).filter(is_day_row) {
        let high = parse_temp(&line[1]);
        let low = parse_temp(&line[2]);
        let spread = high - low;
        if spread < minimal_spread {
            minimal_spread = spread;
            minimal_spread_day = line[0].parse::<i32>().unwrap();
        }
    }

    println!("{}", minimal_spread_day);
}

fn main() {
    part1();
}
