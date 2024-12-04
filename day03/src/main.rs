use regex::Regex;

fn step1(input: &str) -> usize {
    let reg = Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();
    let instances = reg.captures_iter(input).map(|c| c.extract::<2>());

    let sum = instances
        .map(|(_, n)| n)
        .map(|strs| (strs[0].parse::<usize>().unwrap(), strs[1].parse::<usize>().unwrap()))
        .map(|(n1, n2)| n1 * n2)
        .sum();

    sum
}

fn main() {
    let input = include_str!("../input.txt");
    let res1 = step1(input);

    println!("step1: {}", res1);
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn tesT_01() {
        let input = "mul(1,2) mul(3,4)";
        assert_eq!(step1(input), 14);
    }
}
