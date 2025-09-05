use macroquad::prelude::*;

use crate::{GRID_SIZE, GameState, Players, TILE_SIZE, Tile, resources::Resources};

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
            Tile::Block2 => draw_texture_ex(&textures.block, x, y, DARKBLUE, texture_param),
            _ => (),
        }

        draw_rectangle_lines(x, y, TILE_SIZE, TILE_SIZE, 2.0, BLACK);
    }

    for (index, tile) in state.grid.iter().enumerate() {
        let x = (index % GRID_SIZE) as f32;
        let y = (index / GRID_SIZE) as f32;

        for line in state.blocked_lines.iter() {
            let texture_param = DrawTextureParams {
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

            if *line.0 == Tile::CantPush {
                if x == line.1.x && y == line.1.y {
                    draw_texture_ex(
                        &textures.cant_push,
                        x * TILE_SIZE,
                        y * TILE_SIZE,
                        WHITE,
                        texture_param,
                    )
                }
            } else if (x == line.1.x || y == line.1.y)
                && (*tile == Tile::PushUp
                    || *tile == Tile::PushDown
                    || *tile == Tile::PushRight
                    || *tile == Tile::PushLeft)
            {
                draw_texture_ex(
                    &textures.cant_push,
                    x * TILE_SIZE,
                    y * TILE_SIZE,
                    WHITE,
                    texture_param,
                )
            }
        }
    }

    if let Some(index) = state.focused_tile {
        let x = (index % GRID_SIZE) as f32 * TILE_SIZE;
        let y = (index / GRID_SIZE) as f32 * TILE_SIZE;

        if index != 0
            && index != GRID_SIZE - 1
            && index != (GRID_SIZE * GRID_SIZE) - GRID_SIZE
            && index != (GRID_SIZE * GRID_SIZE) - 1
        {
            draw_rectangle(
                x,
                y,
                TILE_SIZE,
                TILE_SIZE,
                Color::from_rgba(255, 255, 255, 100),
            );
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
        let text_color = if *winner == Players::PlayerOne {
            RED
        } else {
            BLUE
        };

        draw_text(
            format!("WINNER!!: {:?}", winner).as_str(),
            5.0,
            70.0,
            32.0,
            text_color,
        );

        draw_text(
            format!("Press R to play again").as_str(),
            5.0,
            85.0,
            16.0,
            WHITE,
        );
    }
}
