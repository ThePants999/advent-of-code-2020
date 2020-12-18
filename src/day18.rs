pub fn day18(input_lines: &[String]) -> (u64, u64) {
    let part1: u64 = input_lines.iter().map(|line| parse_eval(line, Precedence::LeftToRight).0).sum();
    let part2: u64 = input_lines.iter().map(|line| parse_eval(line, Precedence::Addition).0).sum();
    (part1,part2)
}

#[derive(Clone,Copy)]
enum Precedence {
    LeftToRight,
    Addition,
}

#[derive(Clone,Copy)]
enum Operator {
    Add,
    Multiply,
}

struct Operation {
    operator: Operator,
    operand: u64,
}

// Returns the result of an expression, and how many bytes long that expression was.
fn parse_eval(input: &str, precedence: Precedence) -> (u64, usize) {
    let chars: Vec<(usize, char)> = input.char_indices().collect();
    let mut operations: Vec<Operation> = Vec::new();
    let mut operator = Operator::Add;
    let mut bytes_consumed = 0usize;
    let mut skip_to_target = 0usize;

    for (index, c) in chars {
        bytes_consumed = index + c.len_utf8();
        if index < skip_to_target { continue; }
        match c {
            ' ' => continue,
            '(' => {
                let (operand, bytes_to_skip) = parse_eval(&input[bytes_consumed..], precedence);
                operations.push(Operation { operator, operand });
                skip_to_target = bytes_consumed + bytes_to_skip;
            },
            ')' => break,
            '+' => operator = Operator::Add,
            '*' => operator = Operator::Multiply,
            _ => {
                let operand = c.to_digit(10).expect("Invalid input") as u64;
                operations.push(Operation { operator, operand });
            },
        }
    }

    let value = match precedence {
        Precedence::LeftToRight => { operations.into_iter().fold(0, evaluate) },
        Precedence::Addition => {
            let mut values_to_multiply: Vec<u64> = Vec::new();
            let mut current_value = 0u64;
            for op in operations {
                match op.operator {
                    Operator::Add => current_value += op.operand,
                    Operator::Multiply => { values_to_multiply.push(current_value); current_value = op.operand; },
                }                
            }
            values_to_multiply.push(current_value);
            values_to_multiply.into_iter().product()
        },
    };

    (value, bytes_consumed)
}

fn evaluate(lhs: u64, op: Operation) -> u64 {
    match op.operator {
        Operator::Add => lhs + op.operand,
        Operator::Multiply => lhs * op.operand,
    }
}
