use macroquad::prelude::*;

/// Window configuration options
#[derive(Clone, Copy)]
pub struct WindowConfig {
    /// Window title
    pub title: &'static str,
    /// Initial window width
    pub width: i32,
    /// Initial window height
    pub height: i32,
    /// Whether the window starts in fullscreen
    pub fullscreen: bool,
    /// Target FPS (0 for unlimited)
    pub target_fps: u32,
}

impl Default for WindowConfig {
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

/// Builder for configuring the game window
pub struct WindowBuilder {
    config: WindowConfig,
}

impl WindowBuilder {
    /// Create a new window builder with default settings
    pub fn new() -> Self {
        Self {
            config: WindowConfig::default(),
        }
    }

    /// Set the window title
    pub fn title(mut self, title: &'static str) -> Self {
        self.config.title = title;
        self
    }

    /// Set the window dimensions
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.config.width = width;
        self.config.height = height;
        self
    }

    /// Set whether the window starts in fullscreen
    pub fn fullscreen(mut self, fullscreen: bool) -> Self {
        self.config.fullscreen = fullscreen;
        self
    }

    /// Set the target FPS
    pub fn target_fps(mut self, fps: u32) -> Self {
        self.config.target_fps = fps;
        self
    }

    /// Build and initialize the window with the current configuration
    pub async fn build(self) {
        // Set window properties (note: window title can only be set at initialization in macroquad)
        request_new_screen_size(self.config.width as f32, self.config.height as f32);
        set_fullscreen(self.config.fullscreen);
    }
}

/// Helper function to create a window with default settings
pub async fn create_default_window() {
    WindowBuilder::new().build().await;
}

/// Helper function to create a window with custom title and size
pub async fn create_window(title: &'static str, width: i32, height: i32) {
    WindowBuilder::new()
        .title(title)
        .size(width, height)
        .build()
        .await;
}

/// Helper function to create a fullscreen window
pub async fn create_fullscreen_window(title: &'static str) {
    WindowBuilder::new()
        .title(title)
        .fullscreen(true)
        .build()
        .await;
}

/// Helper function to create a borderless window
pub async fn create_borderless_window(title: &'static str, width: i32, height: i32) {
    WindowBuilder::new()
        .title(title)
        .size(width, height)
        .fullscreen(true)
        .build()
        .await;
}