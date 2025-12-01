extern crate regex;

#[derive(PartialEq, Eq, Debug)]
enum Turn {
    Left(u8), Right(u8)
}

#[derive(PartialEq, Eq, Debug)]
struct Combo {
    combo: Vec<Turn>,
}

impl Combo {
    fn new(input: String) -> Self {
        let combo = input.lines().map(|line| Turn::new(line)).collect();
        Self {
            combo
        }
    }
}

impl Turn {
    fn new(line: &str) -> Self {
        let matcher = regex::Regex::new(r"([L|R])(\d{1,2})").unwrap();
        let caps = matcher
            .captures(line)
            .unwrap_or_else(|| panic!("invalid turn: {}", line));
        let dir = caps.get(1).unwrap().as_str();
        let num: u8 = caps.get(2).unwrap().as_str().parse().unwrap();
        match dir {
            "R" => Self::Right(num),
            "L" => Self::Left(num),
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Lock {
    counts: [u8; 99],
    current: u8,
}

impl Lock {
    fn new(start_num: u8) -> Self {
        let counts = [0; 99];
        let current = start_num;
        Lock {
            counts,
            current,
        }
    }

    fn default() -> Self {
        Self::new(0)
    }

    fn enter_turn(&mut self, turn: Turn) {
        match turn {
            Turn::Left(x) => self.current = self.current - x,
            Turn::Right(x) => self.current = self.current + x,
        }
    }
}

pub fn solution(input: String) -> u8 {
    let combo = Combo::new(input);
    println!("{:?}", combo);
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_lock() {
        assert_eq!(Lock::default(), Lock {
            counts: [0; 99],
            current: 0,
        })
    } 

    #[test]
    fn test_make_turn() {
        assert_eq!(Turn::Right(10), Turn::new("R10"));
        assert_eq!(Turn::Left(10), Turn::new("L10"));
    }
}
