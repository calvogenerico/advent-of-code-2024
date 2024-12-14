use std::ops::Range;

pub struct AmphipodDisk {
    data: Vec<Option<usize>>,
    first_free: Option<usize>,
    last_used: usize,
    file_buckets: Vec<Range<usize>>,
    empty_buckets: Vec<Range<usize>>
}

impl AmphipodDisk {
    pub fn new(memory_map: &str) -> AmphipodDisk {
        let mut data = vec![];
        let mut empty_buckets = vec![];
        let mut file_buckets = vec![];
        let mut first_free = None;
        let mut last_used = 0;
        for (index, c) in memory_map.trim().chars().enumerate() {
            if !c.is_ascii_digit() {
                panic!("input should be only numbers")
            }
            let n = c.to_digit(10).unwrap() as usize;

            if index % 2 == 0 {
                let start = data.len();
                for _ in 0..n {
                    data.push(Some(index / 2))
                }
                last_used = data.len() - 1;
                file_buckets.push(start..data.len())
            } else {
                let start = data.len();
                first_free = first_free.or(Some(start));
                for _ in 0..n {
                    data.push(None)
                }
                empty_buckets.push(start..data.len())
            }
        }

        AmphipodDisk {
            data,
            last_used,
            first_free,
            file_buckets ,
            empty_buckets
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

    pub fn compact_defragmented(&mut self) {
        for file in self.file_buckets.iter().rev() {
            let maybe_slot = self.empty_buckets.iter_mut().find(|bucket| {
                bucket.end <= file.start && bucket.len() >= file.len()
            });

            if maybe_slot.is_some() {
                let slot = maybe_slot.unwrap();
                slot.clone().step_by(1).zip(file.clone().step_by(1)).for_each(|(a, b)| {
                    let data = self.data[b].take().unwrap();
                    self.data[a].replace(data);
                });
                let i = file.len();
                slot.start = slot.start + i;
            }
        }

    }

    pub fn checksum(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .map(|(i, cell)| i * cell.unwrap_or(0))
            .sum()
    }
}

fn step1(input: &str) -> usize {
    let mut disk = AmphipodDisk::new(input);
    disk.compact();
    disk.checksum()
}

fn step2(input: &str) -> usize {
    let mut disk = AmphipodDisk::new(input);
    disk.compact_defragmented();
    disk.checksum()
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

        assert_eq!(
            step1(input),
            1928
        )
    }

    #[test]
    fn step_2_simple_case() {
        let input = "13312";
        // 0...111.22
        // -> 022.111.
        // -> 01234567
        // 012345678
        assert_eq!(
            step2(input),
            2 + 4 + 4 + 5 +6
        )
    }

    #[test]
    fn step_2_using_upgraded_size() {
        let input = "13112";
        // 0...1.22
        // -> 0221...
        // -> 01234567
        // 012345678
        assert_eq!(
            step2(input),
            2 + 4 + 3
        )
    }

    #[test]
    fn step_2_provided_example() {
        let input = "2333133121414131402";

        assert_eq!(step2(input), 2858)
    }
}
