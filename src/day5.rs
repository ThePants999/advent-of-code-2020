pub fn day5(input_lines: &[String]) -> (u64, u64) {
    let mut passes: Vec<u64> = input_lines.iter().map(|line| decode_boarding_pass(line)).collect();
    passes.sort_unstable();
    let part1 = *passes.last().unwrap();
    let part2 = passes.into_iter().fold(0, |current, new| {
        if (new == (current + 1)) || (current == 0) {
            new
        } else {
            current
        }
    }) + 1;
    (part1,part2)
}

fn decode_boarding_pass(pass: &str) -> u64 {
    let binary = pass.chars().map(|c| match c {
        'B' | 'R' => '1',
        _ => '0',
    }).collect::<String>();
    u64::from_str_radix(&binary, 2).unwrap()    
}
