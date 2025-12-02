fn digits_count(mut n: i64) -> i64 {
    let mut result = 0;
    while n > 0 {
        n /= 10;
        result += 1;
    }
    result
}

fn count_interval(digits: i64, min: i64, max: i64) -> i64 {
    if digits % 2 == 1 {
        return 0;
    }

    let mut result = 0;
    let start = 10i64.pow(digits as u32 / 2 - 1);
    let stop = 10i64.pow(digits as u32 / 2);
    for i in start..stop {
        let n = i * 10i64.pow(digits as u32 / 2) + i;
        if n >= min && n <= max {
            result += n;
        }
    }
    result
}

fn check_interval(start: i64, stop: i64) -> i64 {
    let start_digits = digits_count(start);
    let stop_digits = digits_count(stop);
    let mut result = 0;
    for digits in start_digits..stop_digits + 1 {
        result += count_interval(digits, start, stop);
    }
    result
}

fn check_pattern(n: i64) -> bool {
    let digits = digits_count(n);
    for i in 2..digits + 1 {
        if digits % i != 0 {
            continue;
        }

        let modulo = 10i64.pow((digits / i) as u32);
        let mut repeat = true;
        let mut v = n;
        let pattern = v % modulo;
        v /= modulo;
        while v > 0 {
            if v % modulo != pattern {
                repeat = false;
                break;
            }
            v /= modulo;
        }
        if repeat {
            return true;
        }
    }
    false
}

fn count_all_patterns(start: i64, stop: i64) -> i64 {
    let mut result = 0;
    for i in start..stop + 1 {
        if check_pattern(i) {
            result += i;
        }
    }
    result
}

pub fn part1(input: &str) -> i64 {
    input
        .trim()
        .split(",")
        .map(|interval| {
            let (start, stop) = interval.split_once("-").unwrap();
            check_interval(start.parse().unwrap(), stop.parse().unwrap())
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    input
        .trim()
        .split(",")
        .map(|interval| {
            let (start, stop) = interval.split_once("-").unwrap();
            count_all_patterns(start.parse().unwrap(), stop.parse().unwrap())
        })
        .sum()
}

mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_day2_part1() {
        assert_eq!(part1(INPUT), 1227775554);
    }

    #[test]
    fn test_day2_part2() {
        assert_eq!(part2(INPUT), 4174379265);
    }
}
