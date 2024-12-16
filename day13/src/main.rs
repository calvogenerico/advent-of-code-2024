fn step1(input: &str) -> usize {
    input.len()
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
            "a"
            "b"
        );
        assert_eq!(step1(input), 4)
    }
}