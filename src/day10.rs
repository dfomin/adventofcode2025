use ahash::{HashSet, HashSetExt};
use rayon::{iter::ParallelIterator, str::ParallelString};

fn process_line(line: &[&str]) -> i64 {
    let target = line[0].as_bytes()[1..line[0].len() - 1]
        .iter()
        .map(|&ch| ch == b'#')
        .collect::<Vec<_>>();
    let ops: Vec<Vec<usize>> = line[1..line.len() - 1]
        .iter()
        .map(|part| {
            part[1..part.len() - 1]
                .split(",")
                .map(|v| v.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    let mut values = vec![(vec![false; target.len()], vec![0; target.len()])];
    let mut used: HashSet<(Vec<bool>, Vec<i64>)> = HashSet::new();
    used.insert(values[0].clone());
    let mut step = 0;
    while !values.is_empty() {
        let mut new_values: Vec<(Vec<bool>, Vec<i64>)> = vec![];
        for (value, cur_joltage) in &values {
            if *value == target {
                return step;
            }
            for op in &ops {
                let mut new_value = value.clone();
                let mut new_joltage = cur_joltage.clone();
                for &index in op {
                    new_value[index] = !new_value[index];
                }
                let new_record = (new_value, new_joltage.clone());
                if !used.contains(&new_record) {
                    new_values.push(new_record.clone());
                    used.insert(new_record);
                }
            }
        }
        values = new_values;
        step += 1;
    }
    -1
}

pub fn part1(input: &str) -> i64 {
    let lines = input
        .lines()
        .map(|line| line.trim().split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut result = 0;
    for line in lines {
        result += process_line(&line);
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let lines = input
        .par_lines()
        .map(|line| line.trim().split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut result = 0;
    for line in lines {
        result += process_line(&line);
    }
    result
}

mod tests {
    use super::*;

    const INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_day10_part1() {
        assert_eq!(part1(INPUT), 7);
    }

    #[test]
    fn test_day10_part2() {
        assert_eq!(part2(INPUT), 33);
    }
}
