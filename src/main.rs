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

    day = 2;
    let input = adventofcode2025::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2025::day2::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2025::day2::part2(&input)
    );

    day = 3;
    let input = adventofcode2025::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2025::day3::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2025::day3::part2(&input)
    );

    day = 4;
    let input = adventofcode2025::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2025::day4::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2025::day4::part2(&input)
    );

    day = 5;
    let input = adventofcode2025::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2025::day5::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2025::day5::part2(&input)
    );
}
