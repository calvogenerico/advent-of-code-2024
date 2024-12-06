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
    #[test]
    fn test_01() {

    }
}