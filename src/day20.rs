use std::collections::HashMap;

const TILE_SIZE: usize = 10;
const SQUARE_SIDE: usize = 12;
const STRIPPED_TILE_SIZE: usize = TILE_SIZE - 2;
const IMAGE_SIZE: usize = SQUARE_SIDE * STRIPPED_TILE_SIZE;

const SEA_MONSTER_ROWS: usize = 3;
const SEA_MONSTER: [&str; SEA_MONSTER_ROWS] = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
];
const SEA_MONSTER_COLS: usize = SEA_MONSTER[0].len();
lazy_static! {
    static ref SEA_MONSTER_CELLS: Vec<(usize, usize)> = {
        let mut cells: Vec<(usize, usize)> = Vec::new();
        for (row_index, row) in SEA_MONSTER.iter().enumerate() {
            for (col_index, cell) in row.chars().enumerate() {
                if cell == '#' { cells.push((row_index, col_index)); }
            }
        }
        cells
    };
}

pub fn day20(input_lines: &[String]) -> (u64, u64) {
    let mut image = Image::construct(input_lines);
    let part1: u64 = Image::find_corners(&image.tiles_by_id).product();
    let part2: u64 = image.water_roughness() as u64;
    (part1,part2)
}

struct Image {
    tiles_by_id: HashMap<u64, Tile>,
    data: [[bool; IMAGE_SIZE]; IMAGE_SIZE],
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::with_capacity((SQUARE_SIDE + 1) * SQUARE_SIDE);
        for row in self.data.iter() {
            for cell in row {
                if *cell { output.push('#'); } else { output.push('.'); }
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}

impl Image {
    fn construct(input_lines: &[String]) -> Self {
        let mut tiles_by_id = Self::construct_tiles(input_lines);
        let tiles = Self::arrange_tiles(&mut tiles_by_id);
        let data = Self::construct_image(&tiles_by_id, &tiles);
        Self { tiles_by_id, data }
    }

    fn construct_tiles(input_lines: &[String]) -> HashMap<u64, Tile> {
        let mut tiles: HashMap<u64, Tile> = HashMap::new();
        let mut edges: HashMap<String, u64> = HashMap::new();

        // We know how many lines each tile will contain - use that to iterate
        // over chunks of lines.
        for input in input_lines.chunks(TILE_SIZE + 2) {
            // Parse the chunk into a Tile structure.
            let mut tile = Tile::parse(input);

            // Try to match up each edge to a tile we've already parsed. We'll
            // worry about rotation/flipping later, so for now consider any edge
            // to any other edge, including matching 1234 to 4321.
            for index in 0..4 {
                if let Some(partner_id) = edges.get(&tile.edges[index]) {
                    tiles.get_mut(partner_id).unwrap().edge_matched(&tile.edges[index], tile.id);
                    tile.partners[index] = Some(*partner_id);
                    edges.remove(&tile.edges[index]);
                } else {
                    let opposite_edge: String = tile.edges[index].chars().rev().collect();
                    if let Some(partner_id) = edges.get(&opposite_edge) {
                        tiles.get_mut(partner_id).unwrap().edge_matched(&opposite_edge, tile.id);
                        tile.partners[index] = Some(*partner_id);
                        edges.remove(&opposite_edge);
                    } else {
                        // We haven't matched to a previous tile, so store off
                        // for a future match.
                        edges.insert(tile.edges[index].clone(), tile.id);
                    }
                }
            }
            tiles.insert(tile.id, tile);
        }
        tiles
    }

    fn arrange_tiles(map: &mut HashMap<u64, Tile>) -> [[u64; SQUARE_SIDE]; SQUARE_SIDE] {
        let mut tiles = [[0u64; SQUARE_SIDE]; SQUARE_SIDE];

        // Each tile now knows which other tiles border it, but we haven't oriented or
        // arranged them - do so now.
        //
        // The logic is:
        // -  Choose an arbitrary corner tile to be at the top-left.  (We know which tiles
        //    are in the corners because they have two bordering tiles.)
        // -  Rotate that tile so that it has partners below and to the right.
        // -  Work along the row: slot our right-hand partner into place, and align it so
        //    that its edge matches up to ours; repeat all the way along the row.
        // -  Do the same thing for our partner below to kick off the next row, and then
        //    work all the way along THAT row. Repeat for all rows.
        //
        // This code would be neater if we could just have variables for Tiles themselves,
        // rather than separately remembering a bunch of IDs and edges, but the borrow
        // checker makes it tricky to hold references to multiple things in the HashMap
        // simultaneously when one of them needs to be mutable.  We could do better,
        // especially if we made the contents of the map RefCell<Tile>, but I've done
        // enough of that for one AoC.
        let mut first_tile_id_in_previous_row: Option<u64> = None;
        let mut bottom_edge_in_previous_row = String::new();
        for row in tiles.iter_mut() {
            let first_tile_in_row = if let Some(previous_tile_id) = first_tile_id_in_previous_row {
                // This is not the first row - find the tile that goes below the
                // one at the start of the previous row, and align it to fit
                // below.
                let previous_tile = map.get(&previous_tile_id).unwrap();
                let new_tile_id = previous_tile.partners[2].unwrap();
                let new_tile = map.get_mut(&new_tile_id).unwrap();
                new_tile.align_to_below(previous_tile_id, &bottom_edge_in_previous_row);
                new_tile
            } else {
                // This is the first row.
                // Select an arbitrary corner tile to be the top-left corner.
                let new_tile_id = Self::find_corners(map).next().unwrap();
                let new_tile = map.get_mut(&new_tile_id).unwrap();
                new_tile.align_to_top_left_corner();
                new_tile
            };

            // Slot this tile in place.
            row[0] = first_tile_in_row.id;

            // Store off information for the next row loop.
            first_tile_id_in_previous_row = Some(first_tile_in_row.id);
            bottom_edge_in_previous_row = first_tile_in_row.edges[2].clone();

            // Prepare for the columns loop.
            let mut previous_tile_id = first_tile_in_row.id;
            let mut previous_tile_right_edge = first_tile_in_row.edges[1].clone();
            let mut next_tile_id = first_tile_in_row.partners[1];

            // Loop through columns, finding the right-hand partner, aligning it using our
            // right-hand edge and slotting it into place.
            for cell in row.iter_mut().skip(1) {
                let next_tile_id_unwrapped = next_tile_id.unwrap();
                *cell = next_tile_id_unwrapped;
                let next_tile = map.get_mut(&next_tile_id_unwrapped).unwrap();
                next_tile.align_to_right_of(previous_tile_id, &previous_tile_right_edge);
                previous_tile_id = next_tile_id_unwrapped;
                previous_tile_right_edge = next_tile.edges[1].clone();
                next_tile_id = next_tile.partners[1];                
            }
        }

        tiles
    }

    fn construct_image(map: &HashMap<u64, Tile>, ids: &[[u64; SQUARE_SIDE]]) -> [[bool; IMAGE_SIZE]; IMAGE_SIZE] {
        // Build the complete image by copying the non-edge data from each tile.
        let mut image = [[false; IMAGE_SIZE]; IMAGE_SIZE];
        for (tile_row, ids_row) in ids.iter().enumerate() {
            for (tile_col, id) in ids_row.iter().enumerate() {
                let tile = map.get(id).unwrap();
                for row_in_tile in 0..STRIPPED_TILE_SIZE {
                    for col_in_tile in 0..STRIPPED_TILE_SIZE {
                        let row = (tile_row * STRIPPED_TILE_SIZE) + row_in_tile;
                        let col = (tile_col * STRIPPED_TILE_SIZE) + col_in_tile;
                        image[row][col] = tile.data[row_in_tile+1][col_in_tile+1];
                    }
                }
            }
        }
        image
    }

    fn find_corners(map: &HashMap<u64, Tile>) -> impl Iterator<Item = u64> + '_ {
        // The corners are the tiles with two partners.
        map.values().filter(|tile| tile.partners.iter().filter(|partner| partner.is_some()).count() == 2).map(|tile| tile.id)
    }

    fn rotate(&mut self) {
        // Rotate a the whole image clockwise.
        let mut new_data = [[false; IMAGE_SIZE]; IMAGE_SIZE];
        for (row, row_data) in self.data.iter().enumerate() {
            for (col, cell) in row_data.iter().enumerate() {
                new_data[col][IMAGE_SIZE - row - 1] = *cell;
            }
        }
        self.data = new_data;
    }

    fn flip(&mut self) {
        // Perform a vertical flip of this image, so that the first row
        // becomes the last row.
        let mut new_data = [[false; IMAGE_SIZE]; IMAGE_SIZE];
        for (row, row_data) in self.data.iter().enumerate() {
            new_data[IMAGE_SIZE - row - 1] = *row_data;
        }
        self.data = new_data;
    }    

    fn water_roughness(&mut self) -> usize {
        self.count_filled_cells() - (self.find_and_count_sea_monsters() * SEA_MONSTER_CELLS.len())
    }

    fn count_filled_cells(&self) -> usize {
        self.data.iter().flatten().filter(|cell| **cell).count()
    }

    fn find_and_count_sea_monsters(&mut self) -> usize {
        // Try every orientation until we can find at least one sea monster.
        let mut sea_monsters = self.count_sea_monsters();
        if sea_monsters == 0 {
            for _ in 0..3 {
                self.rotate();
                sea_monsters = self.count_sea_monsters();
                if sea_monsters != 0 { break; }
            }
        }
        if sea_monsters == 0 {
            self.flip();
            sea_monsters = self.count_sea_monsters();
        }
        if sea_monsters == 0 {
            for _ in 0..3 {
                self.rotate();
                sea_monsters = self.count_sea_monsters();
                if sea_monsters != 0 { break; }
            }
        }
        sea_monsters
    }

    fn count_sea_monsters(&self) -> usize {
        let mut sea_monsters = 0usize;
        for row_index in 0..=IMAGE_SIZE-SEA_MONSTER_ROWS {
            for col_index in 0..=IMAGE_SIZE-SEA_MONSTER_COLS {
                if self.sea_monster_at(row_index, col_index) { sea_monsters += 1; }
            }
        }
        sea_monsters
    }

    fn sea_monster_at(&self, row_index: usize, col_index: usize) -> bool {
        SEA_MONSTER_CELLS.iter().all(|(row, col)| self.data[row + row_index][col + col_index])
    }
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

        // Store off all the tile cells in a 2D array.
        let mut data: [[bool; TILE_SIZE]; TILE_SIZE] = [[false; TILE_SIZE]; TILE_SIZE];
        for (row, line) in input_lines[1..=TILE_SIZE].iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                data[row][col] = c == '#';
            }
        }

        // Construct the edges - top, right, bottom, left. Edges are parsed in a clockwise
        // manner, i.e. the top is read left-to-right while the bottom is read right-to-left.
        let mut edges = [String::with_capacity(TILE_SIZE), String::with_capacity(TILE_SIZE), String::with_capacity(TILE_SIZE), String::with_capacity(TILE_SIZE)];
        let mut build_edge = |edge: usize, row: usize, col: usize| {
            let c: char = if data[row][col] { '#' } else { '.' };
            edges[edge].push(c);
        };
        for index in 0..TILE_SIZE {
            build_edge(0, 0, index);
            build_edge(1, index, TILE_SIZE - 1);
            build_edge(2, TILE_SIZE - 1, TILE_SIZE - index - 1);
            build_edge(3, TILE_SIZE - index - 1, 0);
        }

        Self { id, data, edges, partners: [None; 4] }
    }

    fn edge_matched(&mut self, edge_str: &str, partner_id: u64) {
        // Record that we've matched up one of our edges to a partner.
        for index in 0..4 {
            if self.edges[index].eq(edge_str) {
                self.partners[index] = Some(partner_id);
                break;
            }
        }
    }

    // Rotate this tile (must be a corner tile) so that it's in the top left.
    fn align_to_top_left_corner(&mut self) {
        let times_to_rotate = if self.partners[0].is_some() {
            if self.partners[1].is_some() { 1 } else { 2 }
        } else if self.partners[3].is_some() { 3 } else { 0 };
        self.rotate(times_to_rotate);
    }

    // Rotate/flip this tile such that the specified tile is to its left and the edge matches up.
    fn align_to_right_of(&mut self, tile_to_left: u64, edge: &str) {
        let times_to_rotate = 3 - self.find_partner_direction(tile_to_left);
        self.rotate(times_to_rotate);
        if !self.edge_matches(3, edge) { self.flip_vertical(); }
    }

    // Rotate this tile such that the specified tile is above it.
    fn align_to_below(&mut self, tile_above: u64, edge: &str) {
        let times_to_rotate = (4 - self.find_partner_direction(tile_above)) % 4;
        self.rotate(times_to_rotate);
        if !self.edge_matches(0, edge) { self.flip_horizontal(); }
    }

    fn find_partner_direction(&self, partner_id: u64) -> usize {
        // Which of this tile's partners is the specified tile ID? Returns an index into
        // the partners/edges arrays.
        self.partners.iter().enumerate().find(|(_, partner)| matches!(**partner, Some(tile_id) if tile_id == partner_id)).expect("Mismatching partners").0
    }

    fn rotate(&mut self, times: usize) {
        // Rotate a tile clockwise, X times. Yeah, we could do this much more efficiently by
        // applying different logic depending on how many times we're rotating, but CBA.
        for _ in 0..times {
            let mut new_data = [[false; TILE_SIZE]; TILE_SIZE];
            for (row, row_data) in self.data.iter().enumerate() {
                for (col, cell) in row_data.iter().enumerate() {
                    new_data[col][TILE_SIZE - row - 1] = *cell;
                }
            }
            self.data = new_data;
            self.edges.rotate_right(1);
            self.partners.rotate_right(1);      
        }
    }

    fn edge_matches(&self, edge_index: usize, other_edge: &str) -> bool {
        // Does one of our edges match another edge? This function is used
        // during alignment, not during initial matching, so we specifically
        // require the edge of one tile to be the reverse of the matching
        // edge. If they match without reversing, the tile needs to be
        // flipped.
        self.edges[edge_index].chars().rev().eq(other_edge.chars())
    }

    fn flip_vertical(&mut self) {
        // Perform a vertical flip of this tile, so that the first row
        // becomes the last row.
        let mut new_data = [[false; TILE_SIZE]; TILE_SIZE];
        for (row, row_data) in self.data.iter().enumerate() {
            new_data[TILE_SIZE - row - 1] = *row_data;
        }
        self.data = new_data;
        let new_edge_0: String = self.edges[2].chars().rev().collect();
        self.edges[2] = self.edges[0].chars().rev().collect();
        self.edges[0] = new_edge_0;
        self.edges[1] = self.edges[1].chars().rev().collect();
        self.edges[3] = self.edges[3].chars().rev().collect();
        self.partners.swap(0, 2);
    }

    fn flip_horizontal(&mut self) {   
        // Perform a horizontal flip of this tile, so that the first column
        // becomes the last column.     
        let mut new_data = [[false; TILE_SIZE]; TILE_SIZE];
        for (row, row_data) in self.data.iter().enumerate() {
            for (col, cell) in row_data.iter().enumerate() {
                new_data[row][TILE_SIZE - col - 1] = *cell;
            }
        }
        self.data = new_data;
        let new_edge_1: String = self.edges[3].chars().rev().collect();
        self.edges[3] = self.edges[1].chars().rev().collect();
        self.edges[1] = new_edge_1;
        self.edges[0] = self.edges[0].chars().rev().collect();
        self.edges[2] = self.edges[2].chars().rev().collect();
        self.partners.swap(1, 3);
    }
}