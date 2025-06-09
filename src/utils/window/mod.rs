//! Window Management System
//! 
//! This module provides a comprehensive window management system for the Ruty game engine.
//! It handles window creation, configuration, and lifecycle management.
//! 
//! # Features
//! - Window configuration management
//! - Builder pattern for window creation
//! - Fullscreen and windowed mode support
//! - FPS control
//! 
//! # Examples
//! ```rust
//! use ruty::utils::window::{WindowBuilder, WindowConfig};
//! 
//! // Create a window with default settings
//! WindowBuilder::new()
//!     .title("My Game")
//!     .size(1280, 720)
//!     .target_fps(60)
//!     .build()
//!     .await;
//! ```

mod config;
mod builder;

pub use config::WindowConfig;
pub use builder::WindowBuilder;

/// Creates a window with default settings
/// 
/// # Returns
/// A future that resolves when the window is created
pub async fn create_default_window() {
    WindowBuilder::new().build().await;
}

/// Creates a window with custom title and size
/// 
/// # Arguments
/// * `title` - The window title
/// * `width` - The window width in pixels
/// * `height` - The window height in pixels
/// 
/// # Returns
/// A future that resolves when the window is created
pub async fn create_window(title: &'static str, width: i32, height: i32) {
    WindowBuilder::new()
        .title(title)
        .size(width, height)
        .build()
        .await;
}

/// Creates a fullscreen window
/// 
/// # Arguments
/// * `title` - The window title
/// 
/// # Returns
/// A future that resolves when the window is created
pub async fn create_fullscreen_window(title: &'static str) {
    WindowBuilder::new()
        .title(title)
        .fullscreen(true)
        .build()
        .await;
} 