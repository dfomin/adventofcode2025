use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.trim().as_bytes().to_vec())
        .collect()
}

pub fn part1(input: &str) -> i64 {
    let field = parse(input);
    let mut indices: HashSet<usize> =
        HashSet::from_iter([field[0].iter().position(|&ch| ch == b'S').unwrap()]);
    let mut result = 0;
    for i in 1..field.len() {
        let mut new_indices = HashSet::new();
        for &index in &indices {
            if field[i][index] == b'^' {
                result += 1;
                if index > 0 {
                    new_indices.insert(index - 1);
                }
                if index + 1 < field[0].len() {
                    new_indices.insert(index + 1);
                }
            } else {
                new_indices.insert(index);
            }
        }
        indices = new_indices;
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let field = parse(input);
    let start_index = field[0].iter().position(|&ch| ch == b'S').unwrap();
    let mut indices: HashMap<usize, i64> = HashMap::new();
    indices.insert(start_index, 1);
    for i in 1..field.len() {
        let mut new_indices = HashMap::new();
        for (index, value) in indices {
            if field[i][index] == b'^' {
                if index > 0 {
                    *new_indices.entry(index - 1).or_insert(0) += value;
                }
                if index + 1 < field[0].len() {
                    *new_indices.entry(index + 1).or_insert(0) += value;
                }
            } else {
                *new_indices.entry(index).or_insert(0) += value;
            }
        }
        indices = new_indices;
    }
    indices.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ".......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ...............";

    #[test]
    fn test_day7_part1() {
        assert_eq!(part1(INPUT), 21);
    }

    #[test]
    fn test_day7_part2() {
        assert_eq!(part2(INPUT), 40);
    }
}
