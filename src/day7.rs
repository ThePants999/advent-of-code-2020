use regex::Regex;
use std::collections::{HashSet, HashMap};

pub fn day7(input_lines: &[String]) -> (u64, u64) {
    let bags = parse_input(input_lines);
    let mut set: HashSet<String> = HashSet::new();
    let bag = bags.get("shiny gold").unwrap();
    traverse_contained_in_tree(bag, &bags, &mut set);
    let part1 = set.len() as u64 - 1; // Set contains shiny gold itself
    let part2 = traverse_contents_tree(bag, &bags) - 1; // Again, deduct the shiny gold bag itself
    (part1,part2)
}

fn traverse_contained_in_tree(bag: &Bag, all_bags: &HashMap<String, Bag>, set: &mut HashSet<String>) {
    set.insert(bag.colour.clone());
    for contained_in in &bag.contained_in {
        if !set.contains(contained_in) {
            traverse_contained_in_tree(all_bags.get(contained_in).unwrap(), all_bags, set);
        }
    }
}

fn traverse_contents_tree(bag: &Bag, all_bags: &HashMap<String, Bag>) -> u64 {
    let mut count = 1u64; // Include this bag itself
    for contents in &bag.contains {
        let contained_bag = all_bags.get(&contents.colour).unwrap();
        count += traverse_contents_tree(contained_bag, all_bags) * contents.count;
    }
    count
}

fn parse_input(input_lines: &[String]) -> HashMap<String, Bag> {
    let mut bags: HashMap<String, Bag> = HashMap::new();

    for line in input_lines {
        lazy_static! {
            static ref OUTER_RE: Regex = Regex::new(r"^(\w+ \w+) bag").unwrap();
            static ref CONTENTS_RE: Regex = Regex::new(r"([0-9]) (\w+ \w+) bag").unwrap();
        }
        let containing_colour = OUTER_RE.captures_iter(line).next().unwrap().get(1).map_or("".to_string(), |m| m.as_str().to_string());
        // Loop twice because getting a mutable reference to `containing_bag` mutably borrows `bags` for as
        // long as we hold that reference, preventing us from later getting the contents.  There's undoubtedly
        // a neater fix but CBA to care right now.
        {
            let containing_bag = bags.entry(containing_colour.clone()).or_insert_with(|| Bag::new(containing_colour.clone()));
            for cap in CONTENTS_RE.captures_iter(line) {
                let count: u64 = cap.get(1).map_or("", |m| m.as_str()).parse().unwrap();
                let colour = cap.get(2).map_or("".to_string(), |m| m.as_str().to_string());
                containing_bag.contains(colour, count);
            }    
        }
        for cap in CONTENTS_RE.captures_iter(line) {
            let colour = cap.get(2).map_or("".to_string(), |m| m.as_str().to_string());
            let bag = get_bag(colour, &mut bags);
            bag.is_contained_in(containing_colour.clone());
        }    
    }

    bags
}

fn get_bag<'a>(colour: String, map: &'a mut HashMap<String, Bag>) -> &'a mut Bag {
    map.entry(colour.clone()).or_insert_with(|| Bag::new(colour))
}

struct BagContent {
    colour: String,
    count: u64,
}

struct Bag {
    colour: String,
    contained_in: HashSet<String>,
    contains: Vec<BagContent>,
}

impl Bag {
    fn new(colour: String) -> Self {
        Self {
            colour,
            contained_in: HashSet::new(),
            contains: Vec::new(),
        }
    }

    fn is_contained_in(&mut self, colour: String) {
        self.contained_in.insert(colour);
    }

    fn contains(&mut self, colour: String, count: u64) {
        self.contains.push(BagContent { colour, count });
    }
}