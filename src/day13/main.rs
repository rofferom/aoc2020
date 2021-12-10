use std::fs;

const INPUT_PATH: &str = "src/day13/input.txt";

fn part1(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();

    let ts: u32 = lines[0].parse().unwrap();
    let bus_ids: Vec<u32> = lines[1]
        .split(',')
        .filter_map(|x| {
            if x != "x" {
                Some(x.parse().unwrap())
            } else {
                None
            }
        })
        .collect();

    let (id, wait) = bus_ids
        .iter()
        .map(|id| (id, id - (ts % id)))
        .min_by(|(_, x), (_, y)| x.cmp(y))
        .unwrap();

    id * wait
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13_part1() {
        const INPUT: &'static str = "939
7,13,x,x,59,x,31,19";

        assert_eq!(part1(INPUT), 295);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 2406);
    }
}
