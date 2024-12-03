use regex::Regex;

pub fn part_1(input: &str) -> i32 {
    let regex = Regex::new(r"mul\((?<x>\d+),(?<y>\d+)\)").unwrap();

    regex
        .captures_iter(input)
        .map(|caps| (caps["x"].parse::<i32>().unwrap()) * (caps["y"].parse::<i32>().unwrap()))
        .sum()
}

pub fn part_2(input: &str) -> i32 {
    let mut enable = true;
    let regex = Regex::new(r"(?<cmd>(mul|don't|do))\(((?<x>\d+),(?<y>\d+))?\)").unwrap();

    regex
        .captures_iter(input)
        .filter_map(|caps| match &caps["cmd"] {
            "mul" => {
                if enable {
                    if let (Some(x), Some(y)) = (caps.name("x"), caps.name("y")) {
                        Some(x.as_str().parse::<i32>().unwrap() * y.as_str().parse::<i32>().unwrap())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            "don't" => {
                enable = false;
                None
            }
            "do" => {
                enable = true;
                None
            }
            _ => None,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let s = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part_1(s), 161);
    }

    #[test]
    fn part2() {
        let s = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part_2(s), 48);
    }
}
