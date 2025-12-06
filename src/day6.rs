pub fn part1(input: &str) -> i64 {
    let lines = input
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .filter(|part| !part.is_empty())
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<_>>();
    let mut result = 0;
    for i in 0..lines[0].len() {
        if lines[lines.len() - 1][i] == "+" {
            let mut subresult = 0;
            for j in 0..lines.len() - 1 {
                subresult += lines[j][i].parse::<i64>().unwrap();
            }
            result += subresult;
        } else {
            let mut subresult = 1;
            for j in 0..lines.len() - 1 {
                subresult *= lines[j][i].parse::<i64>().unwrap();
            }
            result += subresult;
        }
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let lines: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let mut result = 0;
    for i in 0..lines[0].len() {
        if lines[lines.len() - 1][i] != b' ' {
            let mut subresult = if lines[lines.len() - 1][i] == b'+' {
                0
            } else {
                1
            };
            let mut col = i;
            while col < lines[0].len() {
                let mut number = 0;
                for j in 0..lines.len() - 1 {
                    if lines[j][col] != b' ' {
                        number *= 10;
                        number += (lines[j][col] - b'0') as i64;
                    }
                }
                if number == 0 {
                    break;
                }
                if lines[lines.len() - 1][i] == b'+' {
                    subresult += number;
                } else {
                    subresult *= number;
                }
                col += 1;
            }
            result += subresult;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_day6_part1() {
        assert_eq!(part1(INPUT), 4277556);
    }

    #[test]
    fn test_day6_part2() {
        assert_eq!(part2(INPUT), 3263827);
    }
}
