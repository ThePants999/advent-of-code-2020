use std::collections::HashSet;

pub fn day24(input_lines: &[String]) -> (u64, u64) {
    let mut exhibit = Exhibit::new();
    exhibit.parse_input(input_lines);
    let part1 = exhibit.black_tiles.len() as u64;
    for _ in 1..=100 {
        exhibit.apply_day_process();
    }
    let part2 = exhibit.black_tiles.len() as u64;
    (part1,part2)
}

struct Exhibit {
    black_tiles: HashSet<Coordinate>,
    min: Coordinate,
    max: Coordinate,
}

impl Exhibit {
    fn new() -> Self {
        Self {
            black_tiles: HashSet::new(),
            min: Coordinate { x: -2, y: -1 },
            max: Coordinate { x: 2, y: 1 },
        }
    }

    fn parse_input(&mut self, input_lines: &[String]) {
        for line in input_lines {
            let directions = Self::parse_directions(line);
            let coords = directions.iter().fold(Coordinate { x: 0, y: 0 }, |current, dir| &current + dir);
            let tile_currently_white = !self.black_tiles.contains(&coords);
            self.set_tile(coords, tile_currently_white);
        }
    }

    fn parse_directions(input: &str) -> Vec<Direction> {
        let mut chars = input.chars();
        let mut directions: Vec<Direction> = Vec::new();
        while let Some(c) = chars.next() {
            let direction = match c {
                'e' => Direction::E,
                's' => match chars.next().expect("Invalid direction in input") {
                    'e' => Direction::SE,
                    'w' => Direction::SW,
                    _ => unreachable!("Invalid direction in input"),
                },
                'w' => Direction::W,
                'n' => match chars.next().expect("Invalid direction in input") {
                    'e' => Direction::NE,
                    'w' => Direction::NW,
                    _ => unreachable!("Invalid direction in input"),
                },
                _ => unreachable!("Invalid direction in input"),
            };
            directions.push(direction);
        }
        directions
    }

    fn set_tile(&mut self, coords: Coordinate, black: bool) {
        if coords.x + 2 > self.max.x { self.max = Coordinate { x: coords.x + 2, y: self.max.y }; }
        if coords.x - 2 < self.min.x { self.min = Coordinate { x: coords.x - 2, y: self.min.y }; }
        if coords.y + 1 > self.max.y { self.max = Coordinate { x: self.max.x, y: coords.y + 1 }; }
        if coords.y - 1 < self.min.y { self.min = Coordinate { x: self.min.x, y: coords.y - 1 }; }
        if black {
            self.black_tiles.insert(coords);
        } else {
            self.black_tiles.remove(&coords);
        }
    }

    fn apply_day_process(&mut self) {
        let new_black_tiles = HashSet::with_capacity(self.black_tiles.len() * 2);
        let current_black_tiles = std::mem::replace(&mut self.black_tiles, new_black_tiles);
        for x in self.min.x..=self.max.x {
            for y in self.min.y..=self.max.y {
                // In hexagonal coordinates, x and y are always either both even or both odd.
                if x.abs() % 2 != y.abs() % 2 { continue; }

                let coords = Coordinate { x, y };
                let adjacent_black_tiles = Self::count_adjacent_black_tiles(&current_black_tiles, &coords);
                if current_black_tiles.contains(&coords) {
                    // Tile is currently black - it stays black if it has 1 or 2 black neighbours.
                    if adjacent_black_tiles == 1 || adjacent_black_tiles == 2 {
                        self.set_tile(coords, true);
                    }
                } else {
                    // Tile is currently white - it flips to black if it has 2 black neighbours.
                    if adjacent_black_tiles == 2 {
                        self.set_tile(coords, true);
                    }
                }
            }
        }
    }

    fn count_adjacent_black_tiles(black_tiles: &HashSet<Coordinate>, coords: &Coordinate) -> usize {
        coords.adjacent_coordinates().filter(|adj_coords| black_tiles.contains(&adj_coords)).count()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Coordinate {
    x: isize,
    y: isize,
}

lazy_static! {
    static ref ALL_DIRECTIONS: Vec<Direction> = vec![Direction::E, Direction::SE, Direction::SW, Direction::W, Direction::NW, Direction::NE];
}

impl Coordinate {
    fn adjacent_coordinates(&self) -> impl Iterator<Item = Self> + '_ {
        ALL_DIRECTIONS.iter().map(move |dir| self + dir)
    }
}

#[derive(Debug)]
enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

impl std::ops::Add<&Direction> for &Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: &Direction) -> Self::Output {
        match rhs {
            Direction::E => Self::Output { x: self.x + 2, y: self.y },
            Direction::SE => Self::Output { x: self.x + 1, y: self.y - 1 },
            Direction::SW => Self::Output { x: self.x - 1, y: self.y - 1 },
            Direction::W => Self::Output { x: self.x - 2, y: self.y },
            Direction::NW => Self::Output { x: self.x - 1, y: self.y + 1 },
            Direction::NE => Self::Output { x: self.x + 1, y: self.y + 1 },
        }
    }
}