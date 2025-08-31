use crate::{GRID_SIZE, GamePhase, GameState, Tile};

pub fn game_setup(state: &mut GameState) {
    *state = GameState::new();

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
            *tile = Tile::PushRight;
            state.button_indexes.p_right.push(index);
        } else if x == GRID_SIZE - 1 && y > 0 && y < GRID_SIZE - 1 {
            *tile = Tile::PushLeft;
            state.button_indexes.p_left.push(index);
        } else if y == 0 && x > 0 && x < GRID_SIZE - 1 {
            *tile = Tile::PushDown;
            state.button_indexes.p_down.push(index);
        } else if y == GRID_SIZE - 1 && x > 0 && x < GRID_SIZE - 1 {
            *tile = Tile::PushUp;
            state.button_indexes.p_up.push(index);
        } else {
            *tile = Tile::Empty
        }
    }

    for index in state.button_indexes.p_down.iter() {
        println!("index up = {}", index);
    }
    for index in state.button_indexes.p_up.iter() {
        println!("index down = {}", index);
    }
    for index in state.button_indexes.p_right.iter() {
        println!("index left = {}", index);
    }
    for index in state.button_indexes.p_left.iter() {
        println!("index right = {}", index);
    }

    println!("Setup Done");
    state.game_phase = GamePhase::First;
}
