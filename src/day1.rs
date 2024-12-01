pub fn generator(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1]))
        .unzip()
}

pub fn part_1(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let (mut left, mut right) = input.clone();
    left.sort_unstable();
    right.sort_unstable();

    left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let s = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(part_1(&generator(s)), 11);
    }
}