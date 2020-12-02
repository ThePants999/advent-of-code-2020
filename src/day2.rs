struct PasswordAndPolicy<'a> {
    // The first and second numbers in the policy.
    num_one: usize,
    num_two: usize,

    required_char: char,
    password: &'a str
}

impl<'a> PasswordAndPolicy<'a> {
    fn new(input_line: &'a str) -> Self {
        // Yeah, this could be regex, but speeeeeeeeeed
        let dash_pos = input_line.find('-').expect("Invalid password policy: no dash");
        let num_one = input_line[0..dash_pos].parse::<usize>().expect("Invalid password policy: minimum count not integer");
        let input_line = &input_line[dash_pos + 1..];
        let space_pos = input_line.find(' ').expect("Invalid password policy: no space");
        let num_two = input_line[0..space_pos].parse::<usize>().expect("Invalid password policy: maximum count not integer");
        let input_line = &input_line[space_pos + 1..];
        let required_char = input_line.chars().next().expect("Invalid password policy: required character missing");
        // Skip over the required character, the colon and the space
        let password = &input_line[3..];
    
        PasswordAndPolicy {
            num_one,
            num_two,
            required_char,
            password
        }        
    }

    fn valid_for_part_one(&self) -> bool {
        // Interpret the two numbers as the minimum and maximum times the required_char may appear.
        let char_count = self.password.matches(self.required_char).count();
        (char_count >= self.num_one) && (char_count <= self.num_two)        
    }

    fn valid_for_part_two(&self) -> bool {
        // Interpret the two numbers as the 1-based indices that the required_char may appear,
        // and require that it appears in precisely one of those positions.
        // Using nth twice on the same iterator avoids needlessly iterating over early characters twice,
        // but requires some arithmetic to figure out the second index.
        let mut chars = self.password.chars();
        let first_char_matches = chars.nth(self.num_one - 1).unwrap_or_default() == self.required_char;
        let second_char_matches = chars.nth(self.num_two - self.num_one - 1).unwrap_or_default() == self.required_char;
        first_char_matches ^ second_char_matches
    }
}

pub fn day2(input_lines: &[String]) -> (u64, u64) {
    let passwords: Vec<PasswordAndPolicy> = input_lines.iter().map(|line| PasswordAndPolicy::new(line)).collect();
    let part1 = passwords.iter().filter(|password| password.valid_for_part_one()).count() as u64;
    let part2 = passwords.iter().filter(|password| password.valid_for_part_two()).count() as u64;
    (part1, part2)
}