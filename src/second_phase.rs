use crate::{GRID_SIZE, GamePhase, GameState, Players, TILE_SIZE, Tile};
use macroquad::prelude::*;

pub fn second_phase(state: &mut GameState, m_pos: &Vec2) {
    if m_pos.x >= 0.0
        && m_pos.x < TILE_SIZE * GRID_SIZE as f32
        && m_pos.y >= 0.0
        && m_pos.y < TILE_SIZE * GRID_SIZE as f32
    {
        let x = (m_pos.x / TILE_SIZE) as usize;
        let y = (m_pos.y / TILE_SIZE) as usize * GRID_SIZE;

        state.focused_tile = Some(x + y);
        info!("index = {}", x + y);
    } else {
        state.focused_tile = None;
    }

    if is_mouse_button_pressed(MouseButton::Right) && state.round > 2 {
        if let Some(index) = state.focused_tile {
            if state.grid[index] == Tile::Wall {
                if state.current_player == Players::PlayerOne {
                    state.grid[index] = Tile::Block1;
                } else if state.current_player == Players::PlayerTwo {
                    state.grid[index] = Tile::Block2;
                }

                state.round += 1;
                match state.current_player {
                    Players::PlayerOne => state.current_player = Players::PlayerTwo,
                    Players::PlayerTwo => state.current_player = Players::PlayerOne,
                }
            }
        }
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        if let Some(index) = state.focused_tile {
            if state.grid[index] == Tile::PushUp
                || state.grid[index] == Tile::PushDown
                || state.grid[index] == Tile::PushRight
                || state.grid[index] == Tile::PushLeft
            {
                state.round += 1;
                match state.current_player {
                    Players::PlayerOne => state.current_player = Players::PlayerTwo,
                    Players::PlayerTwo => state.current_player = Players::PlayerOne,
                }

                let tile_s = GRID_SIZE as usize - 2;

                match state.grid[index] {
                    Tile::PushUp => {
                        let mut count = 0;
                        while count < tile_s - 1 {
                            let plu = tile_s - count;

                            let before = state.grid[index - plu * GRID_SIZE + GRID_SIZE].clone();
                            state.grid[index - plu * GRID_SIZE] = before;

                            count += 1;
                        }
                        state.grid[index - GRID_SIZE] = Tile::Wall;
                    }
                    Tile::PushDown => {
                        let mut count = 0;
                        while count < tile_s - 1 {
                            let plu = tile_s - count;

                            let before = state.grid[index + plu * GRID_SIZE - GRID_SIZE].clone();
                            state.grid[index + plu * GRID_SIZE] = before;

                            count += 1;
                        }
                        state.grid[index + GRID_SIZE] = Tile::Wall;
                    }
                    Tile::PushLeft => {
                        let mut count = 0;
                        while count < tile_s - 1 {
                            let plu = tile_s - count;

                            let before = state.grid[index - plu + 1].clone();
                            state.grid[index - plu] = before;

                            count += 1;
                        }
                        state.grid[index - 1] = Tile::Wall;
                    }
                    Tile::PushRight => {
                        let mut count = 0;
                        while count < tile_s - 1 {
                            let plu = tile_s - count;

                            let before = state.grid[index + plu - 1].clone();
                            state.grid[index + plu] = before;

                            count += 1;
                        }
                        state.grid[index + 1] = Tile::Wall;
                    }
                    _ => (),
                }
            }
        }
    }

    if !state.grid.contains(&Tile::Player2) {
        state.winner = Some(Players::PlayerOne);
        state.game_phase = GamePhase::End;
        state.focused_tile = None;
    }

    if !state.grid.contains(&Tile::Player1) {
        state.winner = Some(Players::PlayerTwo);
        state.game_phase = GamePhase::End;
        state.focused_tile = None;
    }
}
