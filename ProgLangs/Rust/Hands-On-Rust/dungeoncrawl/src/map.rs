use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

// the type of tiles that can be on the map
#[derive(Copy, Clone, PartialEq)] // clone adds clone() function, copy will copy instead of move, partialEq allows you to compare with == 
pub enum TileType {
    Wall, 
    Floor
}

// represents the map
pub struct Map {
    pub tiles: Vec<TileType>,
}

// implements striding, allows you to index vector in x,y manner when it is 1D
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    // constructor
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES], // creates num_tiles entries of the floor type
        }
    }

    // renders the map
    pub fn render(&self, ctx: &mut BTerm) {
        // loop through the entire screen
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x,y); // map to the current index based off of x,y

                // render differently based on tile
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                }
            }
        }
    }

    // checks to make sure that the player is in bounds
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_WIDTH
    }

    // see if you can move to a tile
    pub fn can_enter_tile(&self, point: Point) -> bool {
        // check if you are in bounds and the tile is a floor
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] ==  TileType::Floor
    }

    // check is a point is valid for an index
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        // if the point is not in bounds, it cannot be an index
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }

}









