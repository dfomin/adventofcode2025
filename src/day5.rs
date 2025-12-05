pub fn part1(input: &str) -> i64 {
    let (first, second) = input.split_once("\n\n").unwrap();
    let ranges: Vec<(i64, i64)> = first
        .lines()
        .map(|line| line.trim())
        .map(|line| {
            line.split_once("-")
                .map(|x| (x.0.parse().unwrap(), x.1.parse().unwrap()))
                .unwrap()
        })
        .collect();
    second
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .filter(|&id| {
            for range in &ranges {
                if range.0 <= id && id <= range.1 {
                    return true;
                }
            }
            false
        })
        .count() as i64
}

pub fn part2(input: &str) -> i64 {
    let mut result = 0;
    let mut ranges: Vec<(i64, i64)> = input
        .lines()
        .map(|line| line.trim())
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split_once("-")
                .map(|x| (x.0.parse().unwrap(), x.1.parse().unwrap()))
                .unwrap()
        })
        .collect();
    ranges.sort_unstable_by_key(|x| x.1);
    let mut last = ranges[0];
    for range in ranges.into_iter().skip(1) {
        if last.1 < range.0 {
            result += last.1 - last.0 + 1;
            last = range;
        } else {
            last = (last.0.min(range.0), last.1.max(range.1));
        }
    }
    result += last.1 - last.0 + 1;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32";

    #[test]
    fn test_day5_part1() {
        assert_eq!(part1(INPUT), 3);
    }

    #[test]
    fn test_day5_part2() {
        assert_eq!(part2(INPUT), 14);
    }
}
