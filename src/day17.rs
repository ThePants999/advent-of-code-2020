use std::{collections::HashSet, fmt::Display};

pub fn day17(input_lines: &[String]) -> (u64, u64) {
    let mut dimension = Dimension::new();
    dimension.parse_input(input_lines);
    for _ in 0..6 { dimension.perform_cycle(); }
    let part1 = dimension.active_cubes() as u64;
    (part1,0)
}

#[derive(Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

struct Dimension {
    active_cubes: HashSet<Position>,
    current_iteration_changes: Vec<(Position, bool)>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    min_z: isize,
    max_z: isize,
}

impl Dimension {
    fn new() -> Self {
        Self {
            active_cubes: HashSet::new(),
            current_iteration_changes: Vec::new(),
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            min_z: 0,
            max_z: 0,
        }
    }

    fn parse_input(&mut self, input_lines: &[String]) {
        let mut y = 0isize;
        for line in input_lines {
            for (x, c) in line.chars().enumerate() {
                self.apply_change(Position { x: x as isize, y, z: 0 }, c == '#');
            }
            y -= 1;
        }
    }

    fn perform_cycle(&mut self) {
        assert!(self.current_iteration_changes.is_empty());
        for x in self.min_x-1..=self.max_x+1 {
            for y in self.min_y-1..=self.max_y+1 {
                for z in self.min_z-1..=self.max_z+1 {
                    let pos = Position { x, y, z };
                    let active_neighbours = self.num_active_neighbours(pos);
                    if self.active_cubes.contains(&pos) {
                        // Cube is currently active
                        if active_neighbours < 2 || active_neighbours > 3 {
                            self.current_iteration_changes.push((pos, false));
                        }
                    } else {
                        // Cube is currently inactive
                        if active_neighbours == 3 {
                            self.current_iteration_changes.push((pos, true));
                        }
                    }
                }
            }
        }
        let changes = self.current_iteration_changes.clone();
        for (pos, active) in changes {
            self.apply_change(pos, active);
        }
        self.current_iteration_changes.clear();
    }

    fn num_active_neighbours(&self, pos: Position) -> usize {
        let mut active_neighbours = 0usize;
        for x in pos.x-1..=pos.x+1 {
            for y in pos.y-1..=pos.y+1 {
                for z in pos.z-1..=pos.z+1 {
                    if x == pos.x && y == pos.y && z == pos.z { continue; }
                    if self.active_cubes.contains(&Position { x, y, z }) { active_neighbours += 1; }
                }
            }
        }
        active_neighbours
    }

    fn apply_change(&mut self, pos: Position, active: bool) {
        if active {
            if pos.x < self.min_x { self.min_x = pos.x; }
            if pos.x > self.max_x { self.max_x = pos.x; }
            if pos.y < self.min_y { self.min_y = pos.y; }
            if pos.y > self.max_y { self.max_y = pos.y; }
            if pos.z < self.min_z { self.min_z = pos.z; }
            if pos.z > self.max_z { self.max_z = pos.z; }
            self.active_cubes.insert(pos);
        } else {
            self.active_cubes.remove(&pos);
        }
    }

    fn active_cubes(&self) -> usize {
        self.active_cubes.len()
    }
}

impl Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for z in self.min_z..=self.max_z {
            output.push_str(format!("\nz={}\n", z).as_str());
            for y in (self.min_y..=self.max_y).rev() {
                for x in self.min_x..=self.max_x {
                    if self.active_cubes.contains(&Position { x, y, z }) {
                        output.push('#');
                    } else {
                        output.push('.');
                    }
                }
                output.push('\n');
            }
        }
        write!(f, "{}", output)
    }
}