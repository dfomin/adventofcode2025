use crate::Day;

pub struct Day1 {
    lines: Vec<String>,
}

impl Day for Day1 {
    fn new(input: &str) -> Self {
        Self {
            lines: input
                .lines()
                .map(|line| line.trim().to_string())
                .collect::<Vec<_>>(),
        }
    }

    fn part1(&self) -> i64 {
        self.lines
            .iter()
            .fold((50, 0), |acc, line| {
                let mut value = acc.0;
                if line.as_bytes()[0] == b'L' {
                    value = (value - line[1..].parse::<i64>().unwrap()).rem_euclid(100);
                } else {
                    value = (value + line[1..].parse::<i64>().unwrap()).rem_euclid(100);
                }

                if value == 0 {
                    (value, acc.1 + 1)
                } else {
                    (value, acc.1)
                }
            })
            .1
    }

    fn part2(&self) -> i64 {
        self.lines
            .iter()
            .fold((50, 0), |acc, line| {
                let rotation = line[1..].parse::<i64>().unwrap();
                let mut value = acc.0;
                let mut result = acc.1;
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
                (value, result)
            })
            .1
    }
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
        let day = Day1::new(INPUT);
        assert_eq!(day.part1(), 3);
    }

    #[test]
    fn test_day1_part2() {
        let day = Day1::new(INPUT);
        assert_eq!(day.part2(), 6);
    }
}
