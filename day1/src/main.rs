use std::cmp;

fn parse_data(input: &str) -> (Vec<usize>, Vec<usize>) {
    let lines: Vec<Vec<&str>> = input
        .split("\n")
        .filter(|line| *line != "")
        .map(|line| line.split("   ").collect())
        .collect();

    let mut column1 = vec![];
    let mut column2 = vec![];

    for line in lines {
        column1.push(line.get(0).map(|s| s.parse::<usize>().unwrap()).unwrap());
        column2.push(line.get(1).map(|s| s.parse::<usize>().unwrap()).unwrap());
    }

    (column1, column2)
}

fn day_1_step_1(input: &str) -> usize {
    let (mut column1, mut column2) = parse_data(&input);

    column1.sort();
    column2.sort();

    column1
        .iter()
        .zip(column2.iter())
        .fold(0usize, |acc, (first, second)| {
            let max = cmp::max(first, second);
            let min = cmp::min(first, second);
            // println!("{} {}", first, second);
            acc + (*max - *min)
        })
}

fn day_1_step_2(input: &str) -> usize {
    let (column1, column2) = parse_data(input);

    let mut res: usize = 0;

    for c1 in column1 {
        let repetitions = column2.iter().filter(|c2| **c2 == c1).count();
        res += c1 * repetitions;
    }

    res
}

fn main() {
    let input = include_str!("../input.txt");
    let res = day_1_step_1(input);
    println!("step1: {}", res);

    let res = day_1_step_2(input);
    println!("step2: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_1_pair_returns_difference() {
        let input = "30   34\n";

        assert_eq!(day_1_step_1(input), 4)
    }

    #[test]
    fn for_2_pairs_returns_difference() {
        let input = "30   34\n100   190\n";

        assert_eq!(day_1_step_1(input), 94)
    }

    #[test]
    fn orders_pairs_out_of_rder() {
        // 1 3
        // 10 12
        // 20 26
        let input = "20   3\n10   26\n1   12\n";

        assert_eq!(day_1_step_1(input), 10)
    }

    #[test]
    fn step_2_one_row_with_same_number_turns_that_number() {
        let input = "3   3\n";
        assert_eq!(day_1_step_2(input), 3)
    }

    #[test]
    fn step_2_two_rows_with_repeated_number_returns_double_of_the_number() {
        let input = "3   3\n4   3\n";
        assert_eq!(day_1_step_2(input), 6)
    }

    #[test]
    fn step_2_minimal_example() {
        let input = concat!("3   4\n", "4   3\n", "2   5\n", "1   3\n", "3   9\n", "3   3\n",);
        assert_eq!(day_1_step_2(input), 31)
    }

    // #[test]
    // #[ignore]
    // fn run_main() {
    //     main();
    // }
}
