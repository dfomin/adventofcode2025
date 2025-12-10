use std::i64;

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
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
    let mut values = vec![vec![false; target.len()]];
    let mut used: HashSet<Vec<bool>> = HashSet::new();
    used.insert(values[0].clone());
    let mut step = 0;
    while !values.is_empty() {
        let mut new_values: Vec<Vec<bool>> = vec![];
        for value in &values {
            if *value == target {
                return step;
            }
            for op in &ops {
                let mut new_value = value.clone();
                for &index in op {
                    new_value[index] = !new_value[index];
                }
                if !used.contains(&new_value) {
                    new_values.push(new_value.clone());
                    used.insert(new_value);
                }
            }
        }
        values = new_values;
        step += 1;
    }
    -1
}

fn split(n: i64, count: usize, cache: &mut HashMap<(i64, usize), Vec<Vec<i64>>>) -> Vec<Vec<i64>> {
    if cache.contains_key(&(n, count)) {
        return cache[&(n, count)].clone();
    }
    let mut result = vec![];
    for i in 0..n + 1 {
        if n - i == 0 && count - 1 == 0 {
            result.push(vec![i]);
        } else if count - 1 > 0 {
            let sub = split(n - i, count - 1, cache);
            for mut v in sub {
                v.push(i);
                result.push(v);
            }
        }
    }
    cache.insert((n, count), result.clone());
    result
}

fn count_joltage(
    target: &mut Vec<i64>,
    ops: Vec<Vec<usize>>,
    cur: i64,
    cache: &mut HashMap<(i64, usize), Vec<Vec<i64>>>,
) -> i64 {
    if target.iter().sum::<i64>() == 0 {
        return cur;
    }
    if ops.is_empty() {
        return i64::MAX;
    }
    let mut ops_for_target = vec![0; target.len()];
    for op in &ops {
        for &index in op {
            ops_for_target[index] += 1;
        }
    }
    let ops_indices: Vec<(i64, usize)> = ops_for_target
        .iter()
        .enumerate()
        .filter(|(_, v)| **v > 0)
        .map(|(i, &v)| (v, i))
        .collect();
    let best_index = ops_indices.iter().min().unwrap().1;
    let mut use_ops_indices = vec![];
    let mut result = i64::MAX;
    for (i, op) in ops.iter().enumerate() {
        if op.contains(&best_index) {
            use_ops_indices.push(i);
        }
    }
    for comb in split(target[best_index], use_ops_indices.len(), cache) {
        for (c, &op_index) in comb.iter().zip(use_ops_indices.iter()) {
            for &index in &ops[op_index] {
                target[index] -= c;
            }
        }
        let v = cur + comb.iter().sum::<i64>();
        if target.iter().all(|&x| x >= 0) && v < result {
            let mut new_ops = vec![];
            for (i, op) in ops.iter().enumerate() {
                if !use_ops_indices.contains(&i) {
                    new_ops.push(op.clone());
                }
            }
            let sub = count_joltage(target, new_ops, v, cache);
            result = result.min(sub);
        }
        for (c, &op_index) in comb.iter().zip(use_ops_indices.iter()) {
            for &index in &ops[op_index] {
                target[index] += c;
            }
        }
    }
    result
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
    let mut cache = HashMap::new();
    for line in lines {
        let mut target: Vec<i64> = line[line.len() - 1][1..line[line.len() - 1].len() - 1]
            .split(",")
            .map(|v| v.parse().unwrap())
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
        let sub = count_joltage(&mut target, ops, 0, &mut cache);
        result += sub;
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
