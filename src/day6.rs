use std::collections::HashSet;
use crate::utils;

pub fn day6(input_lines: &[String]) -> (u64, u64) {
    let groups = utils::group_lines_split_by_empty_line(input_lines);
    let (unions, intersections): (Vec<HashSet<char>>, Vec<HashSet<char>>) = groups.iter().map(|group| group_responses(group)).unzip();
    let part1 = unions.iter().map(|set| set.len()).sum::<usize>() as u64;
    let part2 = intersections.iter().map(|set| set.len()).sum::<usize>() as u64;
    (part1,part2)
}

fn group_responses(group: &[String]) -> (HashSet<char>, HashSet<char>) {
    let mut union: HashSet<char> = HashSet::new();
    let mut intersection: HashSet<char> = HashSet::new();
    union.extend(group[0].chars());
    intersection.extend(group[0].chars());
    for response in group[1..].iter() {
        let mut new_set: HashSet<char> = HashSet::new();
        for c in response.chars() {
            union.insert(c);
            new_set.insert(c);
        }
        intersection.retain(|c| new_set.contains(c));
    }
    (union, intersection)
}