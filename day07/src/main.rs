pub struct EquationLine {
    expected: usize,
    elements: Vec<usize>
}

#[derive(Debug, Clone)]
pub enum Operation {
    Sum,
    Prod,
    Concat
}
fn concat_n(a: usize, b: usize) -> usize {
    format!("{}{}", a, b).parse().unwrap()
}

impl EquationLine {
    pub fn from_str(input: &str) -> EquationLine{
        let mut parts = input.split(":");
        let expected = parts.next().unwrap().trim();
        let elements =  parts.next().unwrap().trim().split(" ").map(|chunk| chunk.trim())
            .map(|chunk| {
                chunk.parse().unwrap()
            }).collect();

        EquationLine {
            expected: expected.parse().unwrap(),
            elements
        }
    }

    pub fn calibration_result(&self) -> usize {
        if self.can_achieve_equality() {
            self.expected
        } else {
            0
        }
    }

    fn can_achieve_equality(&self) -> bool {
        let possibilities: Vec<Vec<Operation>> = Self::calculate_possibilities(self.elements.len() - 1);

        possibilities.iter().any(|operations| {

            let calculated = self.elements.iter().cloned().enumerate().reduce(|(index, a), (next_index, b)| {
                let r = match operations[index] {
                    Operation::Sum => a + b,
                    Operation::Prod => a * b,
                    Operation::Concat => {
                        concat_n(a, b)
                    }
                };
                (next_index, r)
            }).map(|(_, a)| a).unwrap();
            calculated == self.expected
        })
    }

    fn calculate_possibilities(final_length: usize) -> Vec<Vec<Operation>>{
        let mut all_possibilities = vec![vec![Operation::Prod], vec![Operation::Sum], vec![Operation::Concat]];
        for _ in 0..(final_length - 1) {
            let mut new = vec![];
            for p in all_possibilities {
                let mut with_mul = p.clone();
                with_mul.push(Operation::Prod);

                let mut with_sum = p.clone();
                with_sum.push(Operation::Sum);

                let mut with_concat = p.clone();
                with_concat.push(Operation::Concat);


                new.push(with_mul);
                new.push(with_sum);
                new.push(with_concat);
            }
            all_possibilities = new;
        }
        all_possibilities
    }
}

fn step1(input: &str) -> usize {
    input
        .lines()
        .filter(|l| l.len() > 0)
        .map(EquationLine::from_str)
        .map(|e| e.calibration_result())
        .sum()
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
            "190: 10 19"
        );
        assert_eq!(step1(input), 190)
    }

    #[test]
    fn test_02() {
        let input = text_block_fnl!(
            "190: 10 19"
            "11: 12 13"
        );
        assert_eq!(step1(input), 190)
    }

    #[test]
    fn test_03() {
        let input = text_block_fnl!(
            "14: 10 3 1"
            "11: 12 13"
        );
        assert_eq!(step1(input), 14)
    }

    #[test]
    fn mini_case_from_text() {
        let input = text_block_fnl! {
            "190: 10 19"
            "3267: 81 40 27"
            "83: 17 5"
            "156: 15 6"
            "7290: 6 8 6 15"
            "161011: 16 10 13"
            "192: 17 8 14"
            "21037: 9 7 18 13"
            "292: 11 6 16 20"
        };
        assert_eq!(step1(input), 11387)
    }

    #[test]
    fn concat_usize_1() {
        assert_eq!(concat_n(1, 2), 12)
    }

}