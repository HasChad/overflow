use macroquad::prelude::*;

use crate::{GamePhase, GameState};

pub fn end(state: &mut GameState) {
    if is_key_pressed(KeyCode::R) {
        state.game_phase = GamePhase::Setup;
    }
}
