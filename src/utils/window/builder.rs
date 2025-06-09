//! Window Builder
//! 
//! This module provides the builder pattern implementation for creating windows
//! in the Ruty game engine. It allows for fluent configuration of window properties.

use super::config::WindowConfig;

/// Builder for configuring the game window
/// 
/// This struct implements the builder pattern for creating windows with
/// various configurations. It provides a fluent interface for setting
/// window properties.
/// 
/// # Examples
/// ```rust
/// use ruty::utils::window::WindowBuilder;
/// 
/// WindowBuilder::new()
///     .title("My Game")
///     .size(1280, 720)
///     .target_fps(60)
///     .build()
///     .await;
/// ```
#[derive(Debug)]
pub struct WindowBuilder {
    config: WindowConfig,
}

impl WindowBuilder {
    /// Creates a new window builder with default settings
    /// 
    /// # Returns
    /// A new `WindowBuilder` instance with default configuration
    pub fn new() -> Self {
        Self {
            config: WindowConfig::default(),
        }
    }

    /// Sets the window title
    /// 
    /// # Arguments
    /// * `title` - The title to display in the window's title bar
    /// 
    /// # Returns
    /// The builder instance for method chaining
    pub fn title(mut self, title: &'static str) -> Self {
        self.config.title = title;
        self
    }

    /// Sets the window dimensions
    /// 
    /// # Arguments
    /// * `width` - The width of the window in pixels
    /// * `height` - The height of the window in pixels
    /// 
    /// # Returns
    /// The builder instance for method chaining
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.config.width = width;
        self.config.height = height;
        self
    }

    /// Sets whether the window should start in fullscreen mode
    /// 
    /// # Arguments
    /// * `fullscreen` - Whether the window should be fullscreen
    /// 
    /// # Returns
    /// The builder instance for method chaining
    pub fn fullscreen(mut self, fullscreen: bool) -> Self {
        self.config.fullscreen = fullscreen;
        self
    }

    /// Sets the target frames per second
    /// 
    /// # Arguments
    /// * `fps` - The target FPS (0 for unlimited)
    /// 
    /// # Returns
    /// The builder instance for method chaining
    pub fn target_fps(mut self, fps: u32) -> Self {
        self.config.target_fps = fps;
        self
    }

    /// Builds and initializes the window with the current configuration
    /// 
    /// # Returns
    /// A future that resolves when the window is created
    pub async fn build(self) {
        self.config.apply();
    }
} 