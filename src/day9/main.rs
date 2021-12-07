use std::fs;

const INPUT_PATH: &str = "src/day9/input.txt";

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn find_needle(values: &[u64], needle: u64) -> bool {
    let mut values: Vec<_> = values.iter().filter(|&&x| x < needle).copied().collect();
    values.sort_unstable();

    for idx in 0..(values.len() - 2) {
        if values
            .iter()
            .skip(idx + 1)
            .any(|&x| x + values[idx] == needle)
        {
            return true;
        }
    }

    false
}

fn part1(input: &str, window_size: usize) -> u64 {
    let values = parse_input(input);

    for i in window_size..values.len() {
        let prev_values = &values[i - window_size..i];
        let needle = values[i];

        if !find_needle(prev_values, needle) {
            return needle;
        }
    }

    0
}

fn part2(input: &str, window_size: usize) -> u64 {
    let values = parse_input(input);
    let needle = part1(input, window_size);

    for i in 0..values.len() {
        let mut sum = 0;

        for j in i..(values.len() - 1) {
            sum += values[j];

            if sum == needle {
                let range = &values[i..(j + 1)];
                return range.iter().min().unwrap() + range.iter().max().unwrap();
            } else if sum > needle {
                break;
            }
        }
    }

    0
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input, 25));
    println!("Part 2: {}", part2(&input, 25));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn day9_part1() {
        assert_eq!(part1(INPUT, 5), 127);
        assert_eq!(
            part1(&fs::read_to_string(INPUT_PATH).unwrap(), 25),
            1212510616
        );
    }

    #[test]
    fn day9_part2() {
        assert_eq!(part2(INPUT, 5), 62);
        assert_eq!(
            part2(&fs::read_to_string(INPUT_PATH).unwrap(), 25),
            171265123
        );
    }
}
