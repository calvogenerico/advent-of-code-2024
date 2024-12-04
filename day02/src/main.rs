struct Level {
    cells: Vec<usize>,
    asc: bool,
}

impl Level {
    fn new(cells: Vec<usize>) -> Level {
        let asc = cells
            .first()
            .zip(cells.last())
            .map(|(f, l)| *f < *l)
            .unwrap_or(true);

        Level { cells, asc }
    }

    fn from_str(input: &str) -> Level {
        let cells: Vec<usize> = input
            .split(" ")
            .map(|chunk| chunk.trim())
            .map(|chunk| chunk.parse::<usize>().unwrap())
            .collect();

        Self::new(cells)
    }

    pub fn is_safe(&self) -> bool {
        let pos0 = self.cells.iter();
        let pos1 = self.cells.iter().skip(1);

        pos0.zip(pos1).all(|(a, b)| self.cmp(*a, *b))
    }

    pub fn is_safe_with_tolerance(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        (0..self.cells.len())
            .into_iter()
            .any(|index| self.without(index).is_safe())
    }

    pub fn without(&self, index: usize) -> Level {
        let mut vec = Vec::with_capacity(self.cells.len() - 1);
        vec.extend_from_slice(&self.cells[0..index]);
        vec.extend_from_slice(&self.cells[index + 1..]);
        Self::new(vec)
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
    let levels: Vec<Level> = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(Level::from_str)
        .collect();
    levels.iter().filter(|l| l.is_safe()).count()
}

fn day2_step2(input: &str) -> usize {
    let levels: Vec<Level> = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(Level::from_str)
        .collect();
    levels.iter().filter(|l| l.is_safe_with_tolerance()).count()
}

fn main() {
    let input = include_str!("../input.txt");
    let step1 = day2_step1(input);
    let step2 = day2_step2(input);

    println!("step 1: {}", step1);
    println!("step 2: {}", step2);
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
    fn tolerance_1_error_and_is_still_safe() {
        let input = "1 2 3 7 4 5\n";

        assert_eq!(day2_step2(input), 1);
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
