const MAX_TURN: usize = 30000000;

pub fn day15(input_lines: &[String]) -> (u64, u64) {
    let starting_nums: Vec<usize> = input_lines[0].split(',').map(|num| num.parse::<usize>().expect("Invald input")).collect();
    let mut last_seen = vec![0; MAX_TURN].into_boxed_slice();
    for (index, num) in starting_nums[0..starting_nums.len()-1].iter().enumerate() {
        last_seen[*num] = index + 1;
    }
    let mut last_number = *starting_nums.last().unwrap();
    let mut part1 = 0u64;
    for turn in starting_nums.len()+1..=MAX_TURN {
        let number = match last_seen[last_number] {
            0 => 0,
            turn_last_seen => turn - turn_last_seen - 1,
        };
        last_seen[last_number] = turn-1;
        last_number = number;
        if turn == 2020 { part1 = number as u64; }
    }
    let part2 = last_number as u64;
    (part1,part2)
}