use std::{fs::read_to_string, path::Path};

mod day1;

fn main() {
    let input = load_day("day1");
    let mut lock = day1::Lock::default();
    let ans = day1::solution(input, &mut lock);
    println!("{}", ans);
}

fn load_day(input: &str) -> String {
    let path = Path::new("inputs").join(input);

    read_to_string(&path).unwrap_or_else(|_| panic!("missing input file: {:?}", path))
}
