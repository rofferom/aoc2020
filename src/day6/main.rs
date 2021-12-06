use std::collections::{HashMap, HashSet};
use std::fs;

const INPUT_PATH: &str = "src/day6/input.txt";

fn part1(input: &str) -> u32 {
    let mut count = 0;

    for group in input.split("\n\n").map(|x| x.to_string()) {
        count += group
            .replace("\n", "")
            .chars()
            .collect::<HashSet<char>>()
            .len() as u32;
    }

    count
}

fn part2(input: &str) -> u32 {
    let mut count = 0;

    for group in input.split("\n\n").map(|x| x.to_string()) {
        let oneline_group = group.replace("\n", "");
        let group_size = group.len() - oneline_group.len() + 1;

        let mut map: HashMap<char, usize> = HashMap::new();
        for c in oneline_group.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        count += map.iter().filter(|(_, &v)| v == group_size).count() as u32;
    }

    count
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn day6_part1() {
        assert_eq!(part1(INPUT), 11);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 6534);
    }

    #[test]
    fn day6_part2() {
        assert_eq!(part2(INPUT), 6);
        assert_eq!(part2(&fs::read_to_string(INPUT_PATH).unwrap()), 3402);
    }
}
