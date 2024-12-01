pub fn part_1(input: &str) -> i32 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1]))
        .unzip();

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

    fn part1() {
        let s = "3   4
4   3
2   5
1   3
3   9
3   3";
        let a = part_1(s);
        println!("part1: {a:?}");
    }
}