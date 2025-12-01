pub fn part1(input: &str) -> i64 {
    let lines = input.lines().map(|line| line.trim());
    let mut value = 50;
    let mut result = 0;
    for line in lines {
        if line.as_bytes()[0] == b'L' {
            value = (value - line[1..].parse::<i64>().unwrap()).rem_euclid(100);
        } else {
            value = (value + line[1..].parse::<i64>().unwrap()).rem_euclid(100);
        }

        if value == 0 {
            result += 1;
        }
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let lines = input.lines().map(|line| line.trim());
    let mut value = 50;
    let mut result = 0;
    for line in lines {
        let rotation = line[1..].parse::<i64>().unwrap();
        if line.as_bytes()[0] == b'L' {
            if rotation >= value {
                if value > 0 {
                    result += 1;
                    result += (rotation - value) / 100;
                } else {
                    result += rotation / 100;
                }
            }
            value = (value - rotation).rem_euclid(100);
        } else {
            if rotation >= 100 - value {
                if value > 0 {
                    result += 1;
                    result += (rotation - (100 - value)) / 100;
                } else {
                    result += rotation / 100;
                }
            }
            value = (value + rotation).rem_euclid(100);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82";

    #[test]
    fn test_day1_part1() {
        assert_eq!(part1(INPUT), 3);
    }

    #[test]
    fn test_day1_part2() {
        assert_eq!(part2(INPUT), 6);
    }
}
