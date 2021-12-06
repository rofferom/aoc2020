use std::collections::{HashMap, HashSet};
use std::fs;

const INPUT_PATH: &str = "src/day7/input.txt";
const BAGNAME: &str = "shiny gold";

fn parse_desc(input: &str) -> (String, Vec<(u32, String)>) {
    let split: Vec<&str> = input.split(" contain ").collect();

    let name = String::from(&split[0][0..(split[0].len() - 5)]);

    let parse_cb = |input: &str| {
        if input.ends_with("no other bags") {
            return None;
        };

        let count: u32 = input.chars().nth(0).unwrap().to_digit(10).unwrap();

        if input.ends_with("bags") {
            Some((count, String::from(&input[2..(input.len() - 5)])))
        } else {
            Some((count, String::from(&input[2..(input.len() - 4)])))
        }
    };

    let content = split[1]
        .split(", ")
        .filter_map(|x| parse_cb(&x.replace(".", "")))
        .collect();

    (name, content)
}

fn parse_input(input: &str) -> HashMap<String, Vec<(u32, String)>> {
    let mut map: HashMap<String, Vec<(u32, String)>> = HashMap::new();

    for l in input.lines() {
        let (name, content) = parse_desc(l);
        map.insert(name, content);
    }

    map
}

fn get_bag_types(map: &HashMap<String, Vec<(u32, String)>>, name: &str) -> HashSet<String> {
    let mut types: HashSet<String> = HashSet::new();

    if let Some(content) = map.get(name) {
        for (_, content) in content {
            let content_types = get_bag_types(map, content);

            types = types.union(&content_types).cloned().collect();
            types.insert(content.clone());
        }
    };

    types
}

fn part1(input: &str, mybag: &str) -> u32 {
    let map = parse_input(input);

    let mut counter = 0;

    for (name, _) in &map {
        if name == mybag {
            continue;
        }

        let bag_content = get_bag_types(&map, name);
        if bag_content.contains(mybag) {
            counter += 1;
        }
    }

    counter
}

fn get_bag_content(map: &HashMap<String, Vec<(u32, String)>>, name: &str) -> u32 {
    let mut counter = 0;

    for (subbag_count, subbag_name) in map.get(name).unwrap() {
        counter += (1 + get_bag_content(map, subbag_name)) * subbag_count;
    }

    counter
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
