use std::collections::HashSet;
use std::fs;

const INPUT_PATH: &str = "src/day4/input.txt";

const REQUIRED_FIELDS: usize = 7;

#[derive(Clone, Debug)]
struct Field {
    name: String,
    data: String,
}

impl Field {
    fn new(s: &str) -> Self {
        let fields: Vec<&str> = s.split(':').collect();

        Self {
            name: String::from(fields[0]),
            data: String::from(fields[1]),
        }
    }
}

fn read_passports(input: &str) -> Vec<Vec<Field>> {
    let mut passports = vec![];

    for raw_passport in input.split("\n\n") {
        let raw_passport = str::replace(raw_passport, "\n", " ");
        passports.push(raw_passport.split(' ').map(|x| Field::new(x)).collect());
    }

    let passport_valid =
        |fields: &Vec<Field>| fields.iter().filter(|&x| x.name != "cid").count() == REQUIRED_FIELDS;

    passports
        .iter()
        .filter(|x: &&Vec<Field>| passport_valid(x))
        .cloned()
        .collect()
}

fn part1(input: &str) -> u32 {
    read_passports(input).len() as u32
}

fn part2(input: &str) -> u32 {
    let passports = read_passports(input);

    let int_in_range = |v: usize, begin: usize, end: usize| (begin..end + 1).contains(&v);

    let int_field_valid = |field: &Field, begin: usize, end: usize| {
        int_in_range(field.data.parse().unwrap(), begin, end)
    };

    let hcl_map = HashSet::from([
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ]);

    let ecl_map = HashSet::from(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]);

    let pid_map = HashSet::from(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);

    let field_valid = |field: &Field| match field.name.as_str() {
        "byr" => int_field_valid(field, 1920, 2002),
        "iyr" => int_field_valid(field, 2010, 2020),
        "eyr" => int_field_valid(field, 2020, 2030),
        "hgt" => {
            let get_value = || -> usize { field.data[0..field.data.len() - 2].parse().unwrap() };

            if field.data.ends_with("cm") {
                int_in_range(get_value(), 150, 193)
            } else if field.data.ends_with("in") {
                int_in_range(get_value(), 59, 76)
            } else {
                false
            }
        }
        "hcl" => {
            field.data.starts_with('#')
                && field
                    .data
                    .chars()
                    .skip(1)
                    .filter(|x| !hcl_map.contains(x))
                    .count()
                    == 0
        }
        "ecl" => ecl_map.contains(field.data.as_str()),
        "pid" => {
            field.data.len() == 9
                && field.data.chars().filter(|x| !pid_map.contains(x)).count() == 0
        }
        "cid" => false,
        _ => {
            panic!("Unexpected field {}", field.name);
        }
    };

    let passport_valid =
        |fields: &Vec<Field>| fields.iter().filter(|&x| field_valid(x)).count() == REQUIRED_FIELDS;

    passports
        .iter()
        .filter(|x: &&Vec<Field>| passport_valid(x))
        .count() as u32
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
    fn day4_part1() {
        const INPUT: &'static str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        assert_eq!(part1(INPUT), 2);
        assert_eq!(part1(&fs::read_to_string(INPUT_PATH).unwrap()), 250);
    }

    #[test]
    fn day4_part2() {
        const INPUT: &'static str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        assert_eq!(part2(INPUT), 4);
        assert_eq!(part2(&fs::read_to_string(INPUT_PATH).unwrap()), 158);
    }
}
