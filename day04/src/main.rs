struct XmasMatrix {
    matrix: Vec<Vec<char>>,
    columns: usize,
    rows: usize,
}

impl XmasMatrix {
    pub fn from_str(data: &str) -> XmasMatrix {
        let matrix = data
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let columns = matrix[0].len();
        let rows = matrix.len();
        XmasMatrix {
            matrix,
            columns,
            rows,
        }
    }

    pub fn count(&self) -> usize {
        let x_positions = self.find_x_positions();
        x_positions
            .iter()
            .map(|(row, column)| self.find_rest_of_the_word(*row, *column))
            .sum()
    }

    fn find_rest_of_the_word(&self, row: usize, column: usize) -> usize {
        self.west_to_east(row, column)
            + self.east_to_west(row, column)
            + self.north_to_south(row, column)
            + self.south_to_north(row, column)
            + self.sw_to_ne(row, column)
            + self.nw_to_se(row, column)
            + self.ne_to_sw(row, column)
            + self.se_to_nw(row, column)
    }

    fn west_to_east(&self, row: usize, column: usize) -> usize {
        let is_match = self.position_is(row, column + 1, 'M')
            && self.position_is(row, column + 2, 'A')
            && self.position_is(row, column + 3, 'S');
        usize::from(is_match)
    }

    fn east_to_west(&self, row: usize, column: usize) -> usize {
        if column < 3 { return 0 }

        let is_match = self.position_is(row, column - 1, 'M')
            && self.position_is(row, column - 2, 'A')
            && self.position_is(row, column - 3, 'S');
        usize::from(is_match)
    }

    fn north_to_south(&self, row: usize, column: usize) -> usize {
        let is_match = self.position_is(row + 1, column, 'M')
            && self.position_is(row + 2, column, 'A')
            && self.position_is(row + 3, column, 'S');
        usize::from(is_match)
    }

    fn south_to_north(&self, row: usize, column: usize) -> usize {
        if row < 3 { return 0 }
        let is_match = self.position_is(row - 1, column, 'M')
            && self.position_is(row - 2, column, 'A')
            && self.position_is(row - 3, column, 'S');
        usize::from(is_match)
    }

    fn sw_to_ne(&self, row: usize, column: usize) -> usize {
        if row < 3 { return 0 }

        let is_match = self.position_is(row - 1, column + 1, 'M')
            && self.position_is(row - 2, column + 2, 'A')
            && self.position_is(row - 3, column + 3, 'S');
        usize::from(is_match)
    }

    fn nw_to_se(&self, row: usize, column: usize) -> usize {
        let is_match = self.position_is(row + 1, column + 1, 'M')
            && self.position_is(row + 2, column + 2, 'A')
            && self.position_is(row + 3, column + 3, 'S');
        usize::from(is_match)
    }

    fn ne_to_sw(&self, row: usize, column: usize) -> usize {
        if column < 3 { return 0 }
        let is_match = self.position_is(row + 1, column - 1, 'M')
            && self.position_is(row + 2, column - 2, 'A')
            && self.position_is(row + 3, column - 3, 'S');
        usize::from(is_match)
    }

    fn se_to_nw(&self, row: usize, column: usize) -> usize {
        if row < 3 { return 0 }
        if column < 3 { return 0 }
        let is_match = self.position_is(row - 1, column - 1, 'M')
            && self.position_is(row - 2, column - 2, 'A')
            && self.position_is(row - 3, column - 3, 'S');
        usize::from(is_match)
    }


    fn position_is(&self, row: usize, column: usize, target: char) -> bool {
        self.matrix
            .get(row)
            .and_then(|inner| inner.get(column))
            .map(|c| *c == target)
            .unwrap_or(false)
    }

    fn find_x_positions(&self) -> Vec<(usize, usize)> {
        let mut res = vec![];

        for row in 0..self.rows {
            for column in 0..self.columns {
                let cell = self.matrix[row][column];
                if cell == 'X' {

                    res.push((row, column))
                }
            }
        }

        res
    }
}

fn step1(input: &str) -> usize {
    let matrix = XmasMatrix::from_str(input);
    matrix.count()
}

fn main() {
    let input = include_str!("../input.txt");
    let res1 = step1(input);

    println!("step1: {}", res1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_count_horizontal() {
        let input = "XMAS\n";
        assert_eq!(step1(input), 1);
    }

    #[test]
    fn can_count_2_horizontal2() {
        let input = "XMASXMAS\n";
        assert_eq!(step1(input), 2);
    }

    #[test]
    fn can_count_vertical() {
        let input = concat!("X\n", "M\n", "A\n", "S\n");
        assert_eq!(step1(input), 1);
    }

    #[test]
    fn can_count_2_vertical() {
        let input = concat!("XX\n", "MM\n", "AA\n", "SS\n");
        assert_eq!(step1(input), 2);
    }

    #[test]
    fn can_cuunt_a_shared_x() {
        let input = concat!("XMAS\n", "M...\n", "A...\n", "S...\n");
        assert_eq!(step1(input), 2);
    }

    #[test]
    fn can_ignore_non_matching_x() {
        let input = concat!("XMAZ\n", "M...\n", "A...\n", "S...\n");
        assert_eq!(step1(input), 1);
    }

    #[test]
    fn can_count_horizontal_backward() {
        let input = concat!("SAMX\n",);
        assert_eq!(step1(input), 1);
    }

    #[test]
    fn can_count_bottom_to_top() {
        let input = concat!(
            "S\n",
            "A\n",
            "M\n",
            "X\n"
        );
        assert_eq!(step1(input), 1);
    }

    #[test]
    fn can_count_ascending() {
        let input = concat!(
        "...S\n",
        "..A.\n",
        ".M..\n",
        "X...\n"
        );
        assert_eq!(step1(input), 1);
    }

    #[test]
    fn can_count_descending() {
        let input = concat!(
        "X...\n",
        ".M..\n",
        "..A.\n",
        "...S\n"
        );
        assert_eq!(step1(input), 1);
    }

    #[test]
    fn can_count_ascendiong_backwards() {
        let input = concat!(
        "...X\n",
        "..M.\n",
        ".A..\n",
        "S...\n"
        );
        assert_eq!(step1(input), 1);
    }

    #[test]
    fn can_count_descending_backwards() {
        let input = concat!(
        "S...\n",
        ".A..\n",
        "..M.\n",
        "...X\n"
        );
        assert_eq!(step1(input), 1);
    }

    #[test]
    fn simple_input() {
        let input = concat!(
            "..X...\n",
            ".SAMX.\n",
            ".A..A.\n",
            "XMAS.S\n",
            ".X....\n"
        );

        assert_eq!(step1(input), 4);
    }

    #[test]
    fn example_input() {
        let input = concat!(
            "MMMSXXMASM\n",
            "MSAMXMSMSA\n",
            "AMXSXMAAMM\n",
            "MSAMASMSMX\n",
            "XMASAMXAMM\n",
            "XXAMMXXAMA\n",
            "SMSMSASXSS\n",
            "SAXAMASAAA\n",
            "MAMMMXMMMM\n",
            "MXMXAXMASX\n"
        );

        assert_eq!(step1(input), 18);
    }
}
