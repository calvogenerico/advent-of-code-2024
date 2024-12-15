use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Position(isize, isize);

impl Position {
    pub fn neighbors(&self) -> Vec<Position> {
        vec![
            Position(self.0 + 1, self.1),
            Position(self.0, self.1 + 1),
            Position(self.0 - 1, self.1),
            Position(self.0, self.1 - 1)
        ]
    }
}

pub struct Garden {
    plants: Vec<Vec<char>>,
    processed: Vec<Vec<bool>>
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
                let position = Position(row_n as isize, column_n as isize);
                let vegetable = self.at(&position);
                let (a, b) = self.calculate_cost_from(&position, vegetable);
                total += a * b;
            }
        }
        total
    }

    pub fn fence_cost_with_discount(&mut self) -> usize {
        let mut total = 0;
        for row_n in 0..self.plants.len() {
            for column_n in 0..self.plants[row_n].len() {
                let position = Position(row_n as isize, column_n as isize);
                let vegetable = self.at(&position);
                let (a, b) = self.calculate_cost_with_discount_from(&position, vegetable);
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
            .chain([(0, 1usize)])
            .reduce(|(a1, a2), (b1, b2)| (a1 + b1, a2 + b2))
            .unwrap()
    }

    // (perimeter, area)
    fn calculate_cost_with_discount_from(&mut self, from: &Position, vegetable: char) -> (usize, usize) {
        let tiles = self.get_tiles(from, vegetable);
        let area = tiles.len();
        let mut all_phantoms: HashSet<Position> = HashSet::new();

        let mut cheap_perimeter = 0;

        for tile in &tiles {
            let all_neighbors = tile.neighbors();
            let edges = all_neighbors.iter().filter(|n| !tiles.contains(n)).collect::<Vec<_>>();

            if edges.len() == 3 {
                cheap_perimeter += 2;
                continue
            }

            if edges.len() == 2 && !(edges[0].0 == edges[1].0 || edges[0].1 == edges[1].1) {
                cheap_perimeter += 1;
                continue
            }

            let phantoms = all_neighbors.iter().filter(|n| !tiles.contains(n)).collect::<Vec<_>>();
            for phantom in phantoms {
                if all_phantoms.get(phantom).is_some() {
                    continue
                }
                all_phantoms.insert(phantom.clone());

                let phantom_neighbors = phantom.neighbors();
                let overlapping_phantom_neighors = phantom_neighbors.iter().filter(|n| tiles.contains(n)).collect::<Vec<_>>();

                if overlapping_phantom_neighors.len() == 3 {
                    cheap_perimeter += 2;
                }

                if overlapping_phantom_neighors.len() == 2 && !(overlapping_phantom_neighors[0].0 == overlapping_phantom_neighors[1].0 || overlapping_phantom_neighors[0].1 == overlapping_phantom_neighors[1].1) {
                    cheap_perimeter += 1;
                }
            }

        }

        // let neighbors = from.neighbors();
        // let edges = neighbors.iter()
        //     .filter(|n| !self.is_at(n, vegetable))
        //     .count() + 4 - neighbors.len();

        // let filter = neighbors.iter()
        //     .filter(|n| self.is_at(n, vegetable))
        //     .collect::<Vec<_>>();
        // let (perimeter, area) = filter.iter()
        //     .map(|n| self.calculate_cost_with_discount_from(n, vegetable))
        //     .reduce(|(a1, a2), (b1, b2)| (a1 + b1, a2 + b2)).unwrap_or((0, 0));

        (cheap_perimeter, area)
    }

    fn get_tiles(&mut self, start: &Position, vegetable: char) -> Vec<Position> {
        let mut set = vec![];
        if self.was_processed(start) {
            return set
        }

        self.get_tiles_aux(start, vegetable, &mut set);
        set
    }

    fn get_tiles_aux(&mut self, start: &Position, vegetable: char, state: &mut Vec<Position>) {
        if self.was_processed(start) {
            return;
        }
        if !self.is_at(start, vegetable) {
            return;
        }

        state.push(start.clone());
        self.mark_processed(start);
        start.neighbors().iter().for_each(|n| {
            self.get_tiles_aux(n, vegetable, state)
        })
    }


    fn mark_processed(&mut self, pos: &Position) {
        self.processed[pos.0 as usize][pos.1 as usize] = true
    }

    fn was_processed(&self, pos: &Position) -> bool {
        self.processed
            .get(pos.0 as usize)
            .and_then(|r| r.get(pos.1 as usize))
            .cloned()
            .unwrap_or(true)
    }

    fn at(&self, pos: &Position) -> char {
        self.plants[pos.0 as usize][pos.1 as usize]
    }

    fn is_at(&self, pos: &Position, vegatable: char) -> bool {
        self.plants
            .get(pos.0 as usize)
            .and_then(|r| r.get(pos.1 as usize))
            .is_some_and(|v| *v == vegatable)
    }
}

fn step1(input: &str) -> usize {
    let mut garden = Garden::from_str(input);
    garden.fence_cost()
}

fn step2(input: &str) -> usize {
    let mut garden = Garden::from_str(input);
    garden.fence_cost_with_discount()
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

    #[test]
    fn step_2_given_example() {
        let input = text_block_fnl!(
            "AAAAAA"
            "AAABBA"
            "AAABBA"
            "ABBAAA"
            "ABBAAA"
            "AAAAAA"
        );
        assert_eq!(step2(input), 368)
    }

    #[test]
    fn step_2_test_01() {
        let input = text_block_fnl!(
            "AA"
        );
        assert_eq!(step2(input), 8)
    }

    #[test]
    fn step_2_test_02() {
        let input = text_block_fnl!(
            "EEEEE"
            "EXXXX"
            "EEEEE"
            "EXXXX"
            "EEEEE"
        );
        assert_eq!(step2(input), 236)
    }

    #[test]
    fn step_2_given_larger_example() {
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
        assert_eq!(step2(input), 1930)
    }
}
