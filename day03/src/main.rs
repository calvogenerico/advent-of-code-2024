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

fn step2(input: &str) -> usize {
    let reg = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))").unwrap();
    let instances = reg.captures_iter(input).map(|c| c.extract::<1>());

    let mut res: usize = 0;
    let mut enabled = true;

    for (_, capture) in instances {
        if capture[0].starts_with("mul(") && enabled {
            let coso = capture[0]
                .to_string()
                .replace("mul(", "")
                .replace(")", "");
            let mut parts = coso
                .split(",");
            let n1 = parts.next().unwrap().parse::<usize>().unwrap();
            let n2 = parts.next().unwrap().parse::<usize>().unwrap();
            res = res + n1 * n2;
        }
        if capture[0].starts_with("do(") {
            enabled = true;
        }
        if capture[0].starts_with("don't(") {
            enabled = false;
        }
    }

    res
}

fn main() {
    let input = include_str!("../input.txt");
    let res1 = step1(input);
    let res2 = step2(input);

    println!("step1: {}", res1);
    println!("step2: {}", res2);
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_01() {
        let input = "mul(1,2) mul(3,4)";
        assert_eq!(step1(input), 14);
    }

    //
    // step2
    //

    #[test]
    fn test_02() {
        let input = "mul(1,2) mul(3,4) don't() mul(1,1)";
        assert_eq!(step2(input), 14);
    }

    #[test]
    fn test_03() {
        let input = "mul(1,2) mul(3,4) don't() mul(1,1) do() mul(3, 1)";
        assert_eq!(step2(input), 17);
    }
}
