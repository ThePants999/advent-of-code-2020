pub fn day12(input_lines: &[String]) -> (u64, u64) {
    let mut ships: Vec<Box<dyn Ship>> = vec![Box::new(Part1Ship::new()), Box::new(Part2Ship::new())];
    input_lines.iter().map(|line| Instruction::parse(line)).for_each(|instruction| { ships.iter_mut().for_each(|ship| ship.apply_instruction(&instruction)) });
    let part1 = ships[0].manhattan_distance() as u64;
    let part2 = ships[1].manhattan_distance() as u64;
    (part1,part2)
}

#[derive(Clone,Copy)]
enum Turns {
    Left,
    Right,
    Around,
}

#[derive(Clone,Copy)]
enum Directions {
    North,
    South,
    East,
    West,
}

impl std::ops::Add<Turns> for Directions {
    type Output = Directions;

    fn add(self, rhs: Turns) -> Self::Output {
        match self {
            Self::North => match rhs {
                Turns::Left => Self::West,
                Turns::Right => Self::East,
                Turns::Around => Self::South,
            },
            Self::South => match rhs {
                Turns::Left => Self::East,
                Turns::Right => Self::West,
                Turns::Around => Self::North,
            },
            Self::East => match rhs {
                Turns::Left => Self::North,
                Turns::Right => Self::South,
                Turns::Around => Self::West,
            },
            Self::West => match rhs {
                Turns::Left => Self::South,
                Turns::Right => Self::North,
                Turns::Around => Self::East,
            },
        }
    }
}

impl std::ops::AddAssign<Turns> for Directions {
    fn add_assign(&mut self, rhs: Turns) {
        *self = *self + rhs;
    }
}

impl std::ops::Add<&Turns> for Directions {
    type Output = Directions;

    fn add(self, rhs: &Turns) -> Self::Output {
        self + *rhs
    }
}

impl std::ops::AddAssign<&Turns> for Directions {
    fn add_assign(&mut self, rhs: &Turns) {
        *self = *self + rhs;
    }
}

struct Movement {
    direction: Directions,
    distance: i64,
}

#[derive(Clone,Copy)]
struct Position {
    north_south: i64,
    east_west: i64,
}

impl Position {
    fn apply_movement(&mut self, movement: &Movement) {
        match movement.direction {
            Directions::North => self.north_south -= movement.distance,
            Directions::South => self.north_south += movement.distance,
            Directions::East => self.east_west += movement.distance,
            Directions::West => self.east_west -= movement.distance,
        }
    }

    fn rotate(&mut self, rotation: &Turns) {
        match rotation {
            Turns::Around => *self *= -1,
            Turns::Left => *self = Self {
                north_south: -self.east_west,
                east_west: self.north_south,
            },
            Turns::Right => *self = Self {
                north_south: self.east_west,
                east_west: -self.north_south,
            },
        }
    }

    fn manhattan_distance(&self) -> i64 {
        self.north_south.abs() + self.east_west.abs()
    }
}

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            north_south: self.north_south + rhs.north_south,
            east_west: self.east_west + rhs.east_west,
        }
    }
}

impl std::ops::AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Mul<i64> for Position {
    type Output = Position;

    fn mul(self, rhs: i64) -> Self::Output {
        Self::Output {
            north_south: self.north_south * rhs,
            east_west: self.east_west * rhs,
        }
    }
}

impl std::ops::MulAssign<i64> for Position {
    fn mul_assign(&mut self, rhs: i64) {
        *self = *self * rhs;
    }
}

impl std::ops::Mul<&i64> for Position {
    type Output = Position;

    fn mul(self, rhs: &i64) -> Self::Output {
        self * *rhs
    }
}

enum Instruction {
    Move(Movement),
    Forwards(i64),
    Turn(Turns),
}

impl Instruction {
    fn parse (input: &str) -> Self {
        let amount = input[1..].parse::<i64>().expect("Invalid input");
        match input.chars().next().expect("Invalid input") {
            'N' => Self::Move(Movement { direction: Directions::North, distance: amount }),
            'S' => Self::Move(Movement { direction: Directions::South, distance: amount }),
            'E' => Self::Move(Movement { direction: Directions::East, distance: amount }),
            'W' => Self::Move(Movement { direction: Directions::West, distance: amount }),
            'F' => Self::Forwards(amount),
            'L' if amount == 90 => Self::Turn(Turns::Left),
            'L' if amount == 270 => Self::Turn(Turns::Right),
            'R' if amount == 90 => Self::Turn(Turns::Right),
            'R' if amount == 270 => Self::Turn(Turns::Left),
            _ => Self::Turn(Turns::Around),
        }
    }
}

trait Ship {
    fn apply_instruction(&mut self, instruction: &Instruction);
    fn manhattan_distance(&self) -> i64;
}

struct Part1Ship {
    position: Position,
    heading: Directions,
}

impl Part1Ship {
    fn new() -> Self {
        Self {
            position: Position {
                east_west: 0,
                north_south: 0,
            },
            heading: Directions::East,
        }
    }
}

impl Ship for Part1Ship {
    fn apply_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Move(movement) => self.position.apply_movement(movement),
            Instruction::Forwards(distance) => self.position.apply_movement(&Movement { direction: self.heading, distance: *distance }),
            Instruction::Turn(turn) => self.heading += turn,
        }
    }

    fn manhattan_distance(&self) -> i64 {
        self.position.manhattan_distance()
    }
}

struct Part2Ship {
    position: Position,
    waypoint: Position,
}

impl Part2Ship {
    fn new() -> Self {
        Self {
            position: Position {
                east_west: 0,
                north_south: 0,
            },
            waypoint: Position {
                east_west: 10,
                north_south: -1,
            }
        }
    }
}

impl Ship for Part2Ship {
    fn apply_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Move(movement) => self.waypoint.apply_movement(movement),
            Instruction::Forwards(distance) => self.position += self.waypoint * distance,
            Instruction::Turn(turn) => self.waypoint.rotate(turn),
        }
    }

    fn manhattan_distance(&self) -> i64 {
        self.position.manhattan_distance()
    }
}