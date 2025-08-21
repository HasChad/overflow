// #![windows_subsystem = "windows"]

use egui_macroquad::egui;
use macroquad::prelude::*;

mod app_settings;
mod first_phase;
mod game_setup;
mod ui;

use app_settings::*;
use first_phase::*;
use game_setup::*;
use ui::*;

const TILE_SIZE: f32 = 50.0;
const GRID_SIZE: usize = 11;

#[derive(Clone, Debug, PartialEq)]
enum Tile {
    Edge,
    PushUp,
    PushDown,
    PushLeft,
    PushRight,
    Empty,
    Wall,
    Player1,
    Player2,
    Block1,
    Block2,
}

#[derive(Debug)]
enum Players {
    PlayerOne,
    PlayerTwo,
}

#[derive(Debug)]
enum GamePhase {
    Setup,
    First,
    Second,
    End,
}

#[derive(Debug)]
struct GameState {
    game_phase: GamePhase,
    grid: Vec<Tile>,
    focused_tile: Option<usize>,
    current_player: Players,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut camera = Camera2D {
        zoom: vec2(2. / screen_width(), 2. / screen_height()),
        ..Default::default()
    };
    let mut zoomer = ZOOM_DEFAULT;

    // create grid
    let mut game_state = GameState {
        game_phase: GamePhase::Setup,
        grid: vec![Tile::Empty; GRID_SIZE * GRID_SIZE],
        focused_tile: None,
        current_player: Players::PlayerOne,
    };

    game_setup(&mut game_state);

    'game: loop {
        camera_fixer(&mut camera, &mut zoomer);

        let world_mpos = camera.screen_to_world(Vec2 {
            x: mouse_position().0,
            y: mouse_position().1,
        });

        if is_key_pressed(KeyCode::Escape) {
            break 'game;
        }

        match game_state.game_phase {
            GamePhase::Setup => game_setup(&mut game_state),
            GamePhase::First => first_phase(&mut game_state, &world_mpos),
            GamePhase::Second => (),
            GamePhase::End => (),
        }

        // ! draw
        clear_background(BLACK);
        set_camera(&camera);

        // ui();

        for (index, tile) in game_state.grid.iter().enumerate() {
            let x = (index % GRID_SIZE) as f32 * TILE_SIZE;
            let y = (index / GRID_SIZE) as f32 * TILE_SIZE;

            match *tile {
                Tile::PushUp => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, GREEN),
                Tile::PushDown => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, PURPLE),
                Tile::PushLeft => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, BLUE),
                Tile::PushRight => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, YELLOW),
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

        if let Some(index) = game_state.focused_tile {
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
            format!("Phase: {:?}", game_state.game_phase).as_str(),
            0.0,
            10.0,
            16.0,
            WHITE,
        );

        draw_text(
            format!("Player: {:?}", game_state.current_player).as_str(),
            0.0,
            20.0,
            16.0,
            WHITE,
        );

        // egui_macroquad::draw();

        next_frame().await
    }
}
