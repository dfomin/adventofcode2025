use ahash::{HashMap, HashMapExt, HashSet};

fn check(field: &[Vec<u8>], x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
    for x in x1..=x2 {
        for y in y1..=y2 {
            if field[y][x] != b'X' {
                return false;
            }
        }
    }
    true
}

pub fn part1(input: &str) -> i64 {
    let points: Vec<Vec<i64>> = input
        .lines()
        .map(|line| line.trim().split(",").map(|x| x.parse().unwrap()).collect())
        .collect();
    let mut result = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let x = (points[i][0] - points[j][0]).abs() + 1;
            let y = (points[i][1] - points[j][1]).abs() + 1;
            result = result.max(x * y);
        }
    }
    result
}
pub fn part2(input: &str) -> i64 {
    let points: Vec<Vec<i64>> = input
        .lines()
        .map(|line| line.trim().split(",").map(|x| x.parse().unwrap()).collect())
        .collect();

    let mut xs = Vec::from_iter(
        points
            .iter()
            .map(|point| point[0])
            .collect::<HashSet<_>>()
            .into_iter(),
    );
    xs.sort_unstable();
    let mut x_map: HashMap<i64, i64> = HashMap::new();
    for (i, x) in xs.into_iter().enumerate() {
        x_map.insert(x, i as i64);
    }

    let mut ys = Vec::from_iter(
        points
            .iter()
            .map(|point| point[1])
            .collect::<HashSet<_>>()
            .into_iter(),
    );
    ys.sort_unstable();
    let mut y_map: HashMap<i64, i64> = HashMap::new();
    for (i, y) in ys.into_iter().enumerate() {
        y_map.insert(y, i as i64);
    }

    let (width, height) = (x_map.len() + 1, y_map.len() + 1);

    let mut field = vec![vec![b'.'; width]; height];
    for i in 0..points.len() {
        let mut x1 = x_map[&points[i][0]];
        let mut y1 = y_map[&points[i][1]];
        let x2 = x_map[&points[(i + 1) % points.len()][0]];
        let y2 = y_map[&points[(i + 1) % points.len()][1]];
        let dx = if x1 < x2 {
            1
        } else if x1 > x2 {
            -1
        } else {
            0
        };
        let dy = if y1 < y2 {
            1
        } else if y1 > y2 {
            -1
        } else {
            0
        };
        while x1 != x2 || y1 != y2 {
            field[y1 as usize][x1 as usize] = b'X';
            x1 += dx;
            y1 += dy;
        }
    }
    for i in 0..height {
        let mut color = false;
        let mut j = 0;
        while j < width {
            if field[i][j] == b'X' {
                color = !color;
                while j < width && field[i][j] == b'X' {
                    j += 1;
                }
            }
            if j < width && color && field[i][j] == b'.' {
                field[i][j] = b'X';
            }
            j += 1;
        }
    }
    let mut result = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let x1 = x_map[&points[i][0]];
            let y1 = y_map[&points[i][1]];
            let x2 = x_map[&points[j][0]];
            let y2 = y_map[&points[j][1]];
            if check(
                &field,
                x1.min(x2) as usize,
                y1.min(y2) as usize,
                x1.max(x2) as usize,
                y1.max(y2) as usize,
            ) {
                let x = (points[i][0] - points[j][0]).abs() + 1;
                let y = (points[i][1] - points[j][1]).abs() + 1;
                result = result.max(x * y);
            }
        }
    }
    result
}

mod tests {
    use super::*;

    const INPUT: &str = "7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3";

    #[test]
    fn test_day9_part1() {
        assert_eq!(part1(INPUT), 50);
    }

    #[test]
    fn test_day9_part2() {
        assert_eq!(part2(INPUT), 24);
    }
}
