use regex::Regex;

pub fn part_1(input: &str) -> i32 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    regex.captures_iter(input)
        .map(|caps| {
            (caps[1].parse::<i32>().unwrap()) * (caps[2].parse::<i32>().unwrap())
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
}