use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
struct Point {
    row: isize,
    column: isize
}

struct AntennaField {
    columns: isize,
    rows: isize,
    antenas: HashMap<char, Vec<Point>>
}

impl AntennaField {
    pub fn from_str(input: &str) -> AntennaField {
        let rows = input.lines().filter(|l| !l.is_empty()).count();
        let columns = input.lines().next().unwrap().len();
        let mut antenas: HashMap<char, Vec<Point>> = HashMap::new();


        input.lines().enumerate().for_each(|(row, l)| {
            l.chars().enumerate().for_each(|(column, frequency)| {
                if frequency.is_alphanumeric() {
                    let point = Point { row: isize::try_from(row).unwrap(), column: isize::try_from(column).unwrap() };
                    antenas.get_mut(&frequency)
                        .and_then(|vec| {
                            Some(vec.push(point))
                        })
                        .or_else(|| {
                            antenas.insert(frequency, vec![point]) ;
                            None
                        } )
                    ;
                }
            })
        });

        AntennaField {
            rows: isize::try_from(rows).unwrap(),
            columns: isize::try_from(columns).unwrap(),
            antenas
        }
    }

    pub fn count_nearer_anti_nodes(&self) -> usize {
        let mut res: HashSet<Point> = HashSet::new();
        self.antenas.iter().for_each(|(_frequency, positions)| {
            positions.iter().for_each(|current | {
                positions.iter().filter(|another| *another != current )
                    .map(|another| self.calculate_nearer_antinode(*current, *another) )
                    .filter(|opt| opt.is_some())
                    .map(|opt| opt.unwrap())
                    .for_each(|p| { res.insert(p); } );
            })
        });

        res.len()
    }

    pub fn count_all_anti_nodes(&self) -> usize {
        let mut res: HashSet<Point> = HashSet::new();
        self.antenas.iter().for_each(|(_frequency, positions)| {
            positions.iter().for_each(|current | {
                positions.iter().filter(|another| *another != current )
                    .map(|another| self.calculate_all_antinodes(*current, *another) )
                    .for_each(|p| {
                        p.iter().for_each(|p| {
                            res.insert(*p);
                        })
                    } );
            })
        });

        res.len()
    }


    // fn print_res(&self, data: &HashSet<Point>) {
    //     let mut vecs = vec![];
    //     for row in 0..self.rows {
    //         let mut vec = vec![];
    //         for column in 0..self.columns {
    //             vec.push('.');
    //         }
    //         vecs.push(vec);
    //     }
    //
    //     for p in data {
    //         vecs[p.row as usize][p.column as usize] = '#'
    //     }
    //
    //     for row in vecs {
    //         println!("{}", row.into_iter().collect::<String>())
    //     }
    // }

    fn calculate_nearer_antinode(&self, p1: Point, p2: Point) -> Option<Point> {
        let d_row = p1.row - p2.row;
        let d_column = p1.column - p2.column;


        let diff_point = Point { row: p2.row - d_row, column: p2.column - d_column };

        (diff_point.row >= 0 && diff_point.column >= 0)
            .then_some(diff_point)
            .filter(|Point {row, column} | *row < self.rows && *column < self.columns )
    }

    fn calculate_all_antinodes(&self, p1: Point, p2: Point) -> Vec<Point> {
        let mut res = vec![p2];
        let mut current1 = p1;
        let mut current2 = p2;
        while let Some(proyected) = self.calculate_nearer_antinode(current1, current2) {
            res.push(proyected);
            current1 = current2;
            current2 = proyected
        }

        res
    }
}

fn step1(input: &str) -> usize {
    let field = AntennaField::from_str(input);
    field.count_nearer_anti_nodes()
}

fn step2(input: &str) -> usize {
    let field = AntennaField::from_str(input);
    field.count_all_anti_nodes()
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
        let input = text_block_fnl!(
            "...."
            ".a.."
            "..a."
            "...."
        );
        assert_eq!(step1(input), 2)
    }

    #[test]
    fn it_does_not_count_antonodes_outside_of_the_map() {
        let input = text_block_fnl!(
            "...."
            "...."
            "..a."
            "...a"
        );
        assert_eq!(step1(input), 1)
    }

    #[test]
    fn it_does_not_count_anti_nodes_outside_of_the_to_left() {
        let input = text_block_fnl!(
            "...."
            "aa.."
            "...."
            "...."
        );
        assert_eq!(step1(input), 1)
    }

    #[test]
    fn provided_simple_scenario() {
        let input = text_block_fnl!(
            "............"
            "........0..."
            ".....0......"
            ".......0...."
            "....0......."
            "......A....."
            "............"
            "............"
            "........A..."
            ".........A.."
            "............"
            "............"
        );
        assert_eq!(step1(input), 14)
    }

    #[test]
    fn step_2_projects_anti_nodes() {
        let input = text_block_fnl!(
            "a...."
            ".a..."
            "....."
            "....."
            "....."
        );
        assert_eq!(step2(input), 5)
    }

    #[test]
    fn provided_simple_scenario_step_2() {
        let input = text_block_fnl!(
            "............"
            "........0..."
            ".....0......"
            ".......0...."
            "....0......."
            "......A....."
            "............"
            "............"
            "........A..."
            ".........A.."
            "............"
            "............"
        );
        assert_eq!(step2(input), 34)
    }
}