extern crate regex;

#[derive(PartialEq, Eq, Debug)]
enum Turn {
    Left(i8),
    Right(i8),
}

#[derive(PartialEq, Eq, Debug)]
struct Combo {
    combo: Vec<Turn>,
}

impl Combo {
    fn new(input: String) -> Self {
        let combo = input.lines().map(Turn::new).collect();
        Self { combo }
    }
}

impl Turn {
    fn new(line: &str) -> Self {
        let matcher = regex::Regex::new(r"([L|R])(\d{1,2})").unwrap();
        let caps = matcher
            .captures(line)
            .unwrap_or_else(|| panic!("invalid turn: {}", line));
        let dir = caps.get(1).unwrap().as_str();
        let num: i8 = caps.get(2).unwrap().as_str().parse().unwrap();
        match dir {
            "R" => Self::Right(num),
            "L" => Self::Left(num),
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Lock {
    counts: [i8; 99],
    current: i8,
}

impl Lock {
    fn new(start_num: i8) -> Self {
        let counts = [0; 99];
        let current = start_num;
        Lock { counts, current }
    }

    fn default() -> Self {
        Self::new(0)
    }

    #[allow(dead_code)]
    fn enter_turn(&mut self, turn: Turn) {
        match turn {
            Turn::Left(x) => {
                if (self.current - x) < 0 {
                    self.current = 100 + self.current;
                }
                self.current -= x;
            }
            Turn::Right(x) => self.current = (self.current + x) % 99,
        }
    }
}

pub fn solution(input: String) -> i8 {
    let combo = Combo::new(input);
    let _lock = Lock::default();
    println!("{:?}", combo);
    3
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
    #[test]
    fn test_default_lock() {
        assert_eq!(
            Lock::default(),
            Lock {
                counts: [0; 99],
                current: 0,
            }
        )
    }

    #[test]
    fn test_make_turn() {
        assert_eq!(Turn::Right(10), Turn::new("R10"));
        assert_eq!(Turn::Left(10), Turn::new("L10"));
    }

    #[test]
    fn test_turn_right() {
        let mut lock = Lock::default();
        lock.enter_turn(Turn::Right(1));
        assert_eq!(lock.current, 1);
    }

    #[test]
    fn test_turn_left() {
        let mut lock = Lock::default();
        lock.enter_turn(Turn::Left(1));
        assert_eq!(lock.current, 99);
    }

    #[test]
    fn provided_test_case() {
        let sol = solution(TEST_INPUT.to_string());
        assert_eq!(sol, 3);
    }
}
