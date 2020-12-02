struct PasswordAndPolicy<'a> {
    // The first and second numbers in the policy.
    num_one: usize,
    num_two: usize,

    required_char: char,
    password: &'a str
}

impl<'a> PasswordAndPolicy<'a> {
    fn valid_for_part_one(&self) -> bool {
        // Interpret the two numbers as the minimum and maximum times the required_char may appear.
        let char_count = self.password.matches(self.required_char).count();
        (char_count >= self.num_one) && (char_count <= self.num_two)        
    }

    fn valid_for_part_two(&self) -> bool {
        // Interpret the two numbers as the 1-based indices that the required_char may appear,
        // and require that it appears in precisely one of those positions.
        let first_char_matches = self.password.chars().nth(self.num_one - 1).unwrap_or_default() == self.required_char;
        let second_char_matches = self.password.chars().nth(self.num_two - 1).unwrap_or_default() == self.required_char;
        first_char_matches ^ second_char_matches
    }
}

pub fn day2(input_lines: &[String]) -> (u64, u64) {
    let passwords: Vec<PasswordAndPolicy> = input_lines.iter().map(|line| parse_password(line)).collect();
	let part1 = passwords.iter().filter(|password| password.valid_for_part_one()).count() as u64;
    let part2 = passwords.iter().filter(|password| password.valid_for_part_two()).count() as u64;
    (part1, part2)
}

fn parse_password(line: &str) -> PasswordAndPolicy {
    let dash_pos = line.find('-').expect("Invalid password policy: no dash");
    let num_one = line[0..dash_pos].parse::<usize>().expect("Invalid password policy: minimum count not integer");
    let line = &line[dash_pos + 1..];
    let space_pos = line.find(' ').expect("Invalid password policy: no space");
    let num_two = line[0..space_pos].parse::<usize>().expect("Invalid password policy: maximum count not integer");
    let line = &line[space_pos + 1..];
    let required_char = line.chars().next().expect("Invalid password policy: required character missing");
    // Skip over the required character, the colon and the space
    let password = &line[3..];

    PasswordAndPolicy {
        num_one,
        num_two,
        required_char,
        password
    }
}