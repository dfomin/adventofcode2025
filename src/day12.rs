#[derive(Debug)]
struct Present {
    pattern: Vec<Vec<bool>>,
    count: usize,
}

fn fit(region: (usize, usize), counts: &[usize], presents: &[Present]) -> bool {
    let region_size = region.0 * region.1;
    let presents_size = counts
        .iter()
        .enumerate()
        .map(|(index, count)| presents[index].count * count)
        .sum::<usize>();
    presents_size <= region_size
}

pub fn part1(input: &str) -> i64 {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let presents: Vec<Present> = parts[..parts.len() - 1]
        .iter()
        .map(|part| {
            let p: Vec<Vec<bool>> = part
                .lines()
                .skip(1)
                .map(|line| {
                    line.trim()
                        .as_bytes()
                        .into_iter()
                        .map(|&ch| ch == b'#')
                        .collect()
                })
                .collect();
            let count = p
                .iter()
                .map(|line| line.iter().filter(|&&ch| ch).count())
                .sum();
            Present { pattern: p, count }
        })
        .collect();
    let regions: Vec<(usize, usize, Vec<usize>)> = parts[parts.len() - 1]
        .lines()
        .map(|line| line.trim().split_once(": ").unwrap())
        .map(|(a, b)| {
            let size = a.split_once("x").unwrap();
            let w = size.0.parse().unwrap();
            let h = size.1.parse().unwrap();
            let counts = b.split(" ").map(|c| c.parse().unwrap()).collect();
            (w, h, counts)
        })
        .collect();
    regions
        .iter()
        .filter(|&region| fit((region.0, region.1), &region.2, &presents))
        .count() as i64
}

pub fn part2(input: &str) -> i64 {
    0
}

mod tests {
    use super::*;

    const INPUT: &str = "0:
        ###
        ##.
        ##.

        1:
        ###
        ##.
        .##

        2:
        .##
        ###
        ##.

        3:
        ##.
        ###
        ##.

        4:
        ###
        #..
        ###

        5:
        ###
        .#.
        ###

        4x4: 0 0 0 0 2 0
        12x5: 1 0 1 0 2 2
        12x5: 1 0 1 0 3 2";

    #[test]
    fn test_day12_part1() {
        assert_eq!(part1(INPUT), 2);
    }

    #[test]
    fn test_day12_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
