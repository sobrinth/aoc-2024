use itertools::Itertools;

pub fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|r| {
            let mut ok = false;
            ok |= r.iter().tuple_windows().all(|(a, b)| a < b);
            ok |= r.iter().tuple_windows().all(|(a, b)| a > b);
            let low_diff= r.iter().tuple_windows().all(|(a, b)| (a - b).abs() <= 3);
            ok && low_diff
        })
        .filter(|x| *x)
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let s = "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";
        assert_eq!(part_1(s), 2);
    }
}
