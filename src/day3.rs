pub fn day3(input_lines: &[String]) -> (u64, u64) {
    let forest = Forest::new(input_lines);
    let part1 = forest.traverse((3, 1));
    let part2: u64 = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter().map(|delta| forest.traverse(*delta)).product();
    (part1, part2)
}

struct Forest {
    tree_positions: Vec<bool>,
    pattern_width: usize,
    pattern_height: usize,
}

impl Forest {
    fn new(input_lines: &[String]) -> Self {
        let mut tree_positions = Vec::new();
        let pattern_width = input_lines[0].len();
        let pattern_height = input_lines.len();
        for line in input_lines {
            for c in line.chars() {
                match c {
                    '#' => tree_positions.push(true),
                    '.' => tree_positions.push(false),
                    _ => panic!("Invalid input character")
                };
            }
        }
        Forest { tree_positions, pattern_width, pattern_height }
    }

    fn traverse(&self, delta: (usize, usize)) -> u64 {
        let (delta_x, delta_y) = delta;
        let mut tree_count: u64 = 0;
        let mut position_x: usize = 0;
        let mut position_y: usize = 0;
        while position_y < self.pattern_height {
            if self.tree_positions[(position_y * self.pattern_width) + (position_x % self.pattern_width)] { tree_count += 1; }
            position_x += delta_x;
            position_y += delta_y;
        }
        tree_count
    }
}