use std::fs;

const INPUT_PATH: &str = "src/day1/input.txt";
const TARGET: u32 = 2020;

fn part1(input: &str) -> u32 {
    let mut numbers: Vec<u32> = input.lines().map(|x| x.parse().unwrap()).collect();
    numbers.sort_unstable();

    for i in 0..numbers.len() {
        for j in (0..numbers.len()).rev() {
            if numbers[i] + numbers[j] == TARGET {
                return numbers[i] * numbers[j];
            } else if numbers[i] + numbers[j] < TARGET {
                break;
            }
        }
    }

    0
}

fn part2(input: &str) -> u32 {
    let mut numbers: Vec<u32> = input.lines().map(|x| x.parse().unwrap()).collect();
    numbers.sort_unstable();

    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            for k in j + 1..numbers.len() {
                let sum = numbers[i] + numbers[j] + numbers[k];

                if sum == TARGET {
                    return numbers[i] * numbers[j] * numbers[k];
                } else if sum > TARGET {
                    break;
                }
            }
        }
    }

    0
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "1721
979
366
299
675
1456";

    #[test]
    fn day1_part1() {
        assert_eq!(part1(INPUT), 514579);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 157059);
    }

    #[test]
    fn day1_part2() {
        assert_eq!(part2(INPUT), 241861950);
        assert_eq!(part2(&fs::read_to_string(INPUT_PATH).unwrap()), 165080960);
    }
}
