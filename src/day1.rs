static TARGET: u64 = 2020;

pub fn day1(input_lines: Vec<String>) -> (u64, u64) {
    let expenses: Vec<u64> = input_lines.iter().map(|line| line.parse::<u64>().expect("Failed to parse input")).collect();
    let (expense_one, expense_two) = find_two_expenses_that_sum_to(&expenses, TARGET);
    let part1 = expense_one * expense_two;
    let (expense_one, expense_two, expense_three) = find_three_expenses_that_sum_to(&expenses, TARGET);
    let part2 = expense_one * expense_two * expense_three;
    (part1, part2)
}

fn find_two_expenses_that_sum_to(expenses: &Vec<u64>, target: u64) -> (u64, u64) {
    for i in 0..expenses.len() {
        for j in 0..expenses.len() {
            if i == j { continue; }
            if expenses[i] + expenses[j] == target {
                return (expenses[i], expenses[j])
            }
        }
    }
    panic!("Couldn't solve part 1");
}

fn find_three_expenses_that_sum_to(expenses: &Vec<u64>, target: u64) -> (u64, u64, u64) {
    for i in 0..expenses.len() {
        for j in 0..expenses.len() {
            if i == j { continue; }
            for k in 0..expenses.len() {
                if i == k { continue; }
                if j == k { continue; }
                if expenses[i] + expenses[j] + expenses[k] == target {
                    return (expenses[i], expenses[j], expenses[k])
                }
            }
        }
    }
    panic!("Couldn't solve part 2");
}