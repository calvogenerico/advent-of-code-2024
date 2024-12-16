pub struct ClawMachine {
    // button_a: (isize, isize),
    // button_b: (isize, isize),
    // prize: (isize, isize)
    a_x: isize,
    a_y: isize,
    b_x: isize,
    b_y: isize,
    r_x: isize,
    r_y: isize
}

// i * a_x + j * b_x = r_x
// i = (r_x - j * b_x) / a_x

// i * a_y + j * b_y = r_y
// (r_x * a_y - j * b_x * a_y) / a_x  + j * b_y = r_y
// (r_x * a_y) / a_x - (j * b_x * a_y) / a_x  + j * b_y = r_y
// - (j * b_x * a_y) / a_x  + j * b_y = r_y - (r_x * a_y) / a_x
// j * ( - (b_x * a_y) / a_x  + b_y ) = r_y - (r_x * a_y) / a_x
// j = (r_y - (r_x * a_y) / a_x) / ( - (b_x * a_y) / a_x  + b_y )
// j = ( a_x * r_y - r_x *a_y) / (-b_x * a_y + b_y * a_x)

impl ClawMachine {
    pub fn from_str(input: &str, offset: isize) -> ClawMachine {
        let mut lines = input.lines();
        let line = lines.next().unwrap();
        let mut numbers = line.strip_prefix("Button A: X+").unwrap().split(", Y+").map(|chunk| chunk.trim().parse::<isize>());
        let a_x = numbers.next().unwrap().unwrap();
        let a_y = numbers.next().unwrap().unwrap();
        let line = lines.next().unwrap();
        let mut numbers = line.strip_prefix("Button B: X+").unwrap().split(", Y+").map(|chunk| chunk.trim().parse::<isize>());
        let b_x = numbers.next().unwrap().unwrap();
        let b_y = numbers.next().unwrap().unwrap();
        let line = lines.next().unwrap();
        let mut numbers = line.strip_prefix("Prize: X=").unwrap().split(", Y=").map(|chunk| chunk.trim().parse::<isize>());
        let r_x = numbers.next().unwrap().unwrap() + offset;
        let r_y = numbers.next().unwrap().unwrap() + offset;

        ClawMachine {
            a_x,
            a_y,
            b_x,
            b_y,
            r_x,
            r_y
        }
    }

    pub fn many_from_str(input: &str, offset: isize) -> Vec<ClawMachine> {
        input.split("\n\n").map(|chunk| Self::from_str(chunk, offset) ).collect()
    }

    pub fn how_many_tokens(&self) -> Option<isize> {
        // (a_x*r_y - r_x *a_y) / (-b_x * a_y + b_y * a_x)
        let how_many_b = (self.a_x* self.r_y - self.r_x *self.a_y) / (-self.b_x * self.a_y + self.b_y * self.a_x);
        // (r_x - j * b_x) / a_x
        let how_many_a = (self.r_x - how_many_b * self.b_x) / self.a_x;

        // i * a_x + j * b_x = r_x
        // i * a_y + j * b_y = r_y
        let eq1 = how_many_a * self.a_x + how_many_b * self.b_x == self.r_x;
        let eq2 = how_many_a * self.a_y + how_many_b * self.b_y == self.r_y;


        (eq1 && eq2).then_some(how_many_a * 3 + how_many_b)
    }
}


fn step1(input: &str) -> usize {
    let machines = ClawMachine::many_from_str(input, 0);
    let res: isize = machines.iter().filter_map(|m| m.how_many_tokens()).sum();
    res as usize
}

fn step2(input: &str) -> usize {
    let machines = ClawMachine::many_from_str(input, 10000000000000);
    let res: isize = machines.iter().filter_map(|m| m.how_many_tokens()).sum();
    res as usize
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
            "Button A: X+94, Y+34"
            "Button B: X+22, Y+67"
            "Prize: X=8400, Y=5400"
            ""
            "Button A: X+26, Y+66"
            "Button B: X+67, Y+21"
            "Prize: X=12748, Y=12176"
            ""
            "Button A: X+17, Y+86"
            "Button B: X+84, Y+37"
            "Prize: X=7870, Y=6450"
            ""
            "Button A: X+69, Y+23"
            "Button B: X+27, Y+71"
            "Prize: X=18641, Y=10279"
        );
        assert_eq!(step1(input), 480)
    }
}