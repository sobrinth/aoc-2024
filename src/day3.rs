use regex::Regex;

pub fn part_1(input: &str) -> i32 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    regex.captures_iter(input)
        .map(|caps| {
            (caps[1].parse::<i32>().unwrap()) * (caps[2].parse::<i32>().unwrap())
        })
        .sum()
}

pub fn part_2(input: &str) -> i32 {
    let mut enable = true;
    let regex = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();
    
    regex.captures_iter(input)
        .filter_map(|caps| {
            if let (Some(x), Some(y)) = (caps.get(1), caps.get(2)) {
                if enable {
                    Some((x.as_str().parse::<i32>().unwrap()) * (y.as_str().parse::<i32>().unwrap()))
                }
                else { None }
            } else { 
                match &caps[0] { 
                    "don't()" => enable = false,
                    "do()" => enable = true,
                    _=> {}
                }
                None
            }
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