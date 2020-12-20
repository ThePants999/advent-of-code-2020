use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

pub fn day19(input_lines: &[String]) -> (u64, u64) {
    let mut map: HashMap<usize, Rc<RefCell<Rule>>> = HashMap::new();
    let mut lines_iter = input_lines.iter();
    while let Some(line) = lines_iter.next() {
        if line.is_empty() {
            break;
        }
        parse_line(&mut map, line);
    }

    let lines_iter_2 = lines_iter.clone();
    let part1 = count_matching_messages(lines_iter, &map.get(&0).unwrap().borrow());
    parse_line(&mut map, "8: 42 | 42 8");
    parse_line(&mut map, "11: 42 31 | 42 11 31");
    let part2 = count_matching_messages(lines_iter_2, &map.get(&0).unwrap().borrow());
    (part1,part2)
}

fn count_matching_messages<'a>(messages: impl Iterator<Item = &'a String>, rule: &Rule) -> u64 {
    messages.filter(|line| rule.matches(line).contains(&line.len())).count() as u64
}

#[derive(PartialEq)]
enum MatchType {
    String(String),
    Patterns,
    Unresolved,
}

type Pattern = Vec<Rc<RefCell<Rule>>>;

struct Rule {
    match_type: MatchType,
    patterns: Vec<Pattern>,
}

impl Rule {
    fn new() -> Self {
        Self {
            match_type: MatchType::Unresolved,
            patterns: Vec::new(),
        }
    }

    fn parse(&mut self, map: &mut HashMap<usize, Rc<RefCell<Rule>>>, tokens: impl Iterator<Item = ParseResult>) {
        let mut pattern = Pattern::new();
        self.match_type = MatchType::Patterns;
        for token in tokens {
            match token {
                ParseResult::String(s) => self.match_type = MatchType::String(s),
                ParseResult::Number(num) => pattern.push(get_rule(map, num)),
                ParseResult::Pipe => { self.patterns.push(pattern); pattern = Pattern::new(); }
            }
        }
        if self.match_type == MatchType::Patterns { self.patterns.push(pattern); }
    }

    fn matches(&self, input: &str) -> HashSet<usize> {
        let mut set: HashSet<usize> = HashSet::new();
        match &self.match_type {
            MatchType::String(s) => if input.starts_with(s) { set.insert(s.len()); },
            MatchType::Patterns => {
                for pattern in &self.patterns {
                    let mut this_pattern_set: HashSet<usize> = HashSet::new();
                    this_pattern_set.insert(0);

                    for subrule_cell in pattern {
                        // Going to be cheeky today and assume ASCII input - proper UTF-8 support can wait for another
                        // time.  I already demonstrated it in day 18.
                        if this_pattern_set.is_empty() {
                            this_pattern_set.insert(0);
                        }
                        let mut replacement_set: HashSet<usize> = HashSet::new();
                        for previous_length in this_pattern_set {
                            let subrule_matches = subrule_cell.borrow().matches(&input[previous_length..]);
                            for length in subrule_matches {
                                replacement_set.insert(previous_length + length);
                            }
                        }
                        this_pattern_set = replacement_set;
                        if this_pattern_set.is_empty() {
                            break;
                        }
                    }

                    set.extend(this_pattern_set);
                }
            },
            MatchType::Unresolved => unreachable!("Rule missing from input"),
        }
        set
    }
}

enum ParseResult {
    Number(usize),
    String(String),
    Pipe,
}

fn parse_line(map: &mut HashMap<usize, Rc<RefCell<Rule>>>, line: &str) {
    let mut tokens = line.split(' ');
    let mut first_token = tokens.next().unwrap().to_string();
    first_token.truncate(first_token.len() - 1);
    let rule_id = first_token.parse::<usize>().expect("Invalid input");
    let rule_cell = get_rule(map, rule_id);
    let mut rule = rule_cell.borrow_mut();
    let tokens = tokens.map(|token| parse_token(token));
    rule.parse(map, tokens);
}

fn parse_token(token: &str) -> ParseResult {
    let mut chars = token.chars();
    match chars.next().unwrap() {
        '"' => {
            let mut string: String = chars.collect();
            string.truncate(string.len() - 1);
            ParseResult::String(string)
        },
        '|' => ParseResult::Pipe,
        c => {
            let mut num = c.to_digit(10).expect("Invalid input");
            for c in chars {
                num *= 10;
                num += c.to_digit(10).expect("Invalid input");
            }
            ParseResult::Number(num as usize)
        }
    }
}

fn get_rule(map: &mut HashMap<usize, Rc<RefCell<Rule>>>, index: usize) -> Rc<RefCell<Rule>> {
    map.entry(index).or_insert_with(|| Rc::new(RefCell::new(Rule::new()))).clone()
}