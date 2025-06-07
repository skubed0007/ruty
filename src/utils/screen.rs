use macroquad::window::{screen_height, screen_width};

/// Returns the current screen width in pixels.
pub fn get_screen_width() -> f32 {
    screen_width()
}

/// Returns the current screen height in pixels.
pub fn get_screen_height() -> f32 {
    screen_height()
}

/// Returns the Y coordinate for the bottom of the screen.
/// Useful for placing UI elements or ground platforms.
pub fn get_screen_bottom() -> f32 {
    screen_height()
}

/// Returns the Y coordinate where a ground object should be drawn
/// so that it aligns with the bottom of the screen.
pub fn get_ground_y(ground_height: f32) -> f32 {
    screen_height() - ground_height
}

/// Returns the Y coordinate for the top of the screen (usually 0.0).
pub fn get_screen_top() -> f32 {
    0.0
}

/// Returns the center coordinates of the screen as (x, y).
pub fn get_screen_center() -> (f32, f32) {
    (screen_width() / 2.0, screen_height() / 2.0)
}
