#![windows_subsystem = "windows"]

use macroquad::prelude::*;

mod app_settings;
mod draw_game;
mod first_phase;
mod game_setup;
mod second_phase;

use app_settings::*;
use draw_game::*;
use first_phase::*;
use game_setup::*;
use second_phase::*;

const TILE_SIZE: f32 = 50.0;
const GRID_SIZE: usize = 6;

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
struct ButtonIndexes {
    p_up: Vec<usize>,
    p_down: Vec<usize>,
    p_left: Vec<usize>,
    p_right: Vec<usize>,
}

#[derive(Debug)]
struct GameState {
    game_phase: GamePhase,
    grid: Vec<Tile>,
    focused_tile: Option<usize>,
    current_player: Players,
    button_indexes: ButtonIndexes,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            game_phase: GamePhase::Setup,
            grid: vec![Tile::Empty; GRID_SIZE * GRID_SIZE],
            focused_tile: None,
            current_player: Players::PlayerOne,
            button_indexes: ButtonIndexes {
                p_up: vec![],
                p_down: vec![],
                p_left: vec![],
                p_right: vec![],
            },
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");
    set_default_filter_mode(FilterMode::Nearest);

    let arrow_texture = load_texture("arrow.png").await.unwrap();

    let mut camera = Camera2D {
        target: Vec2::new(
            GRID_SIZE as f32 / 2.0 * TILE_SIZE,
            GRID_SIZE as f32 / 2.0 * TILE_SIZE,
        ),
        zoom: vec2(2. / screen_width(), 2. / screen_height()),
        ..Default::default()
    };

    let mut game_state = GameState::new();

    game_setup(&mut game_state);

    'game: loop {
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
            GamePhase::Second => second_phase(&mut game_state, &world_mpos),
            GamePhase::End => (),
        }

        // ! draw
        clear_background(BLACK);
        camera_fixer(&mut camera);
        set_camera(&camera);

        draw(&game_state, &arrow_texture);

        next_frame().await
    }
}
