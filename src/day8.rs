use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(PartialEq)]
struct MinNonNan(f64);

impl Eq for MinNonNan {}

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn union(v1: usize, v2: usize, components: &mut [usize], sizes: &mut [usize]) {
    let p1 = find(v1, components);
    let p2 = find(v2, components);
    if p1 != p2 {
        let (p1, p2) = (p1.min(p2), p1.max(p2));
        components[p2] = p1;
        sizes[p1] += sizes[p2];
        sizes[p2] = 0;
    }
}

fn find(v: usize, components: &mut [usize]) -> usize {
    if v != components[v] {
        let p = find(components[v], components);
        components[v] = p;
    }
    components[v]
}

fn connect(input: &str, connections: i64) -> i64 {
    let points: Vec<Vec<i64>> = input
        .lines()
        .map(|line| line.trim().split(",").map(|v| v.parse().unwrap()).collect())
        .collect();
    let mut components = (0..points.len()).collect::<Vec<_>>();
    let mut sizes = vec![1; points.len()];
    let mut distances = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let distance = (points[i]
                .iter()
                .zip(points[j].iter())
                .map(|(a, b)| (a - b) * (a - b))
                .sum::<i64>() as f64)
                .sqrt();
            distances.push((MinNonNan(distance), i, j));
        }
    }
    let mut heap = BinaryHeap::from(distances);
    for _ in 0..connections {
        let (_, i, j) = heap.pop().unwrap();
        union(i, j, &mut components, &mut sizes);
    }
    let mut size_heap = BinaryHeap::from(sizes);
    let mut result = 1;
    for _ in 0..3 {
        result *= size_heap.pop().unwrap();
    }
    result as i64
}

pub fn part1(input: &str) -> i64 {
    connect(input, 1000)
}

pub fn part2(input: &str) -> i64 {
    let points: Vec<Vec<i64>> = input
        .lines()
        .map(|line| line.trim().split(",").map(|v| v.parse().unwrap()).collect())
        .collect();
    let mut components = (0..points.len()).collect::<Vec<_>>();
    let mut sizes = vec![1; points.len()];
    let mut distances = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let distance = (points[i]
                .iter()
                .zip(points[j].iter())
                .map(|(a, b)| (a - b) * (a - b))
                .sum::<i64>() as f64)
                .sqrt();
            distances.push((MinNonNan(distance), i, j));
        }
    }
    let mut heap = BinaryHeap::from(distances);
    let mut result = 0;
    while sizes[0] != sizes.len() {
        let (_, i, j) = heap.pop().unwrap();
        result = points[i][0] * points[j][0];
        union(i, j, &mut components, &mut sizes);
    }
    result
}

mod tests {
    use super::*;

    const INPUT: &str = "162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689";

    #[test]
    fn test_day8_part1() {
        assert_eq!(connect(INPUT, 10), 40);
    }

    #[test]
    fn test_day8_part2() {
        assert_eq!(part2(INPUT), 25272);
    }
}
