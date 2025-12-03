use rayon::{iter::ParallelIterator, str::ParallelString};

fn move_index(indices: &mut [usize], start_index: usize, index: usize, line: &[i64]) {
    for i in (start_index..indices[index]).rev() {
        if line[i] >= line[indices[index]] {
            indices[index] = i;
        }
    }
}

fn max_value(line: &[i64], size: usize) -> i64 {
    let mut indices = (line.len() - size..line.len()).collect::<Vec<_>>();
    for i in (0..indices[0] as usize).rev() {
        move_index(&mut indices, i, 0, line);
        for i in 1..indices.len() {
            let start_index = indices[i - 1] + 1;
            move_index(&mut indices, start_index, i, line);
        }
    }
    let mut result = 0;
    for index in indices {
        result *= 10;
        result += line[index];
    }
    result
}

pub fn part1(input: &str) -> i64 {
    input
        .trim()
        .par_lines()
        .map(|line| {
            max_value(
                &line
                    .trim()
                    .as_bytes()
                    .iter()
                    .map(|ch| (ch - b'0') as i64)
                    .collect::<Vec<_>>(),
                2,
            )
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    input
        .trim()
        .par_lines()
        .map(|line| {
            max_value(
                &line
                    .trim()
                    .as_bytes()
                    .iter()
                    .map(|ch| (ch - b'0') as i64)
                    .collect::<Vec<_>>(),
                12,
            )
        })
        .sum()
}

mod tests {
    use super::*;

    const INPUT: &str = "987654321111111
        811111111111119
        234234234234278
        818181911112111";

    #[test]
    fn test_day3_part1() {
        assert_eq!(part1(INPUT), 357);
    }

    #[test]
    fn test_day3_part2() {
        assert_eq!(part2(INPUT), 3121910778619);
    }
}
