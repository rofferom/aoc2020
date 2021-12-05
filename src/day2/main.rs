use std::fs;

const INPUT_PATH: &str = "src/day2/input.txt";

type Checker = fn(password: &str, c: char, val_one: u32, val_two: u32) -> bool;

fn check_passwords(input: &str, checker: Checker) -> u32 {
    let mut valid_count = 0;

    for l in input.lines() {
        // Get password
        let t: Vec<&str> = l.split(": ").collect();
        let password = t[1];

        // Get char policy
        let t: Vec<&str> = t[0].split(' ').collect();
        let c = t[1].chars().nth(0).unwrap();

        // Get char count policy
        let t: Vec<u32> = t[0].split('-').map(|x| x.parse().unwrap()).collect();
        let (val_one, val_two) = (t[0], t[1]);

        // Check
        if checker(password, c, val_one, val_two) {
            valid_count += 1;
        }
    }

    valid_count
}

fn part1(input: &str) -> u32 {
    let checker = |password: &str, c: char, val_one: u32, val_two: u32| {
        let char_count = password.chars().filter(|&x| x == c).count() as u32;
        (val_one..val_two + 1).contains(&char_count)
    };

    check_passwords(input, checker)
}

fn part2(input: &str) -> u32 {
    let checker = |password: &str, c: char, val_one: u32, val_two: u32| {
        let v = vec![
            password.chars().nth((val_one - 1) as usize).unwrap(),
            password.chars().nth((val_two - 1) as usize).unwrap(),
        ];

        v.iter().filter(|&&x| x == c).count() == 1
    };

    check_passwords(input, checker)
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn day2_part1() {
        assert_eq!(part1(INPUT), 2);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 469);
    }

    #[test]
    fn day2_part2() {
        assert_eq!(part2(INPUT), 1);
        assert_eq!(part2(&fs::read_to_string(INPUT_PATH).unwrap()), 267);
    }
}
