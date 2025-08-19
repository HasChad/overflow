use macroquad::{miniquad::conf::Icon, prelude::*};

pub fn window_conf() -> Conf {
    Conf {
        window_title: "epaint".into(),
        window_width: 800,
        window_height: 600,
        icon: None,
        ..Default::default()
    }
}

pub const ZOOM_DEFAULT: f32 = 2.0;
const ZOOM_VALUE: f32 = 0.2;

pub fn camera_fixer(camera: &mut Camera2D, zoomer: &mut f32) {
    camera.zoom = vec2(*zoomer / screen_width(), *zoomer / screen_height());

    if screen_width() < 320. {
        request_new_screen_size(320., screen_height());
    }

    if screen_height() < 240. {
        request_new_screen_size(screen_width(), 240.);
    }

    if mouse_wheel().1 > 0. {
        *zoomer = (*zoomer * 10.).round() / 10.;

        *zoomer += ZOOM_VALUE;
    } else if mouse_wheel().1 < 0. && *zoomer > ZOOM_VALUE {
        *zoomer = (*zoomer * 10.).round() / 10.;

        *zoomer -= ZOOM_VALUE;

        if *zoomer < ZOOM_VALUE {
            *zoomer = ZOOM_VALUE;
        }
    }

    if is_mouse_button_down(MouseButton::Middle) {
        let mouse_pos = mouse_delta_position();

        camera.target.x += mouse_pos.x * screen_width() / *zoomer;
        camera.target.y += mouse_pos.y * screen_height() / *zoomer;
    }

    if is_key_pressed(KeyCode::Space) {
        camera.target = Vec2::ZERO;
        *zoomer = ZOOM_DEFAULT;
    }
}
