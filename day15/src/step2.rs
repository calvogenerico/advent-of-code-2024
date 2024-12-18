use crate::step1::{Movement, Position};

#[derive(Clone, PartialEq, Eq)]
pub enum Element {
    Wall,
    BoxWest,
    BoxEast,
    Empty,
}

pub struct LargerGoodsDeposit {
    map: Vec<Vec<Element>>,
    robot: Position,
    path: Vec<Movement>,
}

impl Element {
    pub fn from_char(c: char) -> Element {
        match c {
            '#' => Element::Wall,
            '[' => Element::BoxWest,
            ']' => Element::BoxEast,
            '@' | '.' => Element::Empty,
            _ => unreachable!(),
        }
    }
}

impl LargerGoodsDeposit {
    pub fn from_str(input: &str) -> LargerGoodsDeposit {
        let mut sections = input.split("\n\n");

        let map_section: String = sections
            .next()
            .unwrap()
            .chars()
            .map(|char| match char {
                '@' => "@.",
                'O' => "[]",
                '#' => "##",
                '.' => "..",
                '\n' => "\n",
                _ => unreachable!(),
            })
            .collect();

        let map = map_section
            .lines()
            .map(|l| l.chars().map(Element::from_char).collect())
            .collect();
        let robot = map_section
            .lines()
            .enumerate()
            .find_map(|(row, l)| {
                l.chars()
                    .enumerate()
                    .find_map(|(column, char)| (char == '@').then_some(column))
                    .map(|column| (row, column))
            })
            .map(|(row, column)| Position::new(row, column))
            .unwrap();

        let path_section = sections.next().unwrap();

        let path = path_section
            .trim()
            .replace("\n", "")
            .chars()
            .map(Movement::from_char)
            .collect();

        LargerGoodsDeposit { map, robot, path }
    }

    pub fn exec(&mut self) {
        // self.print();
        for movement in self.path.clone() {
            // println!("Next: {}", &movement.to_s());
            // self.print();
            self.try_to_move_robot(&movement);
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        self.map.iter().enumerate().for_each(|(row_n, row)| {
            row.iter().enumerate().for_each(|(column_n, element)| {
                if self.robot.column == column_n && self.robot.row == row_n {
                    print!("@");
                    return;
                }

                match element {
                    Element::Wall => print!("#"),
                    Element::BoxWest => print!("["),
                    Element::BoxEast => print!("]"),
                    Element::Empty => print!("."),
                }
            });
            print!("\n");
        })
    }

    pub fn sum_gps(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(row_n, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, elem)| **elem == Element::BoxWest)
                    .map(move |(column_n, _)| row_n * 100 + column_n)
            })
            .sum()
    }

    fn try_to_move_robot(&mut self, movement: &Movement) {
        let next_position = self.robot.neighbor_at(movement);
        if self.can_push_to(&next_position, movement) {
            self.push_to(&next_position, movement);
            self.robot = next_position;
        }
    }

    fn at(&self, position: &Position) -> &Element {
        &self.map[position.row][position.column]
    }

    fn push_to(&mut self, pos: &Position, movement: &Movement) {
        let next_elem = self.at(pos).clone();

        match (next_elem, movement) {
            (Element::Wall, _) => unreachable!(),
            (Element::Empty, _) => {
                return;
            },
            (Element::BoxEast, Movement::North) | (Element::BoxEast, Movement::South)  => {
                let other_half = pos.west();
                self.push_to(&pos.neighbor_at(movement), movement);
                self.push_to(&other_half.neighbor_at(movement), movement);
                self.replace(&pos.neighbor_at(movement), Element::BoxEast);
                self.replace(&other_half.neighbor_at(movement), Element::BoxWest);
                self.replace(pos, Element::Empty);
                self.replace(&other_half, Element::Empty);
            }
            (Element::BoxWest, Movement::North) | (Element::BoxWest, Movement::South) => {
                let other_half = pos.east();
                self.push_to(&pos.neighbor_at(movement), movement);
                self.push_to(&other_half.neighbor_at(movement), movement);
                self.replace(&pos.neighbor_at(movement), Element::BoxWest);
                self.replace(&other_half.neighbor_at(movement), Element::BoxEast);
                self.replace(pos, Element::Empty);
                self.replace(&other_half, Element::Empty);
            }
            (Element::BoxEast, Movement::West) => {
                self.push_to(&pos.west().west(), movement);
                self.replace(&pos.west().west(), Element::BoxWest);
                self.replace(&pos.west(), Element::BoxEast);
                self.replace(&pos, Element::Empty);
            }
            (Element::BoxWest, Movement::East) => {
                self.push_to(&pos.east().east(), movement);
                self.replace(&pos.east().east(), Element::BoxEast);
                self.replace(&pos.east(), Element::BoxWest);
                self.replace(&pos, Element::Empty);
            }

            _ => unreachable!()
        }
    }

    fn replace(&mut self, position: &Position, elem: Element) {
        self.map[position.row][position.column] = elem
    }

    fn can_push_to(&self, pos: &Position, movement: &Movement) -> bool {
        let next_elem = self.at(pos).clone();

        match (next_elem, movement) {
            (Element::Wall, _) => false,
            (Element::Empty, _) => true,
            (Element::BoxEast, Movement::North) | (Element::BoxEast, Movement::South) => {
                let other_half = pos.west();
                self.can_push_to(&pos.neighbor_at(movement), movement) &&
                    self.can_push_to(&other_half.neighbor_at(movement), movement)
            }
            (Element::BoxWest, Movement::North) | (Element::BoxWest, Movement::South) => {
                let other_half = pos.east();
                self.can_push_to(&pos.neighbor_at(movement), movement) &&
                    self.can_push_to(&other_half.neighbor_at(movement), movement)
            }
            (Element::BoxEast, Movement::West) => {
                self.can_push_to(&pos.west().west(), movement)
            }
            (Element::BoxWest, Movement::East) => {
                self.can_push_to(&pos.east().east(), movement)
            }

            _ => unreachable!()
        }

        // match self.at(pos) {
        //     Element::Wall => false,
        //     Element::BoxWest => match movement {
        //         Movement::North | Movement::South => {
        //             self.can_push_to(&pos.neighbor_at(movement), movement)
        //                 && self.can_push_to(&pos.neighbor_at(&Movement::East).neighbor_at(movement), movement)
        //         }
        //         Movement::East => {
        //             self.can_push_to(&pos.neighbor_at(&Movement::East).neighbor_at(&Movement::East), movement)
        //         }
        //         Movement::West => {
        //             self.can_push_to(&pos.neighbor_at(movement), movement)
        //         }
        //     },
        //     Element::BoxEast => match movement {
        //         Movement::North | Movement::South => {
        //             self.can_push_to(&pos.neighbor_at(movement), movement)
        //                 && self.can_push_to(&pos.neighbor_at(&Movement::West).neighbor_at(movement), movement)
        //         }
        //         Movement::East => {
        //             self.can_push_to(&pos.neighbor_at(movement), movement)
        //         }
        //         Movement::West => {
        //             self.can_push_to(&pos.neighbor_at(&Movement::West).neighbor_at(&Movement::West), movement)
        //         }
        //     },
        //     Element::Empty => true,
        // }
    }
}

#[cfg(test)]
mod tests {
    use crate::step2;
    use text_block_macros::text_block_fnl;
    // use super::*;

    #[test]
    fn test_01() {
        let input = text_block_fnl!(
            "##########"
            "#..O..O.O#"
            "#......O.#"
            "#.OO..O.O#"
            "#..O@..O.#"
            "#O#..O...#"
            "#O..O..O.#"
            "#.OO.O.OO#"
            "#....O...#"
            "##########"
            ""
            "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^"
            "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v"
            "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<"
            "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^"
            "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><"
            "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^"
            ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^"
            "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>"
            "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>"
            "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
        );
        assert_eq!(step2(input), 9021)
    }
}