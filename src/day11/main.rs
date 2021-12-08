use std::fs;

const INPUT_PATH: &str = "src/day11/input.txt";

const EMPTY: char = 'L';
const FLOOR: char = '.';
const OCCUPIED: char = '#';

#[derive(Clone)]
struct Seats {
    seats: Vec<Vec<char>>,
    columns: i32,
    rows: i32,
}

impl Seats {
    fn new(input: &str) -> Self {
        let seats: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

        Self {
            columns: seats[0].len() as i32,
            rows: seats.len() as i32,
            seats,
        }
    }

    fn is_valid_pos(&self, row: i32, col: i32) -> bool {
        0 <= row && row < self.rows && 0 <= col && col < self.columns
    }

    fn occupied_count(&self) -> u32 {
        self.seats
            .iter()
            .flatten()
            .filter(|&&x| x == OCCUPIED)
            .count() as u32
    }
}

type GetOccupiedCount = fn(seats: &Seats, row: i32, column: i32) -> usize;

fn run_round(
    seats: &Seats,
    get_occupied_cb: GetOccupiedCount,
    occupied_thresold: usize,
) -> (Seats, bool) {
    let mut new_seats = seats.clone();
    let mut changes = false;

    for (row_idx, row) in seats.seats.iter().enumerate() {
        for (column_idx, state) in row.iter().enumerate() {
            if *state == FLOOR {
                continue;
            }

            let new_state = &mut new_seats.seats[row_idx as usize][column_idx as usize];
            let occupieds = get_occupied_cb(seats, row_idx as i32, column_idx as i32);

            if occupieds == 0 && *state != OCCUPIED {
                *new_state = OCCUPIED;
                changes = true;
            } else if occupieds >= occupied_thresold && *state != EMPTY {
                *new_state = EMPTY;
                changes = true;
            }
        }
    }

    (new_seats, changes)
}

fn run(input: &str, get_occupied_cb: GetOccupiedCount, occupied_thresold: usize) -> u32 {
    let mut seats = Seats::new(input);

    loop {
        let (new_seats, changes) = run_round(&seats, get_occupied_cb, occupied_thresold);
        if !changes {
            break;
        }

        seats = new_seats;
    }

    seats.occupied_count()
}

fn part1(input: &str) -> u32 {
    let get_occupied_count = |seats: &Seats, row: i32, column: i32| -> usize {
        let positions = [
            (row - 1, column - 1),
            (row - 1, column),
            (row - 1, column + 1),
            (row, column - 1),
            (row, column + 1),
            (row + 1, column - 1),
            (row + 1, column),
            (row + 1, column + 1),
        ];

        positions
            .iter()
            .filter(|(row, column)| {
                seats.is_valid_pos(*row, *column)
                    && seats.seats[*row as usize][*column as usize] == OCCUPIED
            })
            .count()
    };

    run(input, get_occupied_count, 4)
}

fn part2(input: &str) -> u32 {
    let get_occupied_count = |seats: &Seats, row: i32, column: i32| -> usize {
        let mut occupieds = 0;

        let moves = [
            (-1, 0),  // Left
            (1, 0),   // Right
            (0, 1),   // Up
            (0, -1),  // Down
            (-1, -1), // Up left
            (-1, 1),  // Down left
            (1, -1),  // Up right
            (1, 1),   // Down lefright
        ];

        for (move_row, move_column) in moves {
            let mut next_row = row + move_row;
            let mut next_column = column + move_column;

            while seats.is_valid_pos(next_row, next_column) {
                let state = seats.seats[next_row as usize][next_column as usize];

                if state == EMPTY {
                    break;
                } else if state == OCCUPIED {
                    occupieds += 1;
                    break;
                } else {
                    next_row += move_row;
                    next_column += move_column;
                }
            }
        }

        occupieds
    };

    run(input, get_occupied_count, 5)
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn day11_part1() {
        assert_eq!(part1(INPUT), 37);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 2183);
    }

    #[test]
    fn day11_part2() {
        assert_eq!(part2(INPUT), 26);
        assert_eq!(part2(&fs::read_to_string(INPUT_PATH).unwrap()), 1990);
    }
}
