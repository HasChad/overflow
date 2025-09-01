use macroquad::prelude::*;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "epaint".into(),
        window_width: 600,
        window_height: 600,
        icon: None,
        ..Default::default()
    }
}

pub fn camera_fixer(camera: &mut Camera2D) {
    camera.zoom = vec2(2. / screen_width(), 2. / screen_height());

    let min_screen_w = 300.0;
    let min_screen_h = 300.0;

    if screen_width() < min_screen_w {
        request_new_screen_size(min_screen_w, screen_height());
    }

    if screen_height() < min_screen_h {
        request_new_screen_size(screen_width(), min_screen_h);
    }
}
