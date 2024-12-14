use std::collections::HashMap;

type Position = (isize, isize);

pub struct Robot {
    pos: Position,
    velocity: (isize, isize),
}

pub struct SecureBath {
    robots: Vec<Robot>,
    dimensions: (usize, usize),
}

impl Robot {
    pub fn not_in(&self, column: isize, row: isize) -> bool {
        self.pos.0 != column && self.pos.1 != row
    }

    pub fn advance(&self, dimensions: (usize, usize)) -> Robot {
        let new_x = (self.pos.0 + self.velocity.0).rem_euclid(dimensions.0 as isize);
        let new_y = (self.pos.1 + self.velocity.1).rem_euclid(dimensions.1 as isize);

        Robot {
            velocity: self.velocity,
            pos: (new_x, new_y),
        }
    }
}

impl SecureBath {
    pub fn from_str(input: &str, columns: usize, rows: usize) -> SecureBath {
        let robots = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                let mut chunks = l.split(" ");
                let first = chunks.next().unwrap();
                let second = chunks.next().unwrap();
                let mut first_numbers = first[2..].split(",");
                let pos_x = first_numbers.next().unwrap().parse::<isize>().unwrap();
                let pos_y = first_numbers.next().unwrap().parse::<isize>().unwrap();

                let mut second = second[2..].split(",");
                let vel_x = second.next().unwrap().parse::<isize>().unwrap();
                let vel_y = second.next().unwrap().parse::<isize>().unwrap();

                Robot {
                    pos: (pos_x, pos_y),
                    velocity: (vel_x, vel_y),
                }
            })
            .collect();
        SecureBath {
            robots,
            dimensions: (columns, rows),
        }
    }

    pub fn tick(&mut self) {
        self.robots = self
            .robots
            .drain(..)
            .map(|r| r.advance(self.dimensions))
            .collect()
    }

    pub fn check_alignment(&self, column: usize, row: usize) -> bool {
        let mut column_distribution: HashMap<isize,usize> = HashMap::new();
        for r in &self.robots {
            let c = column_distribution.get(&r.pos.0).cloned().unwrap_or(0);
            column_distribution.insert(r.pos.0, c + 1);
        }

        let mut row_distribution: HashMap<isize,usize> = HashMap::new();
        for r in &self.robots {
            let c = row_distribution.get(&r.pos.1).cloned().unwrap_or(0);
            row_distribution.insert(r.pos.1, c + 1);
        }

        column_distribution.values().any(|v| *v >= column) &&
            row_distribution.values().any(|v| *v >= row)
    }

    pub fn print(&self) {
        print!("\n");
        let mut lines = vec![];
        for _ in 0..self.dimensions.1 {
            let mut vec = vec![];
            for _ in 0..self.dimensions.0 {
                vec.push(0)
            }
            lines.push(vec)
        }

        for r in &self.robots {
            lines[r.pos.1 as usize][r.pos.0 as usize] += 1
        }

        for line in lines {
            for n in line {
                if n == 0 {
                    print!(".")
                } else {
                    print!("{n}")
                }
            }
            print!("\n")
        }
    }

    pub fn safety_factor(&self) -> usize {
        let column_to_ignore = (self.dimensions.0 / 2) as isize;
        let row_to_ignore = (self.dimensions.1 / 2) as isize;

        // nort-west
        let (n1, n2, n3, n4) = self
            .robots
            .iter()
            .filter(|r| r.not_in(column_to_ignore, row_to_ignore))
            .fold((0usize, 0usize, 0usize, 0usize), |(nw, ne, sw, se), r| {
                if r.pos.0 < column_to_ignore && r.pos.1 < row_to_ignore {
                    (nw + 1, ne, sw, se)
                } else if r.pos.0 > column_to_ignore && r.pos.1 < row_to_ignore {
                    (nw, ne + 1, sw, se)
                } else if r.pos.0 < column_to_ignore && r.pos.1 > row_to_ignore {
                    (nw, ne, sw + 1, se)
                } else {
                    (nw, ne, sw, se + 1)
                }
            });
        n1 * n2 * n3 * n4
    }
}

pub fn run(input: &str, seconds: usize, columns: usize, rows: usize) -> usize {
    let mut bath = SecureBath::from_str(input, columns, rows);
    // bath.print();
    for _ in 0..seconds {
        bath.tick()
    }
    // bath.print();
    bath.safety_factor()
}

fn step1(input: &str) -> usize {
    run(input, 100, 101, 103)
}

fn step2(input: &str) -> usize {
    let mut bath = SecureBath::from_str(input, 101, 103);

    for i in 0..10000 {
        if bath.check_alignment(30, 30) {
            return i
        }
        bath.tick()
    }
    unreachable!()
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
    fn robot_in_blind_spot_are_ignored() {
        let input = text_block_fnl!(
            "p=1,1 v=1,1"
            "p=6,6 v=1,1"
            "p=0,6 v=1,1"
            "p=6,0 v=1,1"
            "p=3,1 v=1,1"
            "p=1,3 v=1,1"
        );
        assert_eq!(run(input, 0, 7, 7), 1)
    }

    #[test]
    fn robots_on_each_cuadrant_are_multiplied_together() {
        let input = text_block_fnl!(
            "p=1,1 v=1,1"
            "p=1,2 v=1,1" // 2 nor-west
            "p=5,1 v=1,1"
            "p=6,2 v=1,1" // 2 nor-east
            "p=1,5 v=1,1"
            "p=2,5 v=1,1"
            "p=1,6 v=1,1" // 3 south-west
            "p=6,6 v=1,1" // 1 south-east
        );
        assert_eq!(run(input, 0, 7, 7), 2 * 2 * 3)
    }

    #[test]
    fn robots_move_with_time() {
        let input = text_block_fnl!(
            "p=0,0 v=1,1"
            "p=0,6 v=1,1"
            "p=6,6 v=1,-1"
            "p=6,0 v=1,1"
        );
        assert_eq!(run(input, 0, 7, 7), 1);
        assert_eq!(run(input, 4, 7, 7), 0);
    }

    #[test]
    fn robot_movement() {
        let robot = Robot {
            velocity: (2, -3),
            pos: (2, 4),
        };
        let robot = robot.advance((11, 7));
        assert_eq!(robot.pos, (4, 1));
        let robot = robot.advance((11, 7));
        assert_eq!(robot.pos, (6, 5));
        let robot = robot.advance((11, 7));
        assert_eq!(robot.pos, (8, 2));
        let robot = robot.advance((11, 7));
        assert_eq!(robot.pos, (10, 6));
        let robot = robot.advance((11, 7));
        assert_eq!(robot.pos, (1, 3));
    }

    #[test]
    fn provided_example() {
        let input = text_block_fnl!(
            "p=0,4 v=3,-3"
            "p=6,3 v=-1,-3"
            "p=10,3 v=-1,2"
            "p=2,0 v=2,-1"
            "p=0,0 v=1,3"
            "p=3,0 v=-2,-2"
            "p=7,6 v=-1,-3"
            "p=3,0 v=-1,-2"
            "p=9,3 v=2,3"
            "p=7,3 v=-1,2"
            "p=2,4 v=2,-3"
            "p=9,5 v=-3,-3"
        );
        assert_eq!(run(input, 100, 11, 7), 12);
    }
}
