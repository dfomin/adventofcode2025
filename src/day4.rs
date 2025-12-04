fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.trim().as_bytes().iter().cloned().collect())
        .collect()
}

fn count(field: &[Vec<u8>], i: usize, j: usize) -> i64 {
    let mut result = 0;
    for y in (i as i64 - 1).max(0) as usize..(i + 2).min(field.len()) {
        for x in (j as i64 - 1).max(0) as usize..(j + 2).min(field[0].len()) {
            if y == i && x == j {
                continue;
            }
            if field[y][x] == b'@' {
                result += 1;
            }
        }
    }
    result
}

pub fn part1(input: &str) -> i64 {
    let mut result = 0;
    let field = parse(input);
    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if field[i][j] == b'@' && count(&field, i, j) < 4 {
                result += 1;
            }
        }
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let mut field = parse(input);
    let mut result = 0;
    let mut found = true;
    while found {
        found = false;
        for i in 0..field.len() {
            for j in 0..field[i].len() {
                if field[i][j] == b'@' && count(&field, i, j) < 4 {
                    result += 1;
                    field[i][j] = b'.';
                    found = true;
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.";

    #[test]
    fn test_day4_part1() {
        assert_eq!(part1(INPUT), 13);
    }

    #[test]
    fn test_day4_part2() {
        assert_eq!(part2(INPUT), 43);
    }
}
