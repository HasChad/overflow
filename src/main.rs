// #![windows_subsystem = "windows"]

use egui_macroquad::egui;
use macroquad::prelude::*;

mod app_settings;
mod game_setup;

use app_settings::*;
use game_setup::*;

const TILE_SIZE: f32 = 50.0;
const GRID_SIZE: usize = 11;

#[derive(Clone, PartialEq)]
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

enum GamePhase {
    Start,
    Setup,
    Moving,
    End,
}

struct GameState {
    game_phase: GamePhase,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut camera = Camera2D {
        zoom: vec2(2. / screen_width(), 2. / screen_height()),
        ..Default::default()
    };
    let mut zoomer = ZOOM_DEFAULT;

    // create grid
    let mut grid = vec![Tile::Empty; GRID_SIZE * GRID_SIZE];
    let mut focused_tile: Option<usize>;

    game_setup(&mut grid);

    'game: loop {
        camera_fixer(&mut camera, &mut zoomer);

        let world_mpos = camera.screen_to_world(Vec2 {
            x: mouse_position().0,
            y: mouse_position().1,
        });

        if is_key_pressed(KeyCode::Escape) {
            break 'game;
        }

        if world_mpos.x < TILE_SIZE * GRID_SIZE as f32
            && world_mpos.x >= 0.0
            && world_mpos.y < TILE_SIZE * GRID_SIZE as f32
            && world_mpos.y >= 0.0
        {
            let x = (world_mpos.x / TILE_SIZE) as usize;
            let y = (world_mpos.y / TILE_SIZE) as usize * GRID_SIZE;

            focused_tile = Some(x + y)
        } else {
            focused_tile = None;
        }

        // if is_mouse_button_down(MouseButton::Left) {
        //     if focused_tile.is_some() {
        //         grid[focused_tile.unwrap()] = true;
        //     }
        // }

        // ! draw
        clear_background(BLACK);
        set_camera(&camera);

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Settings")
                .collapsible(false)
                .show(egui_ctx, |ui| {
                    ui.label("Test");
                });
        });

        for (index, tile) in grid.iter().enumerate() {
            let x = (index % GRID_SIZE) as f32 * TILE_SIZE;
            let y = (index / GRID_SIZE) as f32 * TILE_SIZE;

            match *tile {
                Tile::Edge => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, BLACK),
                Tile::PushUp => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, GREEN),
                Tile::PushDown => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, PURPLE),
                Tile::PushLeft => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, BLUE),
                Tile::PushRight => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, YELLOW),
                Tile::Empty => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, WHITE),
                Tile::Wall => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, DARKGRAY),
                Tile::Player1 => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, BLACK),
                Tile::Player2 => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, BLACK),
                Tile::Block1 => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, BLACK),
                Tile::Block2 => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, BLACK),
            }

            if *tile == Tile::Empty {
                draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, WHITE);
            }
            draw_rectangle_lines(x, y, TILE_SIZE, TILE_SIZE, 2.0, BLACK);
        }

        if let Some(index) = focused_tile {
            let x = (index % GRID_SIZE) as f32 * TILE_SIZE;
            let y = (index / GRID_SIZE) as f32 * TILE_SIZE;

            if index == 0
                || index == TILE_SIZE as usize - 1
                || index == (TILE_SIZE * TILE_SIZE) as usize - 1
            {
                let alpha_white = Color::from_rgba(255, 255, 255, 100);
                draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, alpha_white);
            }
        }
        egui_macroquad::draw();

        next_frame().await
    }
}
