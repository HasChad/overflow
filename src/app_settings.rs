use macroquad::prelude::*;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "epaint".into(),
        window_width: 800,
        window_height: 600,
        icon: None,
        ..Default::default()
    }
}

pub fn camera_fixer(camera: &mut Camera2D) {
    camera.zoom = vec2(2. / screen_width(), 2. / screen_height());

    if screen_width() < 320. {
        request_new_screen_size(320., screen_height());
    }

    if screen_height() < 240. {
        request_new_screen_size(screen_width(), 240.);
    }
}
