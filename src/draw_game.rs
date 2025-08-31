use macroquad::prelude::*;

use crate::{GRID_SIZE, GameState, TILE_SIZE, Tile};

pub fn draw(state: &GameState, arrow_texture: &Texture2D) {
    for (index, tile) in state.grid.iter().enumerate() {
        let x = (index % GRID_SIZE) as f32 * TILE_SIZE;
        let y = (index / GRID_SIZE) as f32 * TILE_SIZE;

        let mut texture_param = DrawTextureParams {
            dest_size: Some(Vec2 {
                x: TILE_SIZE,
                y: TILE_SIZE,
            }),
            source: None,
            rotation: 0.,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };

        match *tile {
            Tile::PushUp => {
                texture_param.rotation = (270.0f32).to_radians();
                draw_texture_ex(&arrow_texture, x, y, WHITE, texture_param);
            }
            Tile::PushDown => {
                texture_param.rotation = (90.0f32).to_radians();
                draw_texture_ex(&arrow_texture, x, y, WHITE, texture_param);
            }
            Tile::PushLeft => {
                texture_param.rotation = (180.0f32).to_radians();
                draw_texture_ex(&arrow_texture, x, y, WHITE, texture_param);
            }
            Tile::PushRight => draw_texture_ex(&arrow_texture, x, y, WHITE, texture_param),
            Tile::Empty => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, GRAY),
            Tile::Wall => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, DARKGRAY),
            Tile::Player1 => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, RED),
            Tile::Player2 => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, DARKBLUE),
            Tile::Block1 => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, BLACK),
            Tile::Block2 => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, BLACK),
            _ => (),
        }

        draw_rectangle_lines(x, y, TILE_SIZE, TILE_SIZE, 2.0, BLACK);
    }

    if let Some(index) = state.focused_tile {
        let x = (index % GRID_SIZE) as f32 * TILE_SIZE;
        let y = (index / GRID_SIZE) as f32 * TILE_SIZE;

        if index != 0
            && index != GRID_SIZE - 1
            && index != (GRID_SIZE * GRID_SIZE) - GRID_SIZE
            && index != (GRID_SIZE * GRID_SIZE) - 1
        {
            let alpha = ((get_time() as f32 * 2.0).sin().abs() * 100.0) as u8;

            let alpha_white = Color::from_rgba(255, 255, 255, alpha);
            draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, alpha_white);
        }
    }

    set_default_camera();
    draw_text(
        format!("Phase: {:?}", state.game_phase).as_str(),
        0.0,
        10.0,
        16.0,
        WHITE,
    );

    draw_text(
        format!("Player: {:?}", state.current_player).as_str(),
        0.0,
        20.0,
        16.0,
        WHITE,
    );
}
