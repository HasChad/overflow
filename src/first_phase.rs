use crate::{GRID_SIZE, GamePhase, GameState, Players, TILE_SIZE, Tile};
use macroquad::prelude::*;

pub fn first_phase(state: &mut GameState, m_pos: &Vec2) {
    if m_pos.x >= TILE_SIZE
        && m_pos.x < TILE_SIZE * (GRID_SIZE - 1) as f32
        && m_pos.y >= TILE_SIZE
        && m_pos.y < TILE_SIZE * (GRID_SIZE - 1) as f32
    {
        let x = (m_pos.x / TILE_SIZE) as usize;
        let y = (m_pos.y / TILE_SIZE) as usize * GRID_SIZE;

        state.focused_tile = Some(x + y)
    } else {
        state.focused_tile = None;
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        if let Some(index) = state.focused_tile {
            if state.grid[index] == Tile::Empty {
                match state.current_player {
                    crate::Players::PlayerOne => {
                        state.grid[index] = Tile::Player1;
                        state.current_player = Players::PlayerTwo
                    }
                    crate::Players::PlayerTwo => {
                        state.grid[index] = Tile::Player2;
                        state.current_player = Players::PlayerOne
                    }
                }
            }
        }
    }

    if !state.grid.contains(&Tile::Empty) {
        info!("First Phase Done");
        state.game_phase = GamePhase::Second;
    }
}
