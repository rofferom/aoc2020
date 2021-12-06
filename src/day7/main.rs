use std::collections::HashMap;
use std::fs;

const INPUT_PATH: &str = "src/day7/input.txt";
const BAGNAME: &str = "shiny gold";

fn parse_subbags(input: &str) -> Option<(u32, &str)> {
    if !input.ends_with("no other bags.") {
        let count: u32 = input.chars().next().unwrap().to_digit(10).unwrap();

        let input = if input.ends_with('.') {
            &input[0..(input.len() - 1)]
        } else {
            input
        };

        if input.ends_with("bags") {
            Some((count, &input[2..(input.len() - 5)]))
        } else {
            Some((count, &input[2..(input.len() - 4)]))
        }
    } else {
        None
    }
}

fn parse_desc(input: &str) -> (&str, Vec<(u32, &str)>) {
    let split: Vec<&str> = input.split(" contain ").collect();

    let name = &split[0][0..(split[0].len() - 5)];

    let content = split[1]
        .split(", ")
        .filter_map(|x| parse_subbags(x))
        .collect();

    (name, content)
}

fn parse_input(input: &str) -> HashMap<&str, Vec<(u32, &str)>> {
    input.lines().map(|x| parse_desc(x)).collect()
}

fn find_bag(map: &HashMap<&str, Vec<(u32, &str)>>, name: &str, needle: &str) -> bool {
    if let Some(content) = map.get(name) {
        content
            .iter()
            .any(|(_, content)| *content == needle || find_bag(map, content, needle))
    } else {
        false
    }
}

fn get_bag_content(map: &HashMap<&str, Vec<(u32, &str)>>, name: &str) -> u32 {
    map.get(name).unwrap().iter().fold(0, |acc, (count, name)| {
        acc + (1 + get_bag_content(map, name)) * count
    })
}

fn part1(input: &str, mybag: &str) -> u32 {
    let map = parse_input(input);

    map.iter()
        .filter(|(&name, _)| name != mybag)
        .filter(|(&name, _)| find_bag(&map, name, mybag))
        .count() as u32
}

fn part2(input: &str, mybag: &str) -> u32 {
    let map = parse_input(input);

    get_bag_content(&map, mybag)
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input, BAGNAME));
    println!("Part 2: {}", part2(&input, BAGNAME));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &'static str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
bright aqua bags contain 5 plaid magenta bags, 5 muted lavender bags, 4 dim turquoise bags, 1 shiny turquoise bag.
striped lavender bags contain 4 striped gold bags, 3 mirrored olive bags, 2 dim lime bags, 1 muted indigo bag.";

    #[test]
    fn day7_part1() {
        assert_eq!(part1(INPUT1, BAGNAME), 4);
        assert_eq!(
            part1(&fs::read_to_string(INPUT_PATH).unwrap(), BAGNAME),
            248
        );
    }

    #[test]
    fn day7_part2() {
        const INPUT2: &'static str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        assert_eq!(part2(INPUT1, BAGNAME), 32);
        assert_eq!(part2(INPUT2, BAGNAME), 126);
        assert_eq!(
            part2(&fs::read_to_string(INPUT_PATH).unwrap(), BAGNAME),
            57281
        );
    }
}
