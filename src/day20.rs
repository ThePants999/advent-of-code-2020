use std::collections::HashMap;

const TILE_SIZE: usize = 10;

pub fn day20(input_lines: &[String]) -> (u64, u64) {
    let image = Image::construct(input_lines);
    let corners = image.find_corners();
    println!("{:?}", corners);
    let part1: u64 = image.find_corners().iter().product();
    (part1,0)
}

struct Image {
    tiles: HashMap<u64, Tile>,
}

impl Image {
    fn construct(input_lines: &[String]) -> Self {
        let mut tiles: HashMap<u64, Tile> = HashMap::new();
        let mut edges: HashMap<String, u64> = HashMap::new();
        for input in input_lines.chunks(TILE_SIZE + 2) {
            let mut tile = Tile::parse(input);
            for index in 0..4 {
                if let Some(partner_id) = edges.get(&tile.edges[index]) {
                    tiles.get_mut(partner_id).unwrap().edge_matched(&tile.edges[index], tile.id);
                    tile.partners[index] = Some(*partner_id);
                } else {
                    let opposite_edge: String = tile.edges[index].chars().rev().collect();
                    if let Some(partner_id) = edges.get(&opposite_edge) {
                        tiles.get_mut(partner_id).unwrap().edge_matched(&opposite_edge, tile.id);
                        tile.partners[index] = Some(*partner_id);
                    } else {
                        edges.insert(tile.edges[index].clone(), tile.id);
                    }
                }
            }
            tiles.insert(tile.id, tile);
        }
        let square_size = (tiles.len() as f32).sqrt() as usize;

        Self { tiles }
    }

    fn find_corners(&self) -> Vec<u64> {
        //self.tiles.values().filter(|tile| tile.edges.iter().map(|edge| matches!(edge, Edge::Matched(_))).count() == 2).map(|tile| tile.id).collect()
        self.tiles.values().filter_map(|tile| {
            let mut match_count = 0;
            for edge in tile.edges.iter() {
                //if matches!(edge, Edge::Matched(_)) { match_count += 1; }
            }
            if match_count == 2 {
                Some(tile.id)
            } else {
                None
            }
        }).collect()
        //map(|tile| tile.edges.iter().map(|edge| matches!(edge, Edge::Matched(_))).count()).filter(|count| *count == 2).collect()
        //     let matches
        //     //println!("---------\nTile {}", tile.id);
        //     for edge in tile.edges.iter() {
        //         if let Edge::Matched(_) = edge { matches += 1; }
        //     }
        //         let opposite_edge: String = edge.chars().rev().collect();
        //         //println!("Edge {} - straight match {:?}, flipped match {:?}", edge, self.edges.get, self.edges.get(&opposite_edge));
        //         if self.edges.contains_key(&opposite_edge) { matches += 1; }
        //     }
        //     if matches == 2 {
        //         Some(tile.id)
        //     } else {
        //         None
        //     }
        // }).collect()
    }
}

enum Edge {
    Matched(u64),
    Unmatched(String),
}

struct Tile {
    id: u64,
    data: [[bool; TILE_SIZE]; TILE_SIZE],
    edges: [String; 4],
    partners: [Option<u64>; 4],
}

impl Tile {
    fn parse(input_lines: &[String]) -> Self {
        let id = input_lines[0][5..9].parse::<u64>().expect("Invalid input: can't read tile ID");
        let mut data: [[bool; TILE_SIZE]; TILE_SIZE] = [[false; TILE_SIZE]; TILE_SIZE];
        for (row, line) in input_lines[1..=TILE_SIZE].iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                data[row][col] = c == '#';
            }
        }
        let mut edges = [String::with_capacity(TILE_SIZE), String::with_capacity(TILE_SIZE), String::with_capacity(TILE_SIZE), String::with_capacity(TILE_SIZE)];
        let mut build_edge = |edge: usize, row: usize, col: usize| {
            let c: char = if data[row][col] { '#' } else { '.' };
            edges[edge].push(c);
        };
        for index in 0..TILE_SIZE {
            build_edge(0, 0, index);
            build_edge(1, index, TILE_SIZE - 1);
            build_edge(2, TILE_SIZE - 1, index);
            build_edge(3, index, 0);
        }
        Self { id, data, edges, partners: [None; 4] }
    }

    fn edge_matched(&mut self, edge_str: &str, partner_id: u64) {
        for index in 0..4 {
            if self.edges[index].eq(edge_str) {
                self.partners[index] = Some(partner_id);
                break;
            }
        }
    }
}