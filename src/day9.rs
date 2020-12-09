const PREAMBLE_SIZE: usize = 25;

// This is an "IDGAF" day, cos it's about finding the right algorithm not about coding and I'm not interested,
// just want to get past it.

pub fn day9(input_lines: &[String]) -> (u64, u64) {
    let numbers: Vec<u64> = input_lines.iter().map(|number| number.parse().expect("Non-numeric input!")).collect();
    let mut part1 = 0u64;
    let mut part2 = 0u64;
    for pos in PREAMBLE_SIZE..numbers.len() {
        let target = numbers[pos];
        let range = &numbers[pos - PREAMBLE_SIZE..pos];
        // Run through the PREAMBLE_SIZE prior numbers considering whether they're one of a pair that add up to the target.
        // Skip any that are larger than or equal to the target, they clearly can't be one.
        // Also skip any that are precisely half the target, since you can't use a number twice.
        if !range.iter().any(|&num| (target > num) && (num * 2 != target) && range.contains(&(target - num))) {
            part1 = numbers[pos];
            break;
        }
    }
    'outer: for range_size in 2..numbers.len() {
        for first_num in 0..=numbers.len()-range_size {
            let range = &numbers[first_num..first_num+range_size];
            if range.iter().sum::<u64>() == part1 {
                part2 = range.iter().min().unwrap() + range.iter().max().unwrap();
                break 'outer;
            }
        }
    }
    (part1,part2)
}