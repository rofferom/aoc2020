use std::collections::HashMap;
use std::fs;

const INPUT_PATH: &str = "src/day14/input.txt";

fn part1(input: &str) -> u64 {
    let mut mask: Option<&str> = None;
    let mut map: HashMap<u64, u64> = HashMap::new();

    let mut sum: u64 = 0;

    for l in input.lines() {
        if l.starts_with("mask") {
            mask = Some(&l[7..l.len()]);
        } else {
            let mask = mask.unwrap();

            let closing_bracket = l.find(']').unwrap();
            let addr: u64 = l[4..closing_bracket].parse().unwrap();
            let mut value: u64 = l[closing_bracket + 4..l.len()].parse().unwrap();

            for (idx, b) in mask.chars().enumerate().filter(|(_, x)| *x != 'X') {
                let idx = mask.len() - idx - 1;

                if b == '1' {
                    value |= 1 << idx;
                } else {
                    value &= !(1 << idx);
                }

                let entry = map.entry(addr).or_insert(0);
                *entry = value;
            }
        }
    }

    sum += map.values().sum::<u64>();

    sum
}

fn part2(input: &str) -> u64 {
    let mut mask: Option<&str> = None;
    let mut map: HashMap<u64, u64> = HashMap::new();

    let mut sum: u64 = 0;

    for l in input.lines() {
        if l.starts_with("mask") {
            mask = Some(&l[7..l.len()]);
        } else {
            // Parse
            let mask = mask.unwrap();
            let closing_bracket = l.find(']').unwrap();
            let value: u64 = l[closing_bracket + 4..l.len()].parse().unwrap();
            let mut addr: u64 = l[4..closing_bracket].parse().unwrap();

            // Set 1 bits
            for (idx, b) in mask.chars().enumerate().filter(|(_, x)| *x != 'X') {
                let idx = mask.len() - idx - 1;

                if b == '1' {
                    addr |= 1 << idx;
                }
            }

            // Generate addresses
            let mut bit_map = vec![];
            for (idx, _) in mask.chars().enumerate().filter(|(_, x)| *x == 'X') {
                bit_map.push(mask.len() - idx - 1);
            }

            for i in 0..(1 << bit_map.len()) {
                for bit_idx in 0..bit_map.len() {
                    addr = if i & (1 << (bit_map.len() - bit_idx - 1)) != 0 {
                        addr | (1 << bit_map[bit_idx])
                    } else {
                        addr & !(1 << bit_map[bit_idx])
                    };
                }

                let entry = map.entry(addr).or_insert(0);
                *entry = value;
            }
        }
    }

    sum += map.values().sum::<u64>();

    sum
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_part1() {
        const INPUT: &'static str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

        assert_eq!(part1(INPUT), 165);
        assert_eq!(
            part1(&fs::read_to_string(INPUT_PATH).unwrap()),
            7817357407588
        );
    }

    #[test]
    fn day14_part2() {
        const INPUT: &'static str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

        assert_eq!(part2(INPUT), 208);
        assert_eq!(
            part2(&fs::read_to_string(INPUT_PATH).unwrap()),
            4335927555692
        );
    }
}
