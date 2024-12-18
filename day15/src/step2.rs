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

        let map_section: String = sections.next().unwrap().chars().map(|char| {
            match char {
                '@' => "@.",
                'O' => "[]",
                '#' => "##",
                '.' => "..",
                '\n' => "\n",
                _ => unreachable!()
            }
        }).collect();

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
        for movement in self.path.clone() {
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
        // if let Some(first_empty_position) = self.next_empty_space_from(&self.robot, movement) {
        //     let next_position = self.robot.neighbor_at(movement);
        //     self.map[first_empty_position.row][first_empty_position.column] =
        //         self.map[next_position.row][next_position.column].clone();
        //     self.map[next_position.row][next_position.column] = Element::Empty;
        //     self.robot = next_position;
        // }

        let next_position = self.robot.neighbor_at(movement);
        match self.at(&next_position) {
            Element::Wall => {}
            Element::BoxWest => self.try_to_move_box(movement, vec![next_position, next_position.neighbor_at(&Movement::East)]),
            Element::BoxEast => self.try_to_move_box(movement, vec![next_position, next_position.neighbor_at(&Movement::West)]),
            Element::Empty => self.robot = next_position
        }


        // if try_to_move_box(next_pos):
        //   move_robot


    }

    fn next_empty_space_from(&self, position: &Position, movement: &Movement) -> Option<Position> {
        let new_position = position.neighbor_at(movement);
        match self.at(&new_position) {
            Element::Wall => None,
            Element::BoxWest | Element::BoxEast => self.next_empty_space_from(&new_position, movement),
            Element::Empty => Some(new_position),
        }
    }

    fn at(&self, position: &Position) -> &Element {
        &self.map[position.row][position.column]
    }

    fn try_to_move_box(&self, movement: &Movement, box_positions: Vec<Position>) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use text_block_macros::text_block_fnl;
    use crate::step2;
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