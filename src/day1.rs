use itertools::Itertools;

static TARGET: u64 = 2020;

// It's not a remotely performant solution, but it's pretty.

pub fn day1(input_lines: &[String]) -> (u64, u64) {
    let expenses: Vec<u64> = input_lines.iter().map(|line| line.parse::<u64>().expect("Failed to parse input")).collect();
    let part1 = find_product_of_expenses_that_sum_to(&expenses, 2, TARGET).expect("Failed to solve part 1");
    let part2 = find_product_of_expenses_that_sum_to(&expenses, 3, TARGET).expect("Failed to solve part 2");
    (part1, part2) 
}

fn find_product_of_expenses_that_sum_to(expenses: &[u64], subset_size: usize, target: u64) -> Option<u64> {
    expenses
        .iter()
        .combinations(subset_size)
        .find(|combo| combo.iter().copied().sum::<u64>() == target)
        .and_then(|combo| Some(combo.iter().copied().product()))
}