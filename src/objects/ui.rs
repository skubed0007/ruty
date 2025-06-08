//! UI System for Ruty Game Engine
//! 
//! This module provides a comprehensive set of UI components for creating
//! interactive user interfaces in games. The system is built around a theme-based
//! design that ensures consistent styling across all components.
//! 
//! # Examples
//! 
//! ```rust
//! use ruty::objects::ui::{Theme, UiButton, UiPanel};
//! 
//! let theme = Theme::default();
//! let mut panel = UiPanel::new(20.0, 20.0, 300.0, 500.0, theme.clone(), Some("My Panel".to_string()));
//! 
//! let button = UiButton::new(
//!     "Click Me",
//!     40.0,
//!     60.0,
//!     220.0,
//!     40.0,
//!     18,
//!     font.clone(),
//!     theme.clone(),
//!     Some(Box::new(|| println!("Button clicked!"))),
//! );
//! panel.add_element(Box::new(button));
//! ```

use macroquad::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::any::Any;

/// Theme for UI components
#[derive(Clone)]
pub struct Theme {
    /// Primary color for main elements
    pub primary: Color,
    /// Secondary color for less prominent elements
    pub secondary: Color,
    /// Accent color for highlights and interactive elements
    pub accent: Color,
    /// Background color for panels and containers
    pub background: Color,
    /// Text color
    pub text: Color,
    /// Error state color
    pub error: Color,
    /// Success state color
    pub success: Color,
    /// Border radius for rounded corners
    pub border_radius: f32,
    /// Padding for elements
    pub padding: f32,
    /// Animation speed for transitions
    pub animation_speed: f32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: Color::from_rgba(0, 120, 215, 255),
            secondary: Color::from_rgba(45, 45, 45, 255),
            accent: Color::from_rgba(0, 153, 204, 255),
            background: Color::from_rgba(30, 30, 30, 255),
            text: Color::from_rgba(255, 255, 255, 255),
            error: Color::from_rgba(255, 59, 48, 255),
            success: Color::from_rgba(52, 199, 89, 255),
            border_radius: 4.0,
            padding: 8.0,
            animation_speed: 0.2,
        }
    }
}

/// Animation helper for smooth transitions
#[derive(Clone)]
pub struct Animation {
    pub current: f32,
    pub target: f32,
    pub speed: f32,
}

impl Animation {
    /// Create a new animation
    pub fn new(initial: f32, speed: f32) -> Self {
        Self {
            current: initial,
            target: initial,
            speed,
        }
    }

    /// Update the animation
    pub fn update(&mut self) {
        self.current += (self.target - self.current) * self.speed;
    }

    /// Set the target value
    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    /// Get the current value
    pub fn get_current(&self) -> f32 {
        self.current
    }
}

/// Trait for all UI elements
pub trait UiElement {
    /// Draw the UI element
    fn draw(&self, theme: &Theme);
    
    /// Update the UI element's state
    fn update(&mut self, theme: &Theme);
    
    /// Get the bounds of the UI element
    fn get_bounds(&self) -> (f32, f32, f32, f32);
    
    /// Get a reference to the element as Any
    fn as_any(&self) -> &dyn Any;
    
    /// Get a mutable reference to the element as Any
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Helper function to draw rounded rectangles
fn draw_rounded_rectangle(x: f32, y: f32, width: f32, height: f32, radius: f32, color: Color) {
    // Draw the main rectangle
    draw_rectangle(x + radius, y, width - 2.0 * radius, height, color);
    draw_rectangle(x, y + radius, width, height - 2.0 * radius, color);
    
    // Draw the corner circles
    let diameter = radius * 2.0;
    draw_rectangle(x, y, diameter, diameter, color);
    draw_rectangle(x + width - diameter, y, diameter, diameter, color);
    draw_rectangle(x, y + height - diameter, diameter, diameter, color);
    draw_rectangle(x + width - diameter, y + height - diameter, diameter, diameter, color);
}

/// Text UI element
pub struct UiText {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub font_size: u16,
    pub color: Color,
    pub font: Font,
    pub animation: Animation,
    pub alignment: TextAlignment,
}

/// Text alignment options
#[derive(Clone, Copy)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

impl UiText {
    /// Create a new text element
    pub fn new(text: &str, x: f32, y: f32, font_size: u16, color: Color, font: Font) -> Self {
        Self {
            text: text.to_string(),
            x,
            y,
            font_size,
            color,
            font,
            animation: Animation::new(1.0, 0.2),
            alignment: TextAlignment::Left,
        }
    }

    /// Set the text alignment
    pub fn set_alignment(&mut self, alignment: TextAlignment) {
        self.alignment = alignment;
    }
}

impl UiElement for UiText {
    fn draw(&self, theme: &Theme) {
        let alpha = self.animation.current;
        let color = Color::new(
            self.color.r,
            self.color.g,
            self.color.b,
            self.color.a * alpha as f32,
        );

        let text_dim = measure_text(&self.text, Some(&self.font), self.font_size, 1.0);
        let x = match self.alignment {
            TextAlignment::Left => self.x,
            TextAlignment::Center => self.x - text_dim.width / 2.0,
            TextAlignment::Right => self.x - text_dim.width,
        };

        draw_text_ex(
            &self.text,
            x,
            self.y,
            TextParams {
                font: Some(&self.font),
                font_size: self.font_size,
                color,
                ..Default::default()
            },
        );
    }

    fn update(&mut self, _theme: &Theme) {
        self.animation.update();
    }

    fn get_bounds(&self) -> (f32, f32, f32, f32) {
        let text_dim = measure_text(&self.text, Some(&self.font), self.font_size, 1.0);
        (self.x, self.y, text_dim.width, text_dim.height)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Button UI element
pub struct UiButton {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub font_size: u16,
    pub font: Font,
    pub theme: Theme,
    pub on_click: Option<Box<dyn FnMut() + Send + Sync>>,
    pub hover_animation: Animation,
    pub press_animation: Animation,
    pub disabled: bool,
}

impl UiButton {
    /// Create a new button
    pub fn new(
        text: &str,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        font_size: u16,
        font: Font,
        theme: Theme,
        on_click: Option<Box<dyn FnMut() + Send + Sync>>,
    ) -> Self {
        Self {
            text: text.to_string(),
            x,
            y,
            w,
            h,
            font_size,
            font,
            theme,
            on_click,
            hover_animation: Animation::new(0.0, 0.2),
            press_animation: Animation::new(0.0, 0.3),
            disabled: false,
        }
    }

    pub fn is_mouse_over(&self) -> bool {
        let (mx, my) = mouse_position();
        mx >= self.x && mx <= self.x + self.w && my >= self.y && my <= self.y + self.h
    }
}

impl UiElement for UiButton {
    fn draw(&self, theme: &Theme) {
        let hover = self.hover_animation.current;
        let press = self.press_animation.current;
        
        let base_color = if self.disabled {
            theme.secondary
        } else {
            theme.primary
        };

        let color = Color::new(
            base_color.r + (theme.accent.r - base_color.r) * hover,
            base_color.g + (theme.accent.g - base_color.g) * hover,
            base_color.b + (theme.accent.b - base_color.b) * hover,
            base_color.a,
        );

        // Draw button background with rounded corners
        draw_rounded_rectangle(
            self.x,
            self.y,
            self.w,
            self.h,
            theme.border_radius,
            color,
        );

        // Draw pressed effect
        if press > 0.0 {
            draw_rounded_rectangle(
                self.x,
                self.y,
                self.w,
                self.h * press,
                theme.border_radius,
                Color::new(0.0, 0.0, 0.0, 0.2),
            );
        }

        // Draw text
        let text_dim = measure_text(&self.text, Some(&self.font), self.font_size, 1.0);
        let tx = self.x + (self.w - text_dim.width) / 2.0;
        let ty = self.y + (self.h + text_dim.height) / 2.0 - 4.0;
        
        let text_color = if self.disabled {
            Color::new(0.5, 0.5, 0.5, 1.0)
        } else {
            theme.text
        };

        draw_text_ex(
            &self.text,
            tx,
            ty,
            TextParams {
                font: Some(&self.font),
                font_size: self.font_size,
                color: text_color,
                ..Default::default()
            },
        );
    }

    fn update(&mut self, _theme: &Theme) {
        if self.disabled {
            self.hover_animation.set_target(0.0);
            self.press_animation.set_target(0.0);
        } else {
            self.hover_animation.set_target(if self.is_mouse_over() { 1.0 } else { 0.0 });
            
            let mouse_down = is_mouse_button_down(MouseButton::Left);
            self.press_animation.set_target(if mouse_down && self.is_mouse_over() { 1.0 } else { 0.0 });

            if self.is_mouse_over() && is_mouse_button_pressed(MouseButton::Left) {
                if let Some(cb) = &mut self.on_click {
                    cb();
                }
            }
        }

        self.hover_animation.update();
        self.press_animation.update();
    }

    fn get_bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.w, self.h)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Input field UI element
pub struct UiInput {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub font_size: u16,
    pub font: Font,
    pub theme: Theme,
    pub placeholder: String,
    pub focused: bool,
    pub cursor_position: usize,
    pub cursor_animation: Animation,
    pub on_change: Option<Box<dyn FnMut(&str) + Send + Sync>>,
}

impl UiInput {
    /// Create a new input field
    pub fn new(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        font_size: u16,
        font: Font,
        theme: Theme,
        placeholder: &str,
        on_change: Option<Box<dyn FnMut(&str) + Send + Sync>>,
    ) -> Self {
        Self {
            text: String::new(),
            x,
            y,
            w,
            h,
            font_size,
            font,
            theme,
            placeholder: placeholder.to_string(),
            focused: false,
            cursor_position: 0,
            cursor_animation: Animation::new(1.0, 0.1),
            on_change,
        }
    }

    pub fn is_mouse_over(&self) -> bool {
        let (mx, my) = mouse_position();
        mx >= self.x && mx <= self.x + self.w && my >= self.y && my <= self.y + self.h
    }
}

impl UiElement for UiInput {
    fn draw(&self, theme: &Theme) {
        // Draw background
        draw_rounded_rectangle(
            self.x,
            self.y,
            self.w,
            self.h,
            theme.border_radius,
            if self.focused {
                Color::new(0.2, 0.2, 0.2, 1.0)
            } else {
                theme.secondary
            },
        );

        // Draw border
        let border_color = if self.focused {
            theme.accent
        } else {
            Color::new(0.3, 0.3, 0.3, 1.0)
        };
        draw_rectangle_lines(
            self.x,
            self.y,
            self.w,
            self.h,
            1.0,
            border_color,
        );

        // Draw text or placeholder
        let display_text = if self.text.is_empty() && !self.focused {
            &self.placeholder
        } else {
            &self.text
        };

        let text_color = if self.text.is_empty() && !self.focused {
            Color::new(0.5, 0.5, 0.5, 1.0)
        } else {
            theme.text
        };

        let text_dim = measure_text(display_text, Some(&self.font), self.font_size, 1.0);
        let tx = self.x + theme.padding;
        let ty = self.y + (self.h + text_dim.height) / 2.0 - 4.0;

        draw_text_ex(
            display_text,
            tx,
            ty,
            TextParams {
                font: Some(&self.font),
                font_size: self.font_size,
                color: text_color,
                ..Default::default()
            },
        );

        // Draw cursor if focused
        if self.focused {
            let cursor_x = tx + measure_text(
                &self.text[..self.cursor_position],
                Some(&self.font),
                self.font_size,
                1.0,
            ).width;

            if self.cursor_animation.current > 0.5 {
                draw_line(
                    cursor_x,
                    self.y + theme.padding,
                    cursor_x,
                    self.y + self.h - theme.padding,
                    1.0,
                    theme.text,
                );
            }
        }
    }

    fn update(&mut self, theme: &Theme) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.focused = self.is_mouse_over();
        }

        if self.focused {
            self.cursor_animation.update();
            if self.cursor_animation.current <= 0.0 {
                self.cursor_animation.set_target(1.0);
            } else if self.cursor_animation.current >= 1.0 {
                self.cursor_animation.set_target(0.0);
            }

            // Handle text input
            if let Some(key) = get_char_pressed() {
                if key.is_ascii() && !key.is_control() {
                    self.text.insert(self.cursor_position, key);
                    self.cursor_position += 1;
                    if let Some(cb) = &mut self.on_change {
                        cb(&self.text);
                    }
                }
            }

            // Handle backspace
            if is_key_pressed(KeyCode::Backspace) && self.cursor_position > 0 {
                self.text.remove(self.cursor_position - 1);
                self.cursor_position -= 1;
                if let Some(cb) = &mut self.on_change {
                    cb(&self.text);
                }
            }

            // Handle arrow keys
            if is_key_pressed(KeyCode::Left) && self.cursor_position > 0 {
                self.cursor_position -= 1;
            }
            if is_key_pressed(KeyCode::Right) && self.cursor_position < self.text.len() {
                self.cursor_position += 1;
            }
        }
    }

    fn get_bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.w, self.h)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Slider UI element
pub struct UiSlider {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub theme: Theme,
    pub dragging: bool,
    pub on_change: Option<Box<dyn FnMut(f32) + Send + Sync>>,
}

impl UiSlider {
    /// Create a new slider
    pub fn new(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        min: f32,
        max: f32,
        initial: f32,
        theme: Theme,
        on_change: Option<Box<dyn FnMut(f32) + Send + Sync>>,
    ) -> Self {
        Self {
            x,
            y,
            w,
            h,
            value: initial,
            min,
            max,
            theme,
            dragging: false,
            on_change,
        }
    }

    pub fn is_mouse_over_handle(&self) -> bool {
        let (mx, my) = mouse_position();
        let handle_x = self.x + (self.value - self.min) / (self.max - self.min) * self.w;
        let handle_size = self.h * 1.5;
        mx >= handle_x - handle_size/2.0 && mx <= handle_x + handle_size/2.0 &&
        my >= self.y - handle_size/2.0 && my <= self.y + handle_size/2.0
    }
}

impl UiElement for UiSlider {
    fn draw(&self, theme: &Theme) {
        // Draw track
        draw_rounded_rectangle(
            self.x,
            self.y - self.h/2.0,
            self.w,
            self.h,
            theme.border_radius,
            theme.secondary,
        );

        // Draw filled portion
        let fill_width = (self.value - self.min) / (self.max - self.min) * self.w;
        draw_rounded_rectangle(
            self.x,
            self.y - self.h/2.0,
            fill_width,
            self.h,
            theme.border_radius,
            theme.accent,
        );

        // Draw handle
        let handle_x = self.x + fill_width;
        let handle_size = self.h * 1.5;
        draw_circle(
            handle_x,
            self.y,
            handle_size/2.0,
            if self.dragging { theme.accent } else { theme.primary },
        );
    }

    fn update(&mut self, _theme: &Theme) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.dragging = self.is_mouse_over_handle();
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.dragging = false;
        }

        if self.dragging {
            let (mx, _) = mouse_position();
            let new_value = ((mx - self.x) / self.w * (self.max - self.min) + self.min)
                .max(self.min)
                .min(self.max);
            
            if new_value != self.value {
                self.value = new_value;
                if let Some(cb) = &mut self.on_change {
                    cb(self.value);
                }
            }
        }
    }

    fn get_bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y - self.h/2.0, self.w, self.h * 2.0)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Checkbox UI element
pub struct UiCheckbox {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub checked: bool,
    pub theme: Theme,
    pub on_change: Option<Box<dyn FnMut(bool) + Send + Sync>>,
    pub animation: Animation,
}

impl UiCheckbox {
    /// Create a new checkbox
    pub fn new(
        x: f32,
        y: f32,
        size: f32,
        checked: bool,
        theme: Theme,
        on_change: Option<Box<dyn FnMut(bool) + Send + Sync>>,
    ) -> Self {
        Self {
            x,
            y,
            size,
            checked,
            theme,
            on_change,
            animation: Animation::new(if checked { 1.0 } else { 0.0 }, 0.2),
        }
    }

    pub fn is_mouse_over(&self) -> bool {
        let (mx, my) = mouse_position();
        mx >= self.x && mx <= self.x + self.size &&
        my >= self.y && my <= self.y + self.size
    }
}

impl UiElement for UiCheckbox {
    fn draw(&self, theme: &Theme) {
        // Draw box
        draw_rounded_rectangle(
            self.x,
            self.y,
            self.size,
            self.size,
            theme.border_radius,
            if self.checked { theme.accent } else { theme.secondary },
        );

        // Draw checkmark
        if self.animation.current > 0.0 {
            let check_size = self.size * 0.6;
            let check_x = self.x + (self.size - check_size) / 2.0;
            let check_y = self.y + (self.size - check_size) / 2.0;
            
            draw_line(
                check_x,
                check_y + check_size * 0.5,
                check_x + check_size * 0.3,
                check_y + check_size * 0.8,
                check_size * 0.15,
                theme.text,
            );
            draw_line(
                check_x + check_size * 0.3,
                check_y + check_size * 0.8,
                check_x + check_size,
                check_y,
                check_size * 0.15,
                theme.text,
            );
        }
    }

    fn update(&mut self, _theme: &Theme) {
        self.animation.set_target(if self.checked { 1.0 } else { 0.0 });
        self.animation.update();

        if is_mouse_button_pressed(MouseButton::Left) && self.is_mouse_over() {
            self.checked = !self.checked;
            if let Some(cb) = &mut self.on_change {
                cb(self.checked);
            }
        }
    }

    fn get_bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.size, self.size)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Panel UI element
pub struct UiPanel {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub theme: Theme,
    pub title: Option<String>,
    pub elements: Vec<Box<dyn UiElement>>,
}

impl UiPanel {
    /// Create a new panel
    pub fn new(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        theme: Theme,
        title: Option<String>,
    ) -> Self {
        Self {
            x,
            y,
            w,
            h,
            theme,
            title,
            elements: Vec::new(),
        }
    }

    /// Add a UI element to the panel
    pub fn add_element(&mut self, element: Box<dyn UiElement>) {
        self.elements.push(element);
    }
}

impl UiElement for UiPanel {
    fn draw(&self, theme: &Theme) {
        // Draw panel background
        draw_rounded_rectangle(
            self.x,
            self.y,
            self.w,
            self.h,
            theme.border_radius,
            theme.background,
        );

        // Draw title if present
        if let Some(title) = &self.title {
            draw_rectangle(
                self.x,
                self.y,
                self.w,
                30.0,
                theme.secondary,
            );
            draw_text_ex(
                title,
                self.x + theme.padding,
                self.y + 20.0,
                TextParams {
                    font_size: 16,
                    color: theme.text,
                    ..Default::default()
                },
            );
        }

        // Draw elements
        for element in &self.elements {
            element.draw(theme);
        }
    }

    fn update(&mut self, theme: &Theme) {
        for element in &mut self.elements {
            element.update(theme);
        }
    }

    fn get_bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.w, self.h)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Progress bar UI element
pub struct UiProgressBar {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub progress: f32,
    pub theme: Theme,
    pub animation: Animation,
}

impl UiProgressBar {
    /// Create a new progress bar
    pub fn new(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        initial_progress: f32,
        theme: Theme,
    ) -> Self {
        Self {
            x,
            y,
            w,
            h,
            progress: initial_progress.max(0.0).min(1.0),
            theme,
            animation: Animation::new(initial_progress, 0.2),
        }
    }

    /// Set the progress value
    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress.max(0.0).min(1.0);
        self.animation.set_target(self.progress);
    }
}

impl UiElement for UiProgressBar {
    fn draw(&self, theme: &Theme) {
        // Draw background
        draw_rounded_rectangle(
            self.x,
            self.y,
            self.w,
            self.h,
            theme.border_radius,
            theme.secondary,
        );

        // Draw progress
        let progress_width = self.w * self.animation.current;
        draw_rounded_rectangle(
            self.x,
            self.y,
            progress_width,
            self.h,
            theme.border_radius,
            theme.accent,
        );
    }

    fn update(&mut self, _theme: &Theme) {
        self.animation.update();
    }

    fn get_bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.w, self.h)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Dropdown menu UI element
pub struct UiDropdown {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub options: Vec<String>,
    pub selected_index: usize,
    pub is_open: bool,
    pub theme: Theme,
    pub font: Font,
    pub font_size: u16,
    pub on_select: Option<Box<dyn Fn(usize)>>,
    pub hover_index: Option<usize>,
}

impl UiDropdown {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        options: Vec<String>,
        theme: Theme,
        font: Font,
        font_size: u16,
        on_select: Option<Box<dyn Fn(usize)>>,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            options,
            selected_index: 0,
            is_open: false,
            theme,
            font,
            font_size,
            on_select,
            hover_index: None,
        }
    }

    pub fn update(&mut self, theme: &Theme) {
        let (mouse_x, mouse_y) = mouse_position();
        let bounds = self.get_bounds();

        // Check if mouse is over the dropdown
        if mouse_x >= bounds.0 && mouse_x <= bounds.0 + bounds.2 &&
           mouse_y >= bounds.1 && mouse_y <= bounds.1 + bounds.3 {
            if is_mouse_button_pressed(MouseButton::Left) {
                self.is_open = !self.is_open;
            }
        } else if self.is_open {
            // Check if mouse is over any option
            let option_height = self.height;
            let total_height = option_height * (self.options.len() as f32);
            
            if mouse_x >= bounds.0 && mouse_x <= bounds.0 + bounds.2 &&
               mouse_y >= bounds.1 + self.height && mouse_y <= bounds.1 + self.height + total_height {
                let option_index = ((mouse_y - (bounds.1 + self.height)) / option_height) as usize;
                if option_index < self.options.len() {
                    self.hover_index = Some(option_index);
                    if is_mouse_button_pressed(MouseButton::Left) {
                        self.selected_index = option_index;
                        self.is_open = false;
                        if let Some(callback) = &self.on_select {
                            callback(option_index);
                        }
                    }
                }
            } else if is_mouse_button_pressed(MouseButton::Left) {
                // Click outside closes the dropdown
                self.is_open = false;
            }
        }
    }

    pub fn draw(&self, theme: &Theme) {
        // Draw main button
        draw_rounded_rectangle(
            self.x,
            self.y,
            self.width,
            self.height,
            theme.border_radius,
            theme.primary,
        );

        // Draw selected text
        let text = &self.options[self.selected_index];
        let text_size = measure_text(text, Some(&self.font), self.font_size, 1.0);
        draw_text_ex(
            text,
            self.x + 10.0,
            self.y + (self.height + text_size.height) / 2.0,
            TextParams {
                font: Some(&self.font),
                font_size: self.font_size,
                color: theme.text,
                ..Default::default()
            },
        );

        // Draw dropdown arrow
        let arrow_size = 8.0;
        let arrow_x = self.x + self.width - 20.0;
        let arrow_y = self.y + (self.height - arrow_size) / 2.0;
        draw_triangle(
            Vec2::new(arrow_x, arrow_y),
            Vec2::new(arrow_x + arrow_size, arrow_y),
            Vec2::new(arrow_x + arrow_size / 2.0, arrow_y + arrow_size),
            theme.text,
        );

        // Draw options if open
        if self.is_open {
            let option_height = self.height;
            for (i, option) in self.options.iter().enumerate() {
                let option_y = self.y + self.height + (i as f32 * option_height);
                
                // Draw option background
                let bg_color = if Some(i) == self.hover_index {
                    theme.accent
                } else {
                    theme.secondary
                };
                
                draw_rounded_rectangle(
                    self.x,
                    option_y,
                    self.width,
                    option_height,
                    theme.border_radius,
                    bg_color,
                );

                // Draw option text
                let text_size = measure_text(option, Some(&self.font), self.font_size, 1.0);
                draw_text_ex(
                    option,
                    self.x + 10.0,
                    option_y + (option_height + text_size.height) / 2.0,
                    TextParams {
                        font: Some(&self.font),
                        font_size: self.font_size,
                        color: theme.text,
                        ..Default::default()
                    },
                );
            }
        }
    }

    fn get_bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}

impl UiElement for UiDropdown {
    fn draw(&self, theme: &Theme) {
        self.draw(theme);
    }

    fn update(&mut self, theme: &Theme) {
        self.update(theme);
    }

    fn get_bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
