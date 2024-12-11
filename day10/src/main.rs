use std::collections::HashSet;

type Position = (usize, usize);

pub struct TopographicMap {
    map: Vec<Vec<u8>>
}

impl TopographicMap {
    pub fn from_str(input: &str) -> TopographicMap {
         let map = input.lines().filter(|l| !l.is_empty())
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8 ).collect() ).collect();

        TopographicMap {
            map
        }
    }

    pub fn get_trail_heads(&self) -> HashSet<Position> {
        self.map.iter().enumerate().flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, value)| **value == 0)
                .map(move |(column_index, text_block_macros)| (row_index, column_index) )
        }).collect()
    }

    pub fn total_score(&self) -> usize {
        self.get_trail_heads().iter().map(|th| self.score(th)).sum()
    }

    fn score(&self, trailhead: &Position) -> usize {
        self.score_aux(trailhead).len()
    }

    fn score_aux(&self, position: &Position) -> HashSet<Position> {
        let current = self.map[position.0][position.1];
        if current == 9 {
            return HashSet::from([*position])
        }

        self.neighbors(position).iter()
            .filter(|(row, column)| self.map[*row][*column] == current + 1 )
            .flat_map(|pos| self.score_aux(pos)).collect()
    }

    fn neighbors(&self, position: &Position) -> HashSet<Position> {
        let row = position.0;
        let column = position.1;
        let south = (row+1, column);
        let east = (row, column+1);
        let mut vec = vec![south, east];

        if row > 0 {
            vec.push((row-1, column))
        }

        if column > 0 {
            vec.push((row, column-1))
        }

        vec.drain(..).filter(|(row, column)|
            self.map.get(*row).and_then(|row|row.get(*column)).is_some()
        ).collect()
    }
}


fn step1(input: &str) -> usize {
    let map = TopographicMap::from_str(input);
    map.total_score()
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
        let input = text_block_fnl!(
            "0123"
            "1234"
            "8765"
            "9876"
        );
        assert_eq!(step1(input), 1)
    }

    #[test]
    fn test_02() {
        let input = text_block_fnl!(
            "7770777"
            "7771777"
            "7772777"
            "6543456"
            "7777777"
            "8777778"
            "9777779"
        );
        assert_eq!(step1(input), 2)
    }

    #[test]
    fn test_given_example() {
        let input = text_block_fnl! {
            "89010123"
            "78121874"
            "87430965"
            "96549874"
            "45678903"
            "32019012"
            "01329801"
            "10456732"
        };

        assert_eq!(step1(input), 36);
    }
}