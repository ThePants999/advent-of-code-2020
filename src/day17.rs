use std::collections::HashSet;

pub fn day17(input_lines: &[String]) -> (u64, u64) {
    let mut dimension3 = PocketDimension::new(3);
    let mut dimension4 = PocketDimension::new(4);
    dimension3.parse_input(input_lines);
    dimension4.parse_input(input_lines);
    for _ in 0..6 {
        dimension3.perform_cycle();
        dimension4.perform_cycle();
    }
    let part1 = dimension3.active_cubes() as u64;
    let part2 = dimension4.active_cubes() as u64;
    (part1,part2)
}

type Position = Vec<isize>;

struct PocketDimension {
    num_dimensions: usize,
    active_cubes: HashSet<Position>,
    current_iteration_changes: Vec<(Position, bool)>,
    minima: Vec<isize>,
    maxima: Vec<isize>,
}

impl PocketDimension {
    fn new(num_dimensions: usize,) -> Self {
        Self {
            num_dimensions,
            active_cubes: HashSet::new(),
            current_iteration_changes: Vec::new(),
            minima: vec![0; num_dimensions],
            maxima: vec![0; num_dimensions],
        }
    }

    fn parse_input(&mut self, input_lines: &[String]) {
        let mut y = 0isize;
        for line in input_lines {
            for (x, c) in line.chars().enumerate() {
                let mut pos: Position = Vec::new();
                pos.push(x as isize);
                pos.push(y);
                pos.resize(self.num_dimensions, 0);
                self.apply_change(pos, c == '#');
            }
            y -= 1;
        }
    }

    fn perform_cycle(&mut self) {
        assert!(self.current_iteration_changes.is_empty());
        self.recursive_perform_cycle(Vec::new());
        for (pos, active) in std::mem::take(&mut self.current_iteration_changes) {
            self.apply_change(pos, active);
        }
    }

    fn recursive_perform_cycle(&mut self, coords_so_far: Vec<isize>) {
        if coords_so_far.len() == self.num_dimensions {
            // We've got a final set of co-ordinates, check for a change
            let active_neighbours = self.num_active_neighbours(&coords_so_far);
            if self.active_cubes.contains(&coords_so_far) {
                // Cube is currently active
                if active_neighbours < 2 || active_neighbours > 3 {
                    self.current_iteration_changes.push((coords_so_far, false));
                }
            } else {
                // Cube is currently inactive
                if active_neighbours == 3 {
                    self.current_iteration_changes.push((coords_so_far, true));
                }
            }
        } else {
            // Recurse through valid values for next dimension
            let dimension = coords_so_far.len();
            for coord in self.minima[dimension]-1..=self.maxima[dimension]+1 {
                let mut coords = coords_so_far.clone();
                coords.push(coord);
                self.recursive_perform_cycle(coords);
            }
        }
    }

    fn num_active_neighbours(&self, pos: &[isize]) -> usize {
        let mut active_neighbours = self.recursive_check_cube(&pos, 0);
        if self.active_cubes.contains(pos) {
            // We included ourselves in the recursive check - discount ourselves.
            active_neighbours -= 1;
        }
        active_neighbours
    }

    fn recursive_check_cube(&self, pos: &[isize], tweak_dimension: usize) -> usize {
        if tweak_dimension == self.num_dimensions {
            if self.active_cubes.contains(pos) { 1 } else { 0 }
        } else {
            // Recurse via another dimension
            let mut active_neighbours = 0usize;
            let mut new_pos = pos.to_owned();
            active_neighbours += self.recursive_check_cube(&new_pos, tweak_dimension + 1);
            new_pos[tweak_dimension] -= 1;
            active_neighbours += self.recursive_check_cube(&new_pos, tweak_dimension + 1);
            new_pos[tweak_dimension] += 2;
            active_neighbours += self.recursive_check_cube(&new_pos, tweak_dimension + 1);
            active_neighbours
        }
    }

    fn apply_change(&mut self, pos: Position, active: bool) {
        if active {
            for (dim, pos_in_dim) in pos.iter().enumerate() {
                if *pos_in_dim < self.minima[dim] { self.minima[dim] = *pos_in_dim; }
                if *pos_in_dim > self.maxima[dim] { self.maxima[dim] = *pos_in_dim; }
            }
            self.active_cubes.insert(pos);
        } else {
            self.active_cubes.remove(&pos);
        }
    }

    fn active_cubes(&self) -> usize {
        self.active_cubes.len()
    }
}