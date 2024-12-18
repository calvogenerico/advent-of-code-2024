pub struct Position {
    pub row: usize,
    pub column: usize,
}

impl Position {
    pub fn neighbor_at(&self, movement: &Movement) -> Position {
        match movement {
            Movement::North => Self {
                row: self.row - 1,
                column: self.column,
            },
            Movement::East => Self {
                row: self.row,
                column: self.column + 1,
            },
            Movement::South => Self {
                row: self.row + 1,
                column: self.column,
            },
            Movement::West => Self {
                row: self.row,
                column: self.column - 1,
            },
        }
    }

    pub fn north(&self) -> Position {
        self.neighbor_at(&Movement::North)
    }

    pub fn east(&self) -> Position {
        self.neighbor_at(&Movement::East)
    }

    pub fn south(&self) -> Position {
        self.neighbor_at(&Movement::South)
    }

    pub fn west(&self) -> Position {
        self.neighbor_at(&Movement::West)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Element {
    Wall,
    Box,
    Empty,
}

#[derive(Clone)]
pub enum Movement {
    North,
    East,
    South,
    West,
}

impl Movement {

    #[allow(dead_code)]
    pub fn to_s(&self) -> String {
        match self {
            Movement::North => "^".to_string(),
            Movement::East => ">".to_string(),
            Movement::South => "v".to_string(),
            Movement::West => "<".to_string()
        }
    }
}

pub struct GoodsDeposit {
    map: Vec<Vec<Element>>,
    robot: Position,
    path: Vec<Movement>,
}

impl Movement {
    pub fn from_char(c: char) -> Movement {
        match c {
            '^' => Movement::North,
            '>' => Movement::East,
            'v' => Movement::South,
            '<' => Movement::West,
            _ => {

                unreachable!()
            },
        }
    }
}

impl Position {
    pub fn new(row: usize, column: usize) -> Position {
        Self { row, column }
    }
}

impl Element {
    pub fn from_char(c: char) -> Element {
        match c {
            '#' => Element::Wall,
            'O' => Element::Box,
            '@' | '.' => Element::Empty,
            _ => unreachable!(),
        }
    }
}

impl GoodsDeposit {
    pub fn from_str(input: &str) -> GoodsDeposit {
        let mut sections = input.split("\n\n");

        let map_section = sections.next().unwrap();

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

        GoodsDeposit { map, robot, path }
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
                    Element::Box => print!("O"),
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
                    .filter(|(_, elem)| **elem == Element::Box)
                    .map(move |(column_n, _)| row_n * 100 + column_n)
            })
            .sum()
    }

    fn try_to_move_robot(&mut self, movement: &Movement) {
        if let Some(first_empty_position) = self.next_empty_space_from(&self.robot, movement) {
            let next_position = self.robot.neighbor_at(movement);
            self.map[first_empty_position.row][first_empty_position.column] =
                self.map[next_position.row][next_position.column].clone();
            self.map[next_position.row][next_position.column] = Element::Empty;
            self.robot = next_position;
        }
    }

    fn next_empty_space_from(&self, position: &Position, movement: &Movement) -> Option<Position> {
        let new_position = position.neighbor_at(movement);
        match self.at(&new_position) {
            Element::Wall => None,
            Element::Box => self.next_empty_space_from(&new_position, movement),
            Element::Empty => Some(new_position),
        }
    }

    fn at(&self, position: &Position) -> &Element {
        &self.map[position.row][position.column]
    }
}