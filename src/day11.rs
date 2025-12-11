use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};

fn dfs(graph: &HashMap<&str, Vec<&str>>, key: &str) -> i64 {
    if key == "out" {
        return 1;
    }
    let mut result = 0;
    for target in &graph[key] {
        result += dfs(graph, target);
    }
    result
}

pub fn part1(input: &str) -> i64 {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    input
        .lines()
        .map(|line| line.trim().split_once(": ").unwrap())
        .fold(&mut graph, |acc, (first, second)| {
            acc.entry(first).or_default().extend(second.split(" "));
            acc
        });
    dfs(&graph, "you")
}

fn sort(
    graph: &HashMap<&str, Vec<&str>>,
    visited: &mut HashSet<String>,
    key: &str,
    result: &mut Vec<String>,
) {
    for target in graph.get(key).unwrap_or(&vec![]) {
        if visited.contains(&target.to_string()) {
            continue;
        }
        visited.insert(target.to_string());
        sort(graph, visited, target, result);
        result.push(target.to_string())
    }
}

fn dp(graph: &HashMap<&str, Vec<&str>>, sorted: &Vec<String>) -> i64 {
    let mut dp = vec![0; sorted.len()];
    dp[0] = 1;
    let mut start_index = 0;
    for i in 1..dp.len() {
        for j in start_index..i {
            let start = &sorted[j];
            let end = &sorted[i];
            if graph[&start[..]].contains(&&end[..]) {
                dp[i] += dp[j];
            }
        }
        if sorted[i] == "dac" || sorted[i] == "fft" {
            start_index = i
        }
    }
    *dp.last().unwrap()
}

pub fn part2(input: &str) -> i64 {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    input
        .lines()
        .map(|line| line.trim().split_once(": ").unwrap())
        .fold(&mut graph, |acc, (first, second)| {
            acc.entry(first).or_default().extend(second.split(" "));
            acc
        });
    let mut sorted = vec![];
    let mut visited = HashSet::new();
    sort(&graph, &mut visited, "svr", &mut sorted);
    sorted.push("svr".to_string());
    sorted.reverse();
    dp(&graph, &sorted)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "aaa: you hhh
        you: bbb ccc
        bbb: ddd eee
        ccc: ddd eee fff
        ddd: ggg
        eee: out
        fff: out
        ggg: out
        hhh: ccc fff iii
        iii: out";

    const INPUT2: &str = "svr: aaa bbb
        aaa: fft
        fft: ccc
        bbb: tty
        tty: ccc
        ccc: ddd eee
        ddd: hub
        hub: fff
        eee: dac
        dac: fff
        fff: ggg hhh
        ggg: out
        hhh: out";

    #[test]
    fn test_day11_part1() {
        assert_eq!(part1(INPUT), 5);
    }

    #[test]
    fn test_day11_part2() {
        assert_eq!(part2(INPUT2), 2);
    }
}
