use std::collections::{HashMap};

#[derive(Clone, PartialEq)]
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

#[derive(Clone)]
pub struct Guard {
    position: Point,
    direction: Direction,
}

pub struct OfficeWithAGuard {
    guard: Guard,
    map: Vec<Vec<Terrain>>,
    path: HashMap<Point, Vec<Direction>>
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
                            guard.replace(Guard::new(
                                Direction::North,
                                (
                                    row_number.try_into().unwrap(),
                                    column_number.try_into().unwrap(),
                                ),
                            ));
                        }
                        _ => panic!("unknown cell"),
                    });
                map.push(row);
            });

        let guard = guard.unwrap();
        let path = HashMap::new();
        OfficeWithAGuard { map, guard, path }
    }

    pub fn patrol(&mut self) -> usize {
        while self.guard_is_inside() {
            self.register_step();
            self.guard_step();
        }
        self.path.len()
    }

    fn register_step(&mut self) {
        let value = self.path.get_mut(&self.guard.position);
        if value.is_some() {
            value.unwrap().push(self.guard.direction.clone());
        } else {
            self.path.insert(self.guard.position.clone(), vec![self.guard.direction.clone()]);
        }
    }

    fn guard_is_inside(&self) -> bool {
        let (row, column) = self.guard.position;
        if row < 0 || column < 0 {
            return false;
        }

        let row = usize::try_from(row).unwrap();
        let column = usize::try_from(column).unwrap();

        let option = self.map.get(row).and_then(|row| row.get(column));
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
            return false;
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

    fn search_loop(&mut self) -> bool {
        loop {
            if !self.guard_is_inside() {
                return false
            }

            self.register_step();
            self.guard_step();

            let was_here = self.path.get(&self.guard.position)
                .map(|v| v.contains(&self.guard.direction));

            if was_here.is_some_and(|b| b) {
                return true
            }
        }

    }

    fn clone_with_obstacle(&self, (row, column): Point) -> OfficeWithAGuard {
        let mut map = self.map.clone();
        map[row as usize][column as usize] = Terrain::Obstacle;

        OfficeWithAGuard {
            path: HashMap::new(),
            map,
            guard: self.guard.clone(),
        }
    }

    fn calculate_loop_corrections(&self) -> usize {
        let mut res = 0usize;

        let mut copy = self.clone_with_obstacle((0, 0));
        copy.patrol();

        copy.path.keys().for_each(|(row, column)| {
            let mut copy = self.clone_with_obstacle((*row, *column));
            if copy.search_loop() {
                res += 1;
            }
        });

        res
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
    let office = OfficeWithAGuard::from_str(input);
    office.calculate_loop_corrections()
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
        let input = "#\n^\n";
        assert_eq!(step1(input), 2);
    }

    #[test]
    fn test_02() {
        let input = text_block_fnl!(
            "#.."
            "^.#"
            ".#."
        );
        assert_eq!(step1(input), 2);
    }

    #[test]
    fn step2_test_1() {
        let input = text_block_fnl!(
            "....#....."
            ".........#"
            ".........."
            "..#......."
            ".......#.."
            ".........."
            ".#..^....."
            "........#."
            "#........."
            "......#..."
        );

        assert_eq!(step2(input), 6);
    }
}
