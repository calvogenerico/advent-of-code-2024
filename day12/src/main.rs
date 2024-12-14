type Position = (usize, usize);

pub struct Garden {
    plants: Vec<Vec<char>>,
    processed: Vec<Vec<bool>>,
}

impl Garden {
    pub fn from_str(input: &str) -> Garden {
        let plants: Vec<Vec<char>> = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().collect())
            .collect();

        ;

        let processed = plants.iter().map(|row| row.iter().map(|_| false)).collect();

        Garden { processed, plants }
    }
}

fn step1(input: &str) -> usize {
    input.len()
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
    use text_block_macros::text_block_fnl;

    #[test]
    fn test_01() {
        let input = text_block_fnl!("A");
        assert_eq!(step1(input), 4)
    }
}
