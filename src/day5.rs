pub fn day5(input_lines: &[String]) -> (u64, u64) {
    let mut passes: Vec<u64> = input_lines.iter().map(|line| BoardingPass::decode(line)).map(|pass| pass.seat_id()).collect();
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

struct BoardingPass {
    row: u64,
    column: u64,
}

impl BoardingPass {
    fn decode(seat: &str) -> Self {
        Self {
            row: Self::decode_part(&seat[0..7]),
            column: Self::decode_part(&seat[7..]),
        }
    }

    fn decode_part(part: &str) -> u64 {
        let binary = part.chars().map(|c| match c {
            'B' | 'R' => '1',
            _ => '0',
        }).collect::<String>();
        u64::from_str_radix(&binary, 2).unwrap()
    }

    fn seat_id(&self) -> u64 {
        (self.row * 8) + self.column
    }
}