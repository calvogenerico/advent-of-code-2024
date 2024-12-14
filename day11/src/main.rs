use std::collections::{HashMap};

#[derive(PartialEq, Eq, Hash)]
pub struct Stone {
    value: usize
}

pub struct StoneLine {
    stones: HashMap<Stone, usize>
}

fn number_len(mut n: usize) -> usize {
    let mut res = 1;
    while n > 9 {
        res += 1;
        n = n / 10;
    }
    res
}

impl Stone {
    pub fn new(n: usize) -> Stone {
        Stone { value: n }
    }

    pub fn blink(&self) -> Vec<Stone> {
        if self.value == 0 {
            return vec![Stone::new(1)]
        }
        let len = number_len(self.value);
        if len % 2 == 0 {
            let string = format!("{}", self.value);
            let first_half = &string[0..len / 2];
            let second_half = &string[len / 2..];
            return vec![Stone::new(first_half.parse().unwrap()), Stone::new(second_half.parse().unwrap())]
        }
        vec![Stone::new(self.value * 2024)]
    }
}

impl StoneLine {
    pub fn from_str(input: &str) -> StoneLine {
        let stones = input.trim().split(" ").map(|chunk| chunk.parse::<usize>().unwrap() )
            .map(Stone::new).collect::<Vec<Stone>>();

        let mut stone_map = HashMap::new();
        for stone in stones {
            stone_map.insert(stone, 1);
        }

        StoneLine {
            stones: stone_map
        }
    }

    pub fn blink(&mut self, times: usize) {
        for _ in 0..times {
            let mut new_map = HashMap::new();

            for (stone, amount) in self.stones.iter() {
                let blinked = stone.blink();
                for new_stone in blinked {
                    let old = new_map.get(&new_stone).cloned().unwrap_or(0);
                    new_map.insert(new_stone, old + amount);
                }
            }

            self.stones = new_map;
        }
    }

    pub fn count(&self) -> usize {
        self.stones.values().sum()
    }
}

fn step1(input: &str, blinks: usize) -> usize {
    let mut line = StoneLine::from_str(input);
    line.blink(blinks);
    line.count()
}

// fn step2(input: &str) -> usize {
//     input.len()
// }


fn main() {
    let input = include_str!("../input.txt");
    let res1 = step1(input, 25);
    let res2 = step1(input, 75);
    println!("step1: {}", res1);
    println!("step2: {}", res2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stone_0() {
        let input = "0";
        assert_eq!(step1(input, 1), 1)
    }

    #[test]
    fn test_stone_even_digits() {
        let input = "12";
        assert_eq!(step1(input, 1), 2)
    }

    #[test]
    fn test_stone_odd_digits() {
        let input = "1";
        assert_eq!(step1(input, 1), 1)
    }

    #[test]
    fn test_stone_0_3_times() {
        let input = "0";
        assert_eq!(step1(input, 3), 2)
    }

    #[test]
    fn test_provided_example() {
        let input = "125 17";
        assert_eq!(step1(input, 25), 55312)
    }
}