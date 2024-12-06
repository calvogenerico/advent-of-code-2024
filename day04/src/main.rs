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

    pub fn count_xmas(&self) -> usize {
        let x_positions = self.find_positions_of('X');
        x_positions
            .iter()
            .map(|(row, column)| self.find_rest_of_xmas(*row, *column))
            .sum()
    }

    pub fn count_mas_cross(&self) -> usize {
        let x_positions = self.find_positions_of('A');
        x_positions
            .iter()
            .map(|(row, column)| self.find_rest_of_mas_cross(*row, *column))
            .sum()
    }

    fn find_rest_of_xmas(&self, row: usize, column: usize) -> usize {
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


    fn find_rest_of_mas_cross(&self, row: usize, column: usize) -> usize {
        self.m_to_north(row, column)
            + self.m_to_east(row, column)
            + self.m_to_west(row, column)
            + self.m_to_south(row, column)
    }

    fn m_to_north(&self, row: usize, column: usize) -> usize {
        if row < 1 { return 0 }
        if column < 1 { return 0 }

        let is_match = self.position_is(row - 1, column - 1, 'M')
            && self.position_is(row - 1, column + 1, 'M')
            && self.position_is(row + 1, column - 1, 'S')
            && self.position_is(row + 1, column + 1, 'S');

        is_match.into()
    }
    fn m_to_east(&self, row: usize, column: usize) -> usize {
        if row < 1 { return 0 }
        if column < 1 { return 0 }

        let is_match = self.position_is(row + 1, column + 1, 'M')
            && self.position_is(row - 1, column + 1, 'M')
            && self.position_is(row - 1, column - 1, 'S')
            && self.position_is(row + 1, column - 1, 'S');

        is_match.into()
    }
    fn m_to_west(&self, row: usize, column: usize) -> usize {
        if row < 1 { return 0 }
        if column < 1 { return 0 }

        let is_match = self.position_is(row - 1, column - 1, 'M')
            && self.position_is(row + 1, column - 1, 'M')
            && self.position_is(row - 1, column + 1, 'S')
            && self.position_is(row + 1, column + 1, 'S');

        is_match.into()
    }
    fn m_to_south(&self, row: usize, column: usize) -> usize {
        if row < 1 { return 0 }
        if column < 1 { return 0 }

        let is_match = self.position_is(row + 1, column - 1, 'M')
            && self.position_is(row + 1, column + 1, 'M')
            && self.position_is(row - 1, column + 1, 'S')
            && self.position_is(row - 1, column - 1, 'S');

        is_match.into()
    }

    fn position_is(&self, row: usize, column: usize, target: char) -> bool {
        let x = self.matrix
            .get(row)
            .and_then(|inner| inner.get(column))
            .map(|c| {
                *c == target
            })
            .unwrap_or(false);

        x
    }

    fn find_positions_of(&self, token: char) -> Vec<(usize, usize)> {
        let mut res = vec![];

        for row in 0..self.rows {
            for column in 0..self.columns {
                let cell = self.matrix[row][column];
                if cell == token {
                    res.push((row, column))
                }
            }
        }

        res
    }
}

fn step1(input: &str) -> usize {
    let matrix = XmasMatrix::from_str(input);
    matrix.count_xmas()
}

fn step2(input: &str) -> usize {
    let matrix = XmasMatrix::from_str(input);
    matrix.count_mas_cross()
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

    //
    // Step 2
    //

    #[test]
    pub fn step_2_simplest_scenario_north() {
        let input = concat!(
            "M.M\n",
            ".A.\n",
            "S.S\n",
        );
        assert_eq!(step2(input), 1)
    }

    #[test]
    pub fn step_2_simplest_scenario_south() {
        let input = concat!(
        "S.S\n",
        ".A.\n",
        "M.M\n",
        );
        assert_eq!(step2(input), 1)
    }

    #[test]
    pub fn step_2_simplest_scenario_east() {
        let input = concat!(
        "S.M\n",
        ".A.\n",
        "S.M\n",
        );
        assert_eq!(step2(input), 1)
    }

    #[test]
    pub fn step_2_simplest_scenario_wast() {
        let input = concat!(
        "M.S\n",
        ".A.\n",
        "M.S\n",
        );
        assert_eq!(step2(input), 1)
    }
}
