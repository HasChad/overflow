use macroquad::prelude::*;

use crate::{GRID_SIZE, GameState, TILE_SIZE, Tile, resources::Resources};

pub fn draw_game(state: &GameState, textures: &Resources) {
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
                draw_texture_ex(&textures.arrow, x, y, WHITE, texture_param);
            }
            Tile::PushDown => {
                texture_param.rotation = (90.0f32).to_radians();
                draw_texture_ex(&textures.arrow, x, y, WHITE, texture_param);
            }
            Tile::PushLeft => {
                texture_param.rotation = (180.0f32).to_radians();
                draw_texture_ex(&textures.arrow, x, y, WHITE, texture_param);
            }
            Tile::PushRight => draw_texture_ex(&textures.arrow, x, y, WHITE, texture_param),
            Tile::Empty => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, GRAY),
            Tile::Wall => draw_texture_ex(&textures.wall, x, y, WHITE, texture_param),
            Tile::Player1 => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, RED),
            Tile::Player2 => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, DARKBLUE),
            Tile::Block1 => draw_texture_ex(&textures.block, x, y, RED, texture_param),
            Tile::Block2 => draw_texture_ex(&textures.block, x, y, BLUE, texture_param),
            _ => (),
        }

        draw_rectangle_lines(x, y, TILE_SIZE, TILE_SIZE, 2.0, BLACK);
    }

    for line in state.blocked_lines.iter() {
        info!("blocked line = {:?}", line);

        draw_line(
            line.x * TILE_SIZE,
            0.0,
            line.x * TILE_SIZE * GRID_SIZE as f32,
            TILE_SIZE * GRID_SIZE as f32,
            2.0,
            PINK,
        );
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
}

pub fn draw_ui(state: &GameState) {
    draw_text(
        format!("Phase: {:?}", state.game_phase).as_str(),
        5.0,
        15.0,
        16.0,
        WHITE,
    );

    draw_text(
        format!("Turn: {:?}", state.current_player).as_str(),
        5.0,
        30.0,
        16.0,
        WHITE,
    );

    draw_text(
        format!("Pushing Round: {:?}", state.round).as_str(),
        5.0,
        45.0,
        16.0,
        WHITE,
    );

    if let Some(winner) = &state.winner {
        draw_text(
            format!("WINNER!!: {:?}", winner).as_str(),
            5.0,
            60.0,
            16.0,
            WHITE,
        );
    }
}
