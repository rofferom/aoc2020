use std::cmp::{max, min};
use std::fs;

const INPUT_PATH: &str = "src/day5/input.txt";

fn convert<T>(i: T, one: char) -> u32
where
    T: Iterator<Item = char>,
{
    i.fold(0, |acc, c| (acc << 1) | ((c == one) as u32))
}

fn get_seat_id(input: &str) -> u32 {
    let row = convert(input.chars().take(7), 'B');
    let column = convert(input.chars().skip(7), 'R');

    row * 8 + column
}

fn part1(input: &str) -> u32 {
    input.lines().map(|x| get_seat_id(x)).max().unwrap()
}

fn part2(input: &str) -> u32 {
    let valid_filter = |x| {
        const VALID_START: u32 = 8;
        const VALID_END: u32 = (8 * 127) - 1;

        VALID_START <= x && x <= VALID_END
    };

    let mut seat_start = std::u32::MAX;
    let mut seat_end = std::u32::MIN;
    let mut sum = 0;

    for seat in input
        .lines()
        .map(|x| get_seat_id(x))
        .filter(|&x| valid_filter(x))
    {
        seat_start = min(seat_start, seat);
        seat_end = max(seat_end, seat);

        sum += seat;
    }

    let sum_cb = |n| (n * (n + 1)) / 2;
    sum_cb(seat_end) - sum_cb(seat_start - 1) - sum
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
    fn day5_part1() {
        assert_eq!(get_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(get_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(get_seat_id("BBFFBBFRLL"), 820);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 864);
    }

    #[test]
    fn day5_part2() {
        assert_eq!(part2(&fs::read_to_string(INPUT_PATH).unwrap()), 739);
    }
}
