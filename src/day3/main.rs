use std::fs;

const INPUT_PATH: &str = "src/day3/input.txt";

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    let mut map = vec![];

    for str_l in input.lines() {
        let mut line = vec![false; str_l.len()];

        for (idx, c) in str_l.chars().enumerate() {
            if c == '#' {
                line[idx] = true;
            }
        }

        map.push(line);
    }

    map
}

fn count_trees(map: &[Vec<bool>], right: usize, down: usize) -> u32 {
    let line_size = map[0].len();
    let mut count = 0;
    let mut x: usize = 0;
    let mut y: usize = 0;

    loop {
        x = (x + right) % line_size;
        y += down;

        if y >= map.len() {
            break;
        }

        if map[y][x] {
            count += 1;
        }
    }

    count
}

fn part1(input: &str) -> u32 {
    let map = parse_input(input);
    count_trees(&map, 3, 1)
}

fn part2(input: &str) -> u32 {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let map = parse_input(input);
    let mut result = 1;

    for (right, down) in slopes {
        result *= count_trees(&map, right, down);
    }

    result
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn day3_part1() {
        assert_eq!(part1(INPUT), 7);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 167);
    }

    #[test]
    fn day3_part2() {
        assert_eq!(part2(INPUT), 336);
        assert_eq!(part2(&fs::read_to_string(INPUT_PATH).unwrap()), 736527114);
    }
}
