use std::collections::HashSet;

pub enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Clone)]
pub enum Terrain {
    Free,
    Obstacle,
}

type Point = (isize, isize);

pub struct Guard {
    position: Point,
    direction: Direction,
}

pub struct OfficeWithAGuard {
    guard: Guard,
    map: Vec<Vec<Terrain>>,
    path: HashSet<Point>,
}

impl OfficeWithAGuard {
    pub fn from_str(input: &str) -> OfficeWithAGuard {
        let mut map = vec![];
        let mut guard: Option<Guard> = None;

        input
            .lines()
            .filter(|l| !l.is_empty())
            .enumerate()
            .for_each(|(row_number, line)| {
                let mut row = vec![];
                line.trim()
                    .chars()
                    .enumerate()
                    .for_each(|(column_number, cell)| match cell {
                        '.' => row.push(Terrain::Free),
                        '#' => row.push(Terrain::Obstacle),
                        '^' => {
                            row.push(Terrain::Free);
                            guard
                                .replace(Guard::new(Direction::North, (row_number.try_into().unwrap(), column_number.try_into().unwrap())));
                        }
                        _ => panic!("unknown cell"),
                    });
                map.push(row);
            });

        let guard = guard.unwrap();
        let path = HashSet::new();
        OfficeWithAGuard {
            map,
            guard,
            path,
        }
    }

    pub fn patrol(&mut self) -> usize {
        while self.guard_is_inside() {
            self.path.insert(self.guard.position.clone());
            self.guard_step();
        }
        self.path.len()
    }

    fn guard_is_inside(&self) -> bool {
        let (row, column) = self.guard.position;
        if row < 0 || column < 0 {
            return false
        }

        let row = usize::try_from(row).unwrap();
        let column = usize::try_from(column).unwrap();

        let option = self.map.get(row).and_then(|row| {
            row.get(column)
        });
        option.is_some()
    }

    fn guard_step(&mut self) {
        while !self.guard_can_move() {
            self.guard.rotate()
        }
        self.guard.advance();
    }

    fn guard_can_move(&self) -> bool {
        !self.is_obstacle_at(self.guard.next_position())
    }

    fn is_obstacle_at(&self, (row, column): Point) -> bool {
        if row < 0 || column < 0 {
            return false
        }

        let row = usize::try_from(row).unwrap();
        let column = usize::try_from(column).unwrap();

        self.map
            .get(row)
            .and_then(|row| row.get(column))
            .map(|t| match t {
                Terrain::Obstacle => true,
                _ => false,
            })
            .unwrap_or(false)
    }
}

impl Guard {
    pub fn new(direction: Direction, position: Point) -> Guard {
        Guard {
            direction,
            position,
        }
    }

    pub fn next_position(&self) -> Point {
        let (row, column) = self.position;
        match self.direction {
            Direction::North => (row - 1, column),
            Direction::East => (row, column + 1),
            Direction::South => (row + 1, column),
            Direction::West => (row, column - 1),
        }
    }

    pub fn rotate(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn advance(&mut self) {
        self.position = self.next_position()
    }
}

fn step1(input: &str) -> usize {
    let mut office = OfficeWithAGuard::from_str(input);
    office.patrol()
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
        let input = "#\n^\n";
        assert_eq!(step1(input), 2);
    }

    #[test]
    fn test_02() {
        let input = concat!(
            "#..\n",
            "^.#\n",
            ".#.\n"
        );
        assert_eq!(step1(input), 2);
    }
}
