struct Position(usize, usize);

impl Position {
    pub fn neighbors(&self) -> Vec<Position> {
        let mut vec = vec![Position(self.0 + 1, self.1), Position(self.0, self.1 + 1)];

        if self.0 > 0 {
            vec.push(Position(self.0 - 1, self.1))
        }
        if self.1 > 0 {
            vec.push(Position(self.0, self.1 - 1))
        }

        vec
    }
}

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

        let processed = plants
            .iter()
            .map(|row| row.iter().map(|_| false).collect())
            .collect();

        Garden { processed, plants }
    }

    pub fn fence_cost(&mut self) -> usize {
        let mut total = 0;
        for row_n in 0..self.plants.len() {
            for column_n in 0..self.plants[row_n].len() {
                let position = Position(row_n, column_n);
                let vegetable = self.at(&position);
                let (a, b) = self.calculate_cost_from(&position, vegetable);
                total += a * b;
            }
        }
        total
    }

    // (perimeter, area)
    fn calculate_cost_from(&mut self, from: &Position, vegetable: char) -> (usize, usize) {
        if !self.is_at(from, vegetable) {
            return (1, 0);
        }

        if self.was_processed(from) {
            return (0, 0);
        }

        self.mark_processed(from);

        let neighbors = from.neighbors();
        neighbors
            .iter()
            .map(|n| self.calculate_cost_from(n, vegetable))
            .chain([(0usize + 4 - neighbors.len(), 1usize)])
            .reduce(|(a1, a2), (b1, b2)| (a1 + b1, a2 + b2))
            .unwrap()
    }

    fn mark_processed(&mut self, pos: &Position) {
        self.processed[pos.0][pos.1] = true
    }

    fn was_processed(&self, pos: &Position) -> bool {
        self.processed
            .get(pos.0)
            .and_then(|r| r.get(pos.1))
            .cloned()
            .unwrap_or(true)
    }

    fn at(&self, pos: &Position) -> char {
        self.plants[pos.0][pos.1]
    }

    fn is_at(&self, pos: &Position, vegatable: char) -> bool {
        self.plants
            .get(pos.0)
            .and_then(|r| r.get(pos.1))
            .is_some_and(|v| *v == vegatable)
    }
}

fn step1(input: &str) -> usize {
    let mut garden = Garden::from_str(input);
    garden.fence_cost()
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

    #[test]
    fn test_02() {
        let input = text_block_fnl!("AB");
        assert_eq!(step1(input), 8)
    }

    #[test]
    fn does_not_entry_twice_in_the_same_field() {
        let input = text_block_fnl!(
            "AB"
            "AB"
        );
        assert_eq!(step1(input), 6 * 2 * 2)
    }

    #[test]
    fn given_example() {
        let input = text_block_fnl!(
            "RRRRIICCFF"
            "RRRRIICCCF"
            "VVRRRCCFFF"
            "VVRCCCJFFF"
            "VVVVCJJCFE"
            "VVIVCCJJEE"
            "VVIIICJJEE"
            "MIIIIIJJEE"
            "MIIISIJEEE"
            "MMMISSJEEE"
        );
        assert_eq!(step1(input), 1930)
    }
}
