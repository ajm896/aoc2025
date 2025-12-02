extern crate regex;

#[derive(PartialEq, Eq, Debug)]
enum Turn {
    Left(isize),
    Right(isize),
}

impl Turn {
    fn val(&self) -> isize {
        match self {
            Self::Left(x) | Self::Right(x) => *x
        }
    }
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
        let matcher = regex::Regex::new(r"([L|R])(\d+)").unwrap();
        let caps = matcher
            .captures(line)
            .unwrap_or_else(|| panic!("invalid turn: {}", line));
        let dir = caps.get(1).unwrap().as_str();
        let num: isize = caps.get(2).unwrap().as_str().parse().unwrap();
        match dir {
            "R" => Self::Right(num),
            "L" => Self::Left(num),
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Lock {
    counts: [usize; 100],
    current: isize,
}

impl Lock {
    pub fn new(start_num: isize) -> Self {
        let counts = [0; 100];
        let current = start_num;
        Lock { counts, current }
    }

    pub fn default() -> Self {
        Self::new(50)
    }

    #[allow(dead_code)]
    fn enter_turn(&mut self, turn: Turn) {
        //println!("Next Turn:{:?},Curr: {}", turn, self.current);
        let zeros = (turn.val() as f32/100.).floor();
        let prev = self.current;
        match turn {
            Turn::Left(x) => {
                let step = x % 100;
                self.current = (self.current + 100 - step) % 100;
                if self.current >= prev && self.current != 0 { self.counts[0] += 1;}
            }
            Turn::Right(x) => {
                let step = x % 100;
                self.current = (self.current + step) % 100;
                if self.current <= prev  && self.current != 0 { self.counts[0] += 1;}
            } 
        }
        self.counts[self.current as usize] += 1;
        self.counts[0] += zeros as usize;
    }
}

pub fn solution(input: String, lock: &mut Lock) -> usize {
    let combo = Combo::new(input);
    
    for turn in combo.combo {
        lock.enter_turn(turn);
    }
    lock.counts[0]
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
                counts: [0; 100],
                current: 50,
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
        assert_eq!(lock.current, 51);
    }

    #[test]
    fn test_turn_right_rollover() {
        let mut lock = Lock::new(99);
        lock.enter_turn(Turn::Right(2));
        assert_eq!(lock.current, 1);
    }

    #[test]
    fn test_turn_left() {
        let mut lock = Lock::default();
        lock.enter_turn(Turn::Left(1));
        assert_eq!(lock.current, 49);
    }

    #[test]
    fn test_combo_seq() {
        let mut lock = Lock::new(50);
        let combo = Combo::new(TEST_INPUT.to_string());
        for turn in combo.combo {
            lock.enter_turn(turn);
        }
        assert_eq!(lock.current, 32);
    }

    #[test]
    fn test_provided_case() {
        let mut lock = Lock::new(50);
        let sol = solution(TEST_INPUT.to_string(), &mut lock);
        assert_eq!(sol, 6);
    }

    #[test]
    fn test_three_digit_turn() {
        let mut lock = Lock::default();
        lock.enter_turn(Turn::Left(500));
        assert_eq!(lock.current, 50);
    }

    #[test]
    fn test_three_digit_turn_solution() {
        let mut lock = Lock::default();
        let sol = solution("L550".to_string(), &mut lock);
        assert_eq!(sol, 6);
    }
}
