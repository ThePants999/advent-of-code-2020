use std::iter;

pub fn day8(input_lines: &[String]) -> (u64, u64) {
    let part1 = match execute_program(input_lines, None) {
        ProgramResult::Loop(acc) => acc,
        ProgramResult::Exit(_) => unreachable!("Part 1 didn't infinite loop!"),
        ProgramResult::Crash => unreachable!("Crashed in part 1!"),
    };
    let mut part2 = 0u64;
    for index in 0..input_lines.len() {
        match execute_program(input_lines, Some(index)) {
            ProgramResult::Loop(_) => (),
            ProgramResult::Crash => (),
            ProgramResult::Exit(acc) => { part2 = acc; break; },
        }
    }
    (part1,part2)
}

enum ProgramResult {
    Loop(u64),
    Exit(u64),
    Crash,
}

fn execute_program(instructions: &[String], change_line: Option<usize>) -> ProgramResult {
    let mut accumulator: u64 = 0;
    let mut instruction_ptr: usize = 0;
    let mut visited_lines: Vec<bool> = iter::repeat(false).take(instructions.len()).collect();
    while instruction_ptr < instructions.len() && !visited_lines[instruction_ptr] {
        visited_lines[instruction_ptr] = true;
        let flip = change_line == Some(instruction_ptr);
        match Instructions::decode(&instructions[instruction_ptr], flip) {
            Instructions::Noop => { instruction_ptr += 1; },
            Instructions::AddToAccumulator(delta) => { accumulator += delta; instruction_ptr += 1; },
            Instructions::SubtractFromAccumulator(delta) => { 
                if delta > accumulator { return ProgramResult::Crash; }
                accumulator -= delta;
                instruction_ptr += 1;
            },
            Instructions::JumpForwards(offset) => { instruction_ptr += offset; },
            Instructions::JumpBackwards(offset) => { 
                if offset > instruction_ptr { return ProgramResult::Crash; }
                instruction_ptr -= offset;
            },
        }
    }
    if instruction_ptr < instructions.len() {
        ProgramResult::Loop(accumulator)
    } else {
        ProgramResult::Exit(accumulator)
    }
}

enum Instructions {
    Noop,
    AddToAccumulator(u64),
    SubtractFromAccumulator(u64),
    JumpForwards(usize),
    JumpBackwards(usize),
}

impl Instructions {
    fn decode(input_line: &str, flip: bool) -> Self {
        let sign = input_line.chars().nth(4).expect("Invalid input");
        let argument: u64 = input_line[5..].parse().expect("Invalid input");
        match &input_line[0..3] {
            "nop" if !flip => Self::Noop,
            "jmp" if flip => Self::Noop,
            "nop" | "jmp" => if sign == '+' { Self::JumpForwards(argument as usize) } else { Self::JumpBackwards(argument as usize) },
            "acc" => if sign == '+' { Self::AddToAccumulator(argument) } else { Self::SubtractFromAccumulator(argument) },
            _ => unreachable!("Invalid input"),
        }
    }
}