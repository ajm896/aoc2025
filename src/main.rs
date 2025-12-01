use std::{fs::read_to_string, path::Path};

use crate::day1::solution;


mod day1;

fn main() {
    let input = load_day("day1_input");
    solution(input);
}

fn load_day(input: &str) -> String {
    let path = Path::new("inputs").join(input);
    let contents = read_to_string(&path).unwrap_or_else(|_| panic!("missing input file: {:?}", path));

    contents
}
