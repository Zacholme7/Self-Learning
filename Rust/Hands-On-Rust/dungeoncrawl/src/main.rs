mod map;
mod player;
mod map_builder;

// prelude for easy imports 
// any module that uses prelude has access to all of these
mod prelude { 
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
}

use prelude::*;

// represents the state of the game
struct State {
    map: Map,
    player: Player,
}

impl State {
    // constructor for state
    fn new() -> Self {
        Self {
            map: Map::new(),
            player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2))
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls(); // clear screen
        self.player.update(ctx, &self.map); // move the player if needed
        self.map.render(ctx); // render the map
        self.player.render(ctx); // render the player
    }
}

fn main() -> BError{
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .build()?;
    main_loop(context, State::new())
}
