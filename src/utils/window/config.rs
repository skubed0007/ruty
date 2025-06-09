//! Window Configuration
//! 
//! This module provides the configuration system for windows in the Ruty game engine.
//! It defines the structure and behavior of window settings.

use macroquad::prelude::*;

/// Window configuration options
/// 
/// This struct holds all configurable properties of a window.
/// It implements the builder pattern for easy window creation.
/// 
/// # Examples
/// ```rust
/// use ruty::utils::window::WindowConfig;
/// 
/// let config = WindowConfig {
///     title: "My Game",
///     width: 1280,
///     height: 720,
///     fullscreen: false,
///     target_fps: 60,
/// };
/// ```
#[derive(Clone, Copy, Debug)]
pub struct WindowConfig {
    /// The title displayed in the window's title bar
    pub title: &'static str,
    /// The initial width of the window in pixels
    pub width: i32,
    /// The initial height of the window in pixels
    pub height: i32,
    /// Whether the window should start in fullscreen mode
    pub fullscreen: bool,
    /// The target frames per second (0 for unlimited)
    pub target_fps: u32,
}

impl Default for WindowConfig {
    /// Creates a new window configuration with default values
    /// 
    /// # Returns
    /// A new `WindowConfig` with the following defaults:
    /// - title: "Ruty Game"
    /// - width: 800
    /// - height: 600
    /// - fullscreen: false
    /// - target_fps: 60
    fn default() -> Self {
        Self {
            title: "Ruty Game",
            width: 800,
            height: 600,
            fullscreen: false,
            target_fps: 60,
        }
    }
}

impl WindowConfig {
    /// Applies the configuration to the current window
    /// 
    /// This method sets all window properties according to the configuration.
    /// It should be called after creating a new window.
    pub fn apply(&self) {
        request_new_screen_size(self.width as f32, self.height as f32);
        set_fullscreen(self.fullscreen);
        // Note: FPS is set through the macroquad main attribute
    }
} 