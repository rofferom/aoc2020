use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::RangeInclusive;

const INPUT_PATH: &str = "src/day16/input.txt";

#[derive(Debug)]
struct Field<'a> {
    class: &'a str,
    ranges: Vec<RangeInclusive<u32>>,
}

struct ParsedInput<'a> {
    fields: Vec<Field<'a>>,
    myticket: Vec<u32>,
    nearby_valids: Vec<Vec<u32>>,
    nearby_invalids: Vec<(Vec<u32>, u32)>,
}

fn parse_input(input: &str) -> ParsedInput {
    let v: Vec<_> = input.split("\n\n").collect();

    let str_fields = v[0];
    let str_myticket = &v[1][13..];
    let str_nearby_tickets = &v[2][16..];

    // Parse fields
    let mut fields: Vec<Field> = Vec::new();

    for field in str_fields.lines() {
        let v: Vec<_> = field.split(": ").collect();

        let class = v[0];

        let ranges: Vec<_> = v[1]
            .split(" or ")
            .map(|x| {
                let values: Vec<u32> = x.split('-').map(|x| x.parse().unwrap()).collect();

                RangeInclusive::new(values[0], values[1])
            })
            .collect();

        fields.push(Field {
            class: class,
            ranges,
        });
    }

    // Parse my ticket
    let myticket: Vec<u32> = str_myticket
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    // Parse nearby fields
    let mut nearby_valids: Vec<Vec<u32>> = vec![];
    let mut nearby_invalids: Vec<(Vec<u32>, u32)> = vec![];

    for l in str_nearby_tickets.lines() {
        let ticket: Vec<u32> = l.split(',').map(|x| x.parse().unwrap()).collect();
        let mut invalid_field: Option<u32> = None;

        for v in &ticket {
            let mut found = false;

            for field in &fields {
                let valid = field
                    .ranges
                    .iter()
                    .filter(|range| range.contains(v))
                    .count()
                    > 0;

                if valid {
                    found = true;
                    break;
                }
            }

            if !found {
                invalid_field = Some(*v);
                break;
            }
        }

        if let Some(invalid_field) = invalid_field {
            nearby_invalids.push((ticket, invalid_field));
        } else {
            nearby_valids.push(ticket);
        }
    }

    ParsedInput {
        fields,
        myticket,
        nearby_valids,
        nearby_invalids,
    }
}

fn part1(input: &str) -> u32 {
    let input = parse_input(input);

    return input
        .nearby_invalids
        .iter()
        .map(|(_, invalid_field)| invalid_field)
        .sum();
}

fn part2(input: &str) -> u64 {
    let input = parse_input(input);
    let mut remaining_fields: HashSet<_> = (0..input.fields.len()).collect();
    let mut remaining_columns: HashSet<_> = (0..input.fields.len()).collect();

    let mut field_map: HashMap<usize, usize> = HashMap::new();

    while !remaining_columns.is_empty() {
        let mut found_fields: Vec<(usize, usize, &Field)> = vec![];

        for col_idx in &remaining_columns {
            let values: Vec<u32> = input
                .nearby_valids
                .iter()
                .map(|ticket| ticket[*col_idx])
                .collect();

            let mut fields_scores: Vec<(usize, usize, &Field)> = vec![];

            for field_idx in &remaining_fields {
                let field = &input.fields[*field_idx];

                let matches = values.iter().all(|v| {
                    field
                        .ranges
                        .iter()
                        .filter(|range| range.contains(v))
                        .count()
                        == 1
                });

                if matches {
                    fields_scores.push((*field_idx, *col_idx, field));
                }
            }

            if fields_scores.len() == 1 {
                found_fields.push(fields_scores[0]);
            }
        }

        for (field_idx, col_idx, _) in found_fields {
            field_map.insert(field_idx, col_idx);

            remaining_fields.remove(&field_idx);
            remaining_columns.remove(&col_idx);
        }
    }

    input
        .fields
        .iter()
        .enumerate()
        .filter_map(|(idx, field)| {
            if field.class.starts_with("departure") {
                Some(input.myticket[*field_map.get(&idx).unwrap() as usize] as u64)
            } else {
                None
            }
        })
        .product()
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
    fn day16_part1() {
        const INPUT: &'static str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

        assert_eq!(part1(INPUT), 71);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 27802);
    }

    #[test]
    fn day16_part2() {
        assert_eq!(
            part2(&fs::read_to_string(INPUT_PATH).unwrap()),
            279139880759
        );
    }
}
