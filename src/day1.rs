use std::collections::HashMap;

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

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

pub fn part_2(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let (left, right) = input;

    let freq = right.iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    left.iter()
        .map(|l| l * freq.get(l).unwrap_or(&0))
        .sum::<i32>() // Let's hope this is enough :)
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

    #[test]
    fn part2() {
        let s = "3   4
        4   3
        2   5
        1   3
        3   9
        3   3";
        assert_eq!(part_2(&generator(s)), 31);
    }
}
