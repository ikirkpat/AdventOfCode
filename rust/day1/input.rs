use std::io::{self, BufRead};

pub fn get_instructions() -> Vec<(char, i32)> {
    let instructions: Vec<(char, i32)> = read_raw_instructions()
        .into_iter()
        .map(|instr| parse_instruction(&instr))
        .filter(|instr| instr.is_some())
        .map(|instr| instr.unwrap())
        .collect();

    return instructions;
}

fn read_raw_instructions() -> Vec<String> {
    let stdin = io::stdin();
    let mut lines: Vec<String> = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if line.trim() != "" {
            lines.push(line);
        } else {
            break;
        }
    }

    return lines;
}

fn parse_instruction(instruction: &str) -> Option<(char, i32)> {
    let (first, rest) = instruction.split_at(1);
    let letter = first.chars().next();
    if letter.is_none() {
        return None;
    }

    let number = rest.parse::<i32>().ok();
    if number.is_none() {
        return None;
    }

    Some((letter.unwrap().to_ascii_lowercase(), number.unwrap()))
}
