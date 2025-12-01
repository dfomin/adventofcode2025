fn main() {
    let mut day = 1;
    let input = adventofcode2025::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2025::day1::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2025::day1::part2(&input)
    );
}
