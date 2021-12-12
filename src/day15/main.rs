use std::collections::HashMap;
use std::fs;

const INPUT_PATH: &str = "src/day15/input.txt";

fn solve(input: &str, turns: u32) -> u32 {
    let mut values: HashMap<u32, u32> = HashMap::new();
    let mut last_spoken: u32 = 0;
    let mut turn = 0;

    for v in input.split(',') {
        let v = v.parse().unwrap();
        values.insert(v, turn);

        last_spoken = v;
        turn += 1;
    }

    values.remove(&last_spoken);

    while turn < turns {
        if let Some(entry) = values.get_mut(&last_spoken) {
            last_spoken = (turn - 1) - *entry;
            *entry = turn - 1;
        } else {
            values.insert(last_spoken, turn - 1);
            last_spoken = 0;
        }

        turn += 1;
    }

    last_spoken
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", solve(&input, 2020));
    println!("Part 2: {}", solve(&input, 30000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_part1() {
        assert_eq!(solve("0,3,6", 10), 0);
        assert_eq!(solve("1,3,2", 2020), 1);
        assert_eq!(solve("2,1,3", 2020), 10);
        assert_eq!(solve("1,2,3", 2020), 27);
        assert_eq!(solve("2,3,1", 2020), 78);
        assert_eq!(solve("3,2,1", 2020), 438);
        assert_eq!(solve("3,1,2", 2020), 1836);

        assert_eq!(solve(&fs::read_to_string(INPUT_PATH).unwrap(), 2020), 1238);
    }

    #[test]
    fn day15_part2() {
        assert_eq!(solve("0,3,6", 30000000), 175594);
        assert_eq!(solve("1,3,2", 30000000), 2578);
        assert_eq!(solve("2,1,3", 30000000), 3544142);
        assert_eq!(solve("1,2,3", 30000000), 261214);
        assert_eq!(solve("2,3,1", 30000000), 6895259);
        assert_eq!(solve("3,2,1", 30000000), 18);
        assert_eq!(solve("3,1,2", 30000000), 362);
        assert_eq!(
            solve(&fs::read_to_string(INPUT_PATH).unwrap(), 30000000),
            3745954
        );
    }
}
