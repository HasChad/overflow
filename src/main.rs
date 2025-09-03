#![windows_subsystem = "windows"]

use macroquad::prelude::*;

mod app_settings;
mod draw;
mod end_phase;
mod first_phase;
mod game_setup;
mod resources;
mod second_phase;

use app_settings::*;
use draw::*;
use end_phase::*;
use first_phase::*;
use game_setup::*;
use resources::*;
use second_phase::*;

const TILE_SIZE: f32 = 64.;
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

#[derive(Debug, PartialEq)]
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
    blocked_lines: Vec<Vec2>,
    round: usize,
    winner: Option<Players>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            game_phase: GamePhase::Setup,
            grid: vec![Tile::Empty; GRID_SIZE * GRID_SIZE],
            focused_tile: None,
            current_player: Players::PlayerOne,
            blocked_lines: vec![],
            round: 1,
            winner: None,
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");
    set_default_filter_mode(FilterMode::Nearest);

    let textures = Resources::load_textures().await;

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
            GamePhase::End => end(&mut game_state),
        }

        // ! draw
        clear_background(BLACK);

        camera_fixer(&mut camera);
        set_camera(&camera);
        draw_game(&game_state, &textures);

        set_default_camera();
        draw_ui(&game_state);

        next_frame().await
    }
}
