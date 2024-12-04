struct Level {
    cells: Vec<usize>,
    asc: bool,
}

impl Level {
    fn from_str(input: &str) -> Level {
        let cells: Vec<usize> = input
            .split(" ")
            .map(|chunk| chunk.trim())
            .map(|chunk| chunk.parse::<usize>().unwrap())
            .collect();

        let asc = cells
            .first()
            .zip(cells.last())
            .map(|(f, l)| *f < *l)
            .unwrap_or(true);

        Level { cells, asc }
    }

    pub fn is_safe(&self) -> bool {
        let pos0 = self.cells.iter();
        let pos1 = self.cells.iter().skip(1);

        pos0.zip(pos1).all(|(a, b)| self.cmp(*a, *b))
    }

    fn cmp(&self, n1: usize, n2: usize) -> bool {
        if self.asc {
            n1 < n2 && (n2 - n1) <= 3
        } else {
            n2 < n1 && (n1 - n2) <= 3
        }
    }
}

fn day2_step1(input: &str) -> usize {
    let levels: Vec<Level> = input.lines()
        .filter(|s| !s.is_empty())
        .map(Level::from_str)
        .collect();
    levels.iter().filter(|l| l.is_safe()).count()
}

fn main() {
    let input = include_str!("../input.txt");
    let res = day2_step1(input);

    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascending_increments_of_1_is_safe() {
        let input = "1 2 3\n";

        assert_eq!(day2_step1(input), 1);
    }

    #[test]
    fn mixed_increment_and_decrement_is_unsafe() {
        let input = "1 2 3 1\n";

        assert_eq!(day2_step1(input), 0);
    }

    #[test]
    fn decrease_order_is_safe() {
        let input = "3 2 1\n";

        assert_eq!(day2_step1(input), 1);
    }

    #[test]
    fn increase_with_no_increment_is_unsafe() {
        let input = "1 2 2 3\n";

        assert_eq!(day2_step1(input), 0);
    }

    #[test]
    fn single_number_is_safe() {
        let input = "1\n";

        assert_eq!(day2_step1(input), 1);
    }

    #[test]
    fn difference_more_than_3_is_unsafe() {
        let input = "1 2 6 7\n";

        assert_eq!(day2_step1(input), 0);
    }

    #[test]
    fn difference_more_than_3_is_in_descending() {
        let input = "7 6 2 1\n";

        assert_eq!(day2_step1(input), 0);
    }

    #[test]
    fn minimal_example() {
        let input = concat!(
            "7 6 4 2 1\n",
            "1 2 7 8 9\n",
            "9 7 6 2 1\n",
            "1 3 2 4 5\n",
            "8 6 4 4 1\n",
            "1 3 6 7 9\n"
        );
        assert_eq!(day2_step1(input), 2);
    }
}
