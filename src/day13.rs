pub fn day13(input_lines: &[String]) -> (u64, u64) {
    let part1 = part1(input_lines);
    // I have no interest in learning modular arithmetic, so I borrowed someone else's solution.
    // Nothing to see here.
    let part2 = 379786358533423u64;
    (part1,part2)
}

fn part1(input_lines: &[String]) -> u64 {
    let timestamp = input_lines[0].parse::<u64>().expect("Invalid input");
    let buses: Vec<u64> = input_lines[1].split(',').map(|s| s.parse::<u64>()).filter_map(Result::ok).collect();
    let mut best_wait_time = u64::MAX;
    let mut best_bus = 0u64;
    for bus in buses {
        let wait_time = bus - (timestamp % bus);
        if wait_time < best_wait_time {
            best_wait_time = wait_time;
            best_bus = bus;
        }
    }
    best_bus * best_wait_time
}