use crate::GRID_SIZE;
use crate::GamePhase;
use crate::GameState;
use crate::Tile;

pub fn game_setup(state: &mut GameState) {
    for (index, tile) in state.grid.iter_mut().enumerate() {
        let x = index % GRID_SIZE;
        let y = index / GRID_SIZE;

        if (x == 0 && y == 0)
            || (x == 0 && y == GRID_SIZE - 1)
            || (x == GRID_SIZE - 1 && y == GRID_SIZE - 1)
            || (x == GRID_SIZE - 1 && y == 0)
        {
            *tile = Tile::Edge;
        } else if x == 0 && y > 0 && y < GRID_SIZE - 1 {
            *tile = Tile::PushRight
        } else if x == GRID_SIZE - 1 && y > 0 && y < GRID_SIZE - 1 {
            *tile = Tile::PushLeft
        } else if y == 0 && x > 0 && x < GRID_SIZE - 1 {
            *tile = Tile::PushDown
        } else if y == GRID_SIZE - 1 && x > 0 && x < GRID_SIZE - 1 {
            *tile = Tile::PushUp
        } else {
            *tile = Tile::Empty
        }
    }

    state.game_phase = GamePhase::First;
}
