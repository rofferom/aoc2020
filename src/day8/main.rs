use std::collections::HashSet;
use std::fs;

const INPUT_KO_PATH: &str = "src/day8/input_ko.txt";
const INPUT_OK_PATH: &str = "src/day8/input_ok.txt";

fn parse_code(input: &str) -> Vec<(&str, i32)> {
    input
        .lines()
        .map(|l| {
            let s: Vec<&str> = l.split(' ').collect();
            (s[0], s[1].parse().unwrap())
        })
        .collect()
}

fn run(code: Vec<(&str, i32)>) -> (i32, bool) {
    let mut accumulator: i32 = 0;
    let mut pc: usize = 0;

    let mut known_pc: HashSet<usize> = HashSet::new();

    loop {
        if known_pc.contains(&pc) {
            println!("Loop detected. PC={}", pc);
            return (accumulator, true);
        } else if pc == code.len() {
            return (accumulator, false);
        } else {
            known_pc.insert(pc);
        }

        let (opcode, data) = code[pc];

        match opcode {
            "acc" => {
                println!("{:3} - {} {:+4}        A:{}", pc, opcode, data, accumulator);
                accumulator += data;
                pc += 1;
            }
            "jmp" => {
                let target_pc = if data > 0 {
                    pc + data as usize
                } else {
                    pc - data.abs() as usize
                };

                println!(
                    "{:3} - {} {:+4} [{:3}]  A:{}",
                    pc, opcode, data, target_pc, accumulator
                );

                pc = target_pc;
            }
            "nop" => {
                println!("{:3} - {} {:+4}        A:{}", pc, opcode, data, accumulator);
                pc += 1;
            }
            _ => {
                panic!("Unknown opcode {}", opcode);
            }
        }
    }
}

fn part1() -> i32 {
    let input = &fs::read_to_string(INPUT_KO_PATH).unwrap();
    let code = parse_code(input);

    let (accumulator, _) = run(code);
    accumulator
}

fn part2() -> i32 {
    let input = &fs::read_to_string(INPUT_KO_PATH).unwrap();
    let code = parse_code(input);

    for (pc, new_opcode) in code.iter().enumerate().filter_map(|(pc, (opcode, _))| {
        if *opcode == "jmp" {
            Some((pc, "nop"))
        } else if *opcode == "nop" {
            Some((pc, "jmp"))
        } else {
            None
        }
    }) {
        let mut patched_code = code.clone();
        patched_code[pc].0 = new_opcode;

        let (accumulator, loop_detected) = run(patched_code);
        if !loop_detected {
            println!(
                "PC: {} ({} {}) changed to {}",
                pc, code[pc].0, code[pc].1, new_opcode
            );

            return accumulator;
        }
    }

    0
}

fn main() {
    let part1_result = part1();
    let part2_result = part2();

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8_part1() {
        const INPUT: &'static str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        assert_eq!(run(parse_code(INPUT)), (5, true));
        assert_eq!(
            run(parse_code(&fs::read_to_string(INPUT_KO_PATH).unwrap())),
            (1941, true)
        );
    }

    #[test]
    fn day8_part2() {
        const INPUT: &'static str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
nop -4
acc +6";

        assert_eq!(run(parse_code(INPUT)), (8, false));
        assert_eq!(
            run(parse_code(&fs::read_to_string(INPUT_OK_PATH).unwrap())),
            (2096, false)
        );
    }
}
