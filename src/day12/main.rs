use std::fs;

const INPUT_PATH: &str = "src/day12/input.txt";

fn move_direction(x: i32, y: i32, direction: char, delta: i32) -> (i32, i32) {
    match direction {
        'N' => (x, y + delta),
        'S' => (x, y - delta),
        'E' => (x + delta, y),
        'W' => (x - delta, y),
        _ => {
            panic!("Unexpected action {}", direction);
        }
    }
}

fn rotate(x: i32, y: i32, angle: i32) -> (i32, i32) {
    match angle {
        90 => (-y, x),
        180 => (-x, -y),
        270 => (y, -x),
        _ => {
            panic!("Unexpected angle ")
        }
    }
}

fn distance(x: i32, y: i32) -> i32 {
    x.abs() + y.abs()
}

fn part1(input: &str) -> i32 {
    let instructions: Vec<(char, i32)> = input
        .lines()
        .map(|l| (l.chars().next().unwrap(), l[1..l.len()].parse().unwrap()))
        .collect();

    let mut waypoint_x: i32 = 1;
    let mut waypoint_y: i32 = 0;

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for (action, value) in instructions {
        match action {
            'N' | 'S' | 'E' | 'W' => {
                let delta = move_direction(x, y, action, value);
                x = delta.0;
                y = delta.1;
            }
            'L' => {
                let delta = rotate(waypoint_x, waypoint_y, value);
                waypoint_x = delta.0;
                waypoint_y = delta.1;
            }
            'R' => {
                let delta = rotate(waypoint_x, waypoint_y, 360 - value);
                waypoint_x = delta.0;
                waypoint_y = delta.1;
            }
            'F' => {
                x += waypoint_x * value;
                y += waypoint_y * value;
            }
            _ => {
                panic!("Unexpected action {}", action);
            }
        }
    }

    distance(x, y)
}

fn part2(input: &str) -> i32 {
    let instructions: Vec<(char, i32)> = input
        .lines()
        .map(|l| (l.chars().next().unwrap(), l[1..l.len()].parse().unwrap()))
        .collect();

    let mut waypoint_x: i32 = 10;
    let mut waypoint_y: i32 = 1;

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for (action, value) in instructions {
        match action {
            'N' | 'S' | 'E' | 'W' => {
                let delta = move_direction(waypoint_x, waypoint_y, action, value);
                waypoint_x = delta.0;
                waypoint_y = delta.1;
            }
            'L' => {
                let delta = rotate(waypoint_x, waypoint_y, value);
                waypoint_x = delta.0;
                waypoint_y = delta.1;
            }
            'R' => {
                let delta = rotate(waypoint_x, waypoint_y, 360 - value);
                waypoint_x = delta.0;
                waypoint_y = delta.1;
            }
            'F' => {
                x += waypoint_x * value;
                y += waypoint_y * value;
            }
            _ => {
                panic!("Unexpected action {}", action);
            }
        }
    }

    distance(x, y)
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "F10
N3
F7
R90
F11";

    #[test]
    fn day12_part1() {
        assert_eq!(part1(INPUT), 25);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 362);
    }

    #[test]
    fn day12_part2() {
        assert_eq!(part2(INPUT), 286);
        assert_eq!(part2(&fs::read_to_string(INPUT_PATH).unwrap()), 29895);
    }
}
