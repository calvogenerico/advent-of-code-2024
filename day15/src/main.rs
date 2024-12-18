use step1::GoodsDeposit;

mod step1;

fn step1(input: &str) -> usize {
    let mut deposit = GoodsDeposit::from_str(input);
    deposit.exec();
    deposit.sum_gps()
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
            "########"
            "#..O.O.#"
            "##@.O..#"
            "#...O..#"
            "#.#.O..#"
            "#...O..#"
            "#......#"
            "########"
            ""
            "<^^>>>vv<v>>v<<"
        );
        assert_eq!(step1(input), 2028)
    }
}
