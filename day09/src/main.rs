pub struct AmphipodDisk {
    data: Vec<Option<usize>>,
    first_free: Option<usize>,
    last_used: usize,
}

impl AmphipodDisk {
    pub fn new(memory_map: &str) -> AmphipodDisk {
        let mut data = vec![];
        let mut first_free = None;
        let mut last_used = 0;
        for (index, c) in memory_map.trim().chars().enumerate() {
            if !c.is_ascii_digit() {
                panic!("input shuld be only numbers")
            }
            let n = c.to_digit(10).unwrap() as usize;

            if index % 2 == 0 {
                for _ in 0..n {
                    data.push(Some(index / 2))
                }
                last_used = data.len() - 1;
            } else {
                first_free = first_free.or(Some(data.len()));
                for _ in 0..n {
                    data.push(None)
                }
            }
        }

        AmphipodDisk {
            data,
            last_used,
            first_free,
        }
    }

    pub fn compact(&mut self) {
        while self.first_free.is_some_and(|first| first < self.last_used) {
            let mut first_free = self.first_free.unwrap();
            let value = self.data[self.last_used].take();
            self.data[first_free].replace(value.unwrap());
            while self.data[self.last_used].is_none() {
                self.last_used -= 1;
            }

            while self.data.get(first_free).is_some_and(|d| d.is_some()) {
                first_free += 1;
            }
            self.first_free.replace(first_free);
        }
    }

    pub fn checksum(&self) -> usize {
        self.data
            .iter()
            .filter(|cell| cell.is_some())
            .enumerate()
            .map(|(i, cell)| i * cell.unwrap())
            .sum()
    }
}

fn step1(input: &str) -> usize {
    let mut disk = AmphipodDisk::new(input);
    disk.compact();
    disk.checksum()
}

fn step2(input: &str) -> usize {
    input.len()
}

fn main() {
    let input = include_str!("../input.txt");
    let res1 = step1(input);
    let res2 = step2(input);
    println!("step1: {}", res1);
    println!("step2: {}", res2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let input = "11";
        // 0. <- already ordered
        // output 0 * 0 = 0
        assert_eq!(step1(input), 0)
    }

    #[test]
    fn test_02() {
        let input = "1234";
        // 0..111....
        // 0111......
        assert_eq!(step1(input), 0 + 1 + 2 + 3)
    }

    #[test]
    fn test_03() {
        let input = "12345";
        // 0..111....22222
        // 022111222
        // 012345678
        assert_eq!(
            step1(input),
            0*0 + 1 * 2 + 2 * 2 + 3 * 1 + 4 * 1 + 5*1 + 6 * 2 + 7 * 2 + 8 * 2
        )
    }

    #[test]
    fn provided_small_scenario() {
        let input = "2333133121414131402";
        // 0..111....22222
        // 022111222
        // 012345678
        assert_eq!(
            step1(input),
            1928
        )
    }
}
