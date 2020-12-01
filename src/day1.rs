use itertools::Itertools;

static TARGET: u64 = 2020;

pub fn day1(input_lines: &[String]) -> (u64, u64) {
    let expenses: Vec<u64> = input_lines.iter().map(|line| line.parse::<u64>().expect("Failed to parse input")).collect();
    let part1 = multiply_expenses(&find_n_expenses_that_sum_to(&expenses, 2, TARGET).expect("Failed to solve part 1"));
    let part2 = multiply_expenses(&find_n_expenses_that_sum_to(&expenses, 3, TARGET).expect("Failed to solve part 2"));
    (part1, part2) 
}

fn find_n_expenses_that_sum_to(expenses: &[u64], n: usize, target: u64) -> Option<Vec<&u64>> {
    expenses.iter().combinations(n).find(|combo| combo.iter().copied().sum::<u64>() == target)
}

fn multiply_expenses(expenses: &[&u64]) -> u64 {
    expenses.iter().fold(1, |acc, x| acc * **x)
}
