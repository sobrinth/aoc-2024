use itertools::Itertools;

pub fn gen(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        }).collect::<Vec<Vec<i32>>>()
}

pub fn part_1(input: &Vec<Vec<i32>>) -> i32 {
    input
        .iter()
        .fold(0, |mut acc, r| {
            let mut ok = false;
            ok |= r.iter().tuple_windows().all(|(a, b)| a < b);
            ok |= r.iter().tuple_windows().all(|(a, b)| a > b);
            let low_diff = r.iter().tuple_windows().all(|(a, b)| (a - b).abs() <= 3);
            if ok && low_diff { acc += 1 }
            acc
        })
}

pub fn part_2(input: &Vec<Vec<i32>>) -> i32 {
    input
        .iter()
        .fold(0, |mut acc, r| {
            if (0..r.len()).any(|i| {
                let mut rem = r.to_vec();
                rem.remove(i);
                let mut ok = false;
                ok |= rem.iter().tuple_windows().all(|(a, b)| a < b);
                ok |= rem.iter().tuple_windows().all(|(a, b)| a > b);
                let low_diff = rem.iter().tuple_windows().all(|(a, b)| (a - b).abs() <= 3);
                ok && low_diff
            }) { acc += 1 }
            acc
        })
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
        assert_eq!(part_1(&gen(s)), 2);
    }

    #[test]
    fn part2() {
        let s = "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";
        assert_eq!(part_2(&gen(s)), 4);
    }
}
