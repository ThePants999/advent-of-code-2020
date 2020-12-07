use regex::Regex;
use std::collections::{HashSet, HashMap};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub fn day7(input_lines: &[String]) -> (u64, u64) {
    let bags = parse_input(input_lines);
    let bag = bags.get("shiny gold").expect("No shiny gold bag in input!");
    let mut set: HashSet<&str> = HashSet::new();
    add_bags_containing_bag(&mut set, &bag);
    let part1 = set.len() as u64;
    let part2 = count_bag_contents(&bag);
    (part1,part2)
}

fn add_bags_containing_bag<'a>(set: &mut HashSet<&'a str>, bag_cell: &RefCell<Bag<'a>>) {
    let bag = bag_cell.borrow();
    for containing_bag_ptr in &bag.contained_in {
        let containing_bag = containing_bag_ptr.upgrade().unwrap();
        if !set.contains(containing_bag.borrow().colour) {
            set.insert(containing_bag.borrow().colour);
            add_bags_containing_bag(set, &containing_bag);
        }
    }
}

fn count_bag_contents(bag_cell: &RefCell<Bag>) -> u64 {
    let mut count = 0u64;
    let bag = bag_cell.borrow();
    for contents in &bag.contents {
        let contained_bag = contents.bag.upgrade().unwrap();
        // If A contains X Bs, and B contains Y total bags, A contains
        // X(Y+1) total bags - count_bag_contents(B) only returns Y.
        count += (count_bag_contents(&contained_bag) + 1) * contents.count;
    }
    count
}

fn parse_input(input_lines: &[String]) -> HashMap<&str, Rc<RefCell<Bag>>> {
    let mut bags: HashMap<&str, Rc<RefCell<Bag>>> = HashMap::new();

    for line in input_lines {
        lazy_static! {
            static ref OUTER_RE: Regex = Regex::new(r"^(\w+ \w+) bag").unwrap();
            static ref CONTENTS_RE: Regex = Regex::new(r"([0-9]) (\w+ \w+) bag").unwrap();
        }
        let containing_colour = OUTER_RE.captures_iter(line).next().unwrap().get(1).expect("Invalid input!").as_str();
        let containing_bag = get_bag(containing_colour, &mut bags);
        for cap in CONTENTS_RE.captures_iter(line) {
            let count: u64 = cap.get(1).expect("Invalid input!").as_str().parse().unwrap();
            let colour = cap.get(2).expect("Invalid input!").as_str();
            let bag = get_bag(colour, &mut bags);
            containing_bag.borrow_mut().add_contents(&bag, count);
            bag.borrow_mut().add_contained_in(&containing_bag);
        }    
    }

    bags
}

struct BagContents<'a> {
    bag: Weak<RefCell<Bag<'a>>>,
    count: u64,
}

struct Bag<'a> {
    colour: &'a str,
    contained_in: Vec<Weak<RefCell<Bag<'a>>>>,
    contents: Vec<BagContents<'a>>,
}

impl<'a> Bag<'a> {
    fn new(colour: &'a str) -> Self {
        Self {
            colour,
            contained_in: Vec::new(),
            contents: Vec::new(),
        }
    }

    fn add_contained_in(&mut self, bag: &Rc<RefCell<Bag<'a>>>) {
        self.contained_in.push(Rc::downgrade(bag));
    }

    fn add_contents(&mut self, bag: &Rc<RefCell<Bag<'a>>>, count: u64) {
        self.contents.push(BagContents { bag: Rc::downgrade(bag), count });
    }
}

fn get_bag<'a>(colour: &'a str, map: &mut HashMap<&'a str, Rc<RefCell<Bag<'a>>>>) -> Rc<RefCell<Bag<'a>>> {
    map.entry(colour).or_insert_with(|| Rc::new(RefCell::new(Bag::new(colour)))).clone()
}
