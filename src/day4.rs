pub fn part_1(input: &str) -> i32 {
    const DIRECTIONS: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, -1),
        (-1, 1),
        (1, 1),
        (-1, -1),
    ];
    const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut xmas_count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == XMAS[0] {
                for d in &DIRECTIONS {
                    let mut present = true;
                    for (i, k) in XMAS.iter().skip(1).enumerate() {
                        let dx = x as i32 + d.0 * (i as i32 + 1);
                        let dy = y as i32 + d.1 * (i as i32 + 1);
                        if dx < 0
                            || dy < 0
                            || dx >= grid[0].len() as i32
                            || dy >= grid.len() as i32
                            || grid[dy as usize][dx as usize] != *k
                        {
                            present = false;
                            break;
                        }
                    }
                    if present {
                        xmas_count += 1;
                    }
                }
            }
        }
    }
    xmas_count
}

pub fn part_2(input: &str) -> i32 {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut xmas_count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'A' && x > 0 && y > 0 && x < grid[0].len() - 1 && y < grid.len() - 1 {
                let c1 = grid[y - 1][x - 1];
                let c2 = grid[y + 1][x + 1];
                let c3 = grid[y - 1][x + 1];
                let c4 = grid[y + 1][x - 1];
                if ((c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M'))
                    && ((c3 == 'M' && c4 == 'S') || (c3 == 'S' && c4 == 'M'))
                {
                    xmas_count += 1;
                }
            }
        }
    }
    xmas_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let s = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part_1(s), 18);
    }

    #[test]
    fn part2() {
        let s = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part_2(s), 9);
    }
}