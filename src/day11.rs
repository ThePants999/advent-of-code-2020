use std::fmt::Display;
use itertools::Itertools;
use strum::IntoEnumIterator;

const ROWS: usize = 97;
const COLS: usize = 98;

#[derive(Clone,Copy,PartialEq,Eq)]
enum States {
    Floor,
    Empty,
    Occupied,
}

impl Display for States {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Floor => '.',
            Self::Empty => 'L',
            Self::Occupied => '#',
        })
    }
}

#[derive(Clone,Copy,PartialEq,Eq)]
enum Rulesets {
    Part1,
    Part2,
}

#[derive(Clone,Copy,EnumIter)]
enum Directions {
    UpLeft,
    Up,
    UpRight,
    Left,
    Right,
    DownLeft,
    Down,
    DownRight,
}

#[derive(Clone,Copy)]
struct Coordinate {
    row: usize,
    col: usize,
}

impl std::ops::Add<Directions> for Coordinate {
    type Output = Option<Coordinate>;

    fn add(self, other: Directions) -> Option<Coordinate> {
        match other {
            Directions::UpLeft if self.row == 0 => None,
            Directions::UpLeft if self.col == 0 => None,
            Directions::UpLeft => Some(Self { row: self.row - 1, col: self.col - 1 }),
            Directions::Up if self.row == 0 => None,
            Directions::Up => Some(Self { row: self.row - 1, col: self.col }),
            Directions::UpRight if self.row == 0 => None,
            Directions::UpRight if self.col == COLS-1 => None,
            Directions::UpRight => Some(Self { row: self.row - 1, col: self.col + 1 }),
            Directions::Left if self.col == 0 => None,
            Directions::Left => Some(Self { row: self.row, col: self.col - 1 }),
            Directions::Right if self.col == COLS-1 => None,
            Directions::Right => Some(Self { row: self.row, col: self.col + 1 }),
            Directions::DownLeft if self.row == ROWS-1 => None,
            Directions::DownLeft if self.col == 0 => None,
            Directions::DownLeft => Some(Self { row: self.row + 1, col: self.col - 1 }),
            Directions::Down if self.row == ROWS-1 => None,
            Directions::Down => Some(Self { row: self.row + 1, col: self.col }),
            Directions::DownRight if self.row == ROWS-1 => None,
            Directions::DownRight if self.col == COLS-1 => None,
            Directions::DownRight => Some(Self { row: self.row + 1, col: self.col + 1 }),
        }
    }
}

type Grid = [[States; COLS]; ROWS];

pub fn day11(input_lines: &[String]) -> (u64, u64) {
    let starting_grid = parse_input(input_lines);
    let part1 = run_until_stable(&starting_grid, Rulesets::Part1).iter().flatten().filter(|&&seat| seat == States::Occupied).count() as u64;
    let part2 = run_until_stable(&starting_grid, Rulesets::Part2).iter().flatten().filter(|&&seat| seat == States::Occupied).count() as u64;
    (part1,part2)
}

fn parse_input(input_lines: &[String]) -> Grid {
    let mut grid: Grid = [[States::Empty; COLS]; ROWS];
    for (row, line) in input_lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            grid[row][col] = match c {
                'L' => States::Empty,
                '.' => States::Floor,
                _ => unreachable!("Invalid input"),
            }
        }
    }
    grid
}

fn run_until_stable(starting_grid: &Grid, ruleset: Rulesets) -> Grid {
    let mut grid = *starting_grid;
    loop {
        let new_grid = apply_round(&grid, ruleset);
        if new_grid == grid {
            break;
        }
        grid = new_grid;
    }
    grid
}

fn apply_round(grid: &Grid, ruleset: Rulesets) -> Grid {
    let mut new_grid: Grid = [[States::Empty; COLS]; ROWS];
    for (row, col) in (0..ROWS).cartesian_product(0..COLS) {
        new_grid[row][col] = match grid[row][col] {
            States::Floor => States::Floor,
            States::Empty => if count_adjacent_occupied_seats(&grid, row, col, ruleset) == 0 { States::Occupied } else { States::Empty },
            States::Occupied if ruleset == Rulesets::Part1 => if count_adjacent_occupied_seats(grid, row, col, ruleset) >= 4 { States::Empty } else { States::Occupied },
            States::Occupied => if count_adjacent_occupied_seats(grid, row, col, ruleset) >= 5 { States::Empty } else { States::Occupied },
        }
    }
    new_grid
}

fn count_adjacent_occupied_seats(grid: &Grid, row: usize, col: usize, ruleset: Rulesets) -> u8 {
    Directions::iter().map(|dir| can_see_occupied_seat(grid, Coordinate { row, col }, dir, ruleset)).sum()
    // let mut occupied_seats = 0u8;
    // if row > 0 {
    //     if col > 0 && grid[row-1][col-1] == States::Occupied { occupied_seats += 1; }
    //     if grid[row-1][col] == States::Occupied { occupied_seats += 1; }
    //     if col < (COLS - 1) && grid[row-1][col+1] == States::Occupied { occupied_seats += 1; }
    // }
    // if col > 0 && grid[row][col-1] == States::Occupied { occupied_seats += 1; }
    // if col < (COLS - 1) && grid[row][col+1] == States::Occupied { occupied_seats += 1; }
    // if row < (ROWS - 1) {
    //     if col > 0 && grid[row+1][col-1] == States::Occupied { occupied_seats += 1; }
    //     if grid[row+1][col] == States::Occupied { occupied_seats += 1; }
    //     if col < (COLS - 1) && grid[row+1][col+1] == States::Occupied { occupied_seats += 1; }
    // }
    // occupied_seats
}

fn can_see_occupied_seat(grid: &Grid, from: Coordinate, dir: Directions, ruleset: Rulesets) -> u8 {
    let mut can_see_seat = 0u8;
    let mut current_coord = from;
    while let Some(coord) = current_coord + dir {
        current_coord = coord;
        match grid[current_coord.row][current_coord.col] {
            States::Occupied => {
                can_see_seat = 1;
                break;
            },
            States::Empty => break,
            States::Floor if ruleset == Rulesets::Part1 => break,
            States::Floor => (),
        }
    }
    can_see_seat
}

#[allow(dead_code)]
fn display_grid(grid: &Grid) {
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
    println!();
}