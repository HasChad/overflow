use egui_macroquad::egui;

pub fn ui() {
    egui_macroquad::ui(|egui_ctx| {
        egui::Window::new("Settings")
            .collapsible(false)
            .show(egui_ctx, |ui| {
                ui.label("Test");
            });
    });
}
