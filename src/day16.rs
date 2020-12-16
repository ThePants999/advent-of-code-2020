use std::collections::HashSet;
use regex::Regex;

const NUM_FIELDS: usize = 20;

// Don't even look at this, it's awful, I rushed it cos I had the opportunity to
// be the first in my group to get today's star ;-)

pub fn day16(input_lines: &[String]) -> (u64, u64) {
    for index in 0..input_lines.len() {
        if input_lines[index].is_empty() {
            let mut fields = parse_fields(&input_lines[0..index]);
            let my_ticket: Vec<u64> = input_lines[index + 2].split(',').map(|num| num.parse::<u64>().expect("Invalid input")).collect();
            let nearby_tickets: Vec<Vec<u64>> = input_lines[(index + 5)..].iter().map(|line| {
                line.split(',').map(|num| num.parse::<u64>().expect("Invalid input")).collect()
            }).collect();

            let mut invalid_values: Vec<u64> = Vec::new();
            let mut valid_tickets: Vec<Vec<u64>> = Vec::new();
            for ticket in &nearby_tickets {
                let mut ticket_valid = true;
                for value in ticket {
                    let mut field_valid = false;
                    for field in &fields {
                        if (*value >= field.min1 && *value <= field.max1) || (*value >= field.min2 && *value <= field.max2) {
                            field_valid = true;
                            break;
                        }
                    }
                    if !field_valid {
                        invalid_values.push(*value);
                        ticket_valid = false;
                    }
                }
                if ticket_valid { valid_tickets.push(ticket.clone()); }
            }

            let part1 = invalid_values.iter().sum();

            for ticket in valid_tickets {
#[allow(clippy::needless_range_loop)]                
                for position in 0..NUM_FIELDS {
                    for field in fields.iter_mut() {
                        let val = ticket[position];
                        if val < field.min1 || (val > field.max1 && val < field.min2) || val > field.max2 {
                            field.eliminate(position)
                        }
                    }
                }
            }

            loop {
                let mut all_fields_solved = true;
                let mut pos_to_eliminate: Vec<usize> = Vec::new();
                for field in &fields {
                    if let Some(position) = field.position {
                        pos_to_eliminate.push(position);
                    } else {
                        all_fields_solved = false;
                    }
                }
                for field in fields.iter_mut() {
                    for pos in &pos_to_eliminate {
                        field.eliminate(*pos);
                    }
                }
                if all_fields_solved { break; }
            }

            let mut departure_fields: Vec<u64> = Vec::new();
            for field in fields.iter_mut() {
                if field.name.starts_with("departure") {
                    departure_fields.push(my_ticket[field.position.unwrap()]);
                }
            }
            let part2 = departure_fields.iter().product();

            return (part1, part2);
        }
    }
    (0,0)
}

fn parse_fields(input_lines: &[String]) -> Vec<Field> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([^:]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)$").unwrap();
    }
    let mut fields: Vec<Field> = Vec::new();
    for line in input_lines {
        if let Some(caps) = RE.captures(line) {
            let name = caps.get(1).unwrap().as_str().to_string();
            let min1 = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
            let max1 = caps.get(3).unwrap().as_str().parse::<u64>().unwrap();
            let min2 = caps.get(4).unwrap().as_str().parse::<u64>().unwrap();
            let max2 = caps.get(5).unwrap().as_str().parse::<u64>().unwrap();
            fields.push(Field::new(name, min1, max1, min2, max2));
        }
    }

    fields
}

#[derive(PartialEq,Eq)]
struct Field {
    name: String,
    min1: u64,
    max1: u64,
    min2: u64,
    max2: u64,
    potentially_valid_positions: HashSet<usize>,
    position: Option<usize>,
}

impl Field {
    fn new(name: String, min1: u64, max1: u64, min2: u64, max2: u64) -> Self {
        let mut set: HashSet<usize> = HashSet::with_capacity(NUM_FIELDS);
        for position in 0..NUM_FIELDS {
            set.insert(position);
        }
        Self { name, min1, max1, min2, max2, potentially_valid_positions: set, position: None }
    }

    fn eliminate(&mut self, position: usize) {
        self.potentially_valid_positions.remove(&position);
        if self.potentially_valid_positions.len() == 1 {
            self.position = Some(*self.potentially_valid_positions.iter().next().unwrap());
        }
    }
}