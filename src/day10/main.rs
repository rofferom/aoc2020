use std::fs;

const INPUT_PATH: &str = "src/day10/input.txt";

fn parse_input(input: &str) -> Vec<u32> {
    let mut values: Vec<u32> = input.lines().map(|x| x.parse().unwrap()).collect();
    values.push(0);
    values.sort_unstable();
    values.push(values[values.len() - 1] + 3);
    values
}

fn part1(input: &str) -> u32 {
    let values = parse_input(input);

    let mut ones = 0;
    let mut threes = 0;

    for i in 0..(values.len() - 1) {
        let delta = values[i + 1] - values[i];

        if delta == 1 {
            ones += 1;
        } else if delta == 3 {
            threes += 1;
        }
    }

    ones * threes
}

fn valid_arrangements(values: &mut [Option<u32>]) -> u64 {
    let values_count = values.len();

    // Assume this function is always called with valid values
    let mut count = 1;

    for i in 0..(values_count - 1) {
        // Skip if None
        if values[i].is_none() {
            continue;
        }

        // Get the next valid index
        let next_idx_offset = values[(i + 1)..values_count]
            .iter()
            .enumerate()
            .find(|(_, &x)| x.is_some());

        // Stop if no more item available
        let next_idx = {
            if let Some((offset, _)) = next_idx_offset {
                i + 1 + offset
            } else {
                break;
            }
        };

        // No change possible, at this point
        if values[next_idx].unwrap() - values[i].unwrap() == 3 {
            continue;
        }

        // Try to check if the values are still valid with the next element removed.
        // In this case, try to check the number of valid arrangements in this new branch.
        if next_idx + 1 < values.len() && values[next_idx + 1].unwrap() - values[i].unwrap() <= 3 {
            let backup = values[next_idx];
            values[next_idx] = None;

            count += valid_arrangements(&mut values[i..values_count]);

            values[next_idx] = backup;
        }
    }

    count
}

fn compute_gaps(values: &[u32]) -> Vec<usize> {
    let mut distances = vec![];

    for i in 0..(values.len() - 1) {
        distances.push(values[i + 1] - values[i]);
    }

    distances
        .iter()
        .enumerate()
        .filter_map(
            |(idx, &distance)| {
                if distance == 3 {
                    Some(idx)
                } else {
                    None
                }
            },
        )
        .collect()
}

fn part2(input: &str) -> u64 {
    let values = parse_input(input);

    // Split into subproblems
    //
    // A subproblem is a slice that distance of 3 with the previous and the next subproblem.
    // It means that each subproblem can't have any impact with the other ones, so it can be
    // resolved independently.
    //
    // The final result is the product of all the subproblem results.
    let mut start_idx = 0;
    let mut result = 1;

    let gaps = compute_gaps(&values);

    for gap in gaps {
        // Get problem len
        let problem_len = (gap - start_idx) + 1;

        // Solve subproblem
        let mut suproblem: Vec<_> = values
            .iter()
            .skip(start_idx)
            .take(problem_len)
            .map(|&x| Some(x))
            .collect();

        result *= valid_arrangements(&mut suproblem);

        // Move to next subproblem
        start_idx += problem_len;
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

    const INPUT1: &'static str = "16
10
15
5
1
11
7
19
6
12
4";

    const INPUT2: &'static str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn day10_part1() {
        assert_eq!(part1(INPUT1), 7 * 5);
        assert_eq!(part1(INPUT2), 22 * 10);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 2812);
    }

    #[test]
    fn day10_part2() {
        assert_eq!(part2(INPUT1), 8);
        assert_eq!(part2(INPUT2), 19208);
        assert_eq!(
            part2(&fs::read_to_string(INPUT_PATH).unwrap()),
            386869246296064
        );
    }
}
