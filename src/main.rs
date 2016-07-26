use std::fs::File;
use std::io::{BufReader, BufRead};
use std::default::Default;

trait ArbitraryBucket {
    fn filename() -> &'static str;
    fn parse_num(s: &str) -> i32;
    fn is_valid_row(row: &Vec<String>) -> bool;
    fn indices_to_compare() -> (usize, usize);
    fn identifier_index() -> usize;
}

struct Part1;

impl ArbitraryBucket for Part1 {
    fn filename() -> &'static str {
        "weather.dat"
    }
    
    fn parse_num(s: &str) -> i32 {
        if s.ends_with("*") {
            return s[..s.len() - 1].parse::<i32>().unwrap();
        }
        s.parse::<i32>().unwrap()
    }

    fn is_valid_row(row: &Vec<String>) -> bool {
        row.len() > 3 && row[0].parse::<i32>().is_ok()
    }

    fn indices_to_compare() -> (usize, usize) {
        (1, 2)
    }

    fn identifier_index() -> usize {
        0
    }
}

struct Part2;

impl ArbitraryBucket for Part2 {
    fn filename() -> &'static str {
        "football.dat"
    }
    
    fn parse_num(s: &str) -> i32 {
        s.parse::<i32>().unwrap()
    }

    fn is_valid_row(row: &Vec<String>) -> bool {
        row.len() == 10
    }

    fn indices_to_compare() -> (usize, usize) {
        (6, 8)
    }

    fn identifier_index() -> usize {
        1
    }
}

fn process_row(s: String) -> Vec<String> {
    s.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()
}

// Since everything on the trait is static, we don't actually care what the object
// is called.  So we use _.
fn driver<T: ArbitraryBucket>(_: T) {
    let f = File::open(T::filename()).unwrap();
    let f = BufReader::new(f);
    let mut minimal_difference_identifier: String = Default::default();
    let mut minimal_difference = std::i32::MAX;

    for line in f.lines().map(|l| process_row(l.unwrap())).filter(T::is_valid_row) {
        let (col1, col2) = T::indices_to_compare();
        let (x, y) = (T::parse_num(&line[col1]), T::parse_num(&line[col2]));
        let difference = (x - y).abs();
        if difference < minimal_difference {
            minimal_difference = difference;
            minimal_difference_identifier = line[T::identifier_index()].clone();
        }
    }

    println!("{}", minimal_difference_identifier);
}

fn main() {
    driver(Part1);
    driver(Part2);
}
