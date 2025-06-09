//! UI System for Ruty Game Engine
//!
//! This module provides a comprehensive set of UI components for creating
//! interactive user interfaces in games. The system is built around a theme-based
//! design that ensures consistent styling across all components.
//!
//! # Overview
//!
//! The UI system is composed of several core elements:
//! - [`Theme`]: Defines the color palette and style for all UI components.
//! - [`UiElement`]: Trait implemented by all UI widgets (buttons, sliders, dropdowns, etc).
//! - [`UiManager`]: Manages a collection of UI elements, their z-order, and rendering.
//! - Individual UI widgets: [`UiButton`], [`UiInput`], [`UiSlider`], [`UiCheckbox`], [`UiPanel`], [`UiProgressBar`], [`UiDropdown`], [`UiText`].
//!
//! # Usage Example
//!
//! ```rust
//! use ruty::objects::ui::{Theme, UiButton, UiPanel};
//! let theme = Theme::default();
//! let mut panel = UiPanel::new(20.0, 20.0, 300.0, 500.0, theme.clone(), Some("My Panel".to_string()));
//! let button = UiButton::new(
//!     "Click Me", 40.0, 60.0, 220.0, 40.0, 18, font.clone(), theme.clone(), Some(Box::new(|| println!("Button clicked!"))),
//! );
//! panel.add_element(Box::new(button));
//! ```

use macroquad::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::any::Any;

/// Theme for UI components
///
/// The `Theme` struct defines the color palette and style parameters used by all UI elements in the system.
/// It provides a consistent look and feel across the UI by centralizing color and style configuration.
///
/// # Fields
/// - `primary`: Main color for prominent UI elements (e.g., buttons).
/// - `secondary`: Secondary color for less prominent elements (e.g., backgrounds).
/// - `accent`: Accent color for highlights and interactive states.
/// - `background`: Background color for panels and containers.
/// - `text`: Color for all text.
/// - `error`: Color for error states.
/// - `success`: Color for success states.
/// - `border_radius`: Corner radius for rounded rectangles.
/// - `padding`: Default padding for UI elements.
/// - `animation_speed`: Default speed for UI animations.
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
    /// Returns a default theme with a modern, accessible color palette and sensible defaults for all style parameters.
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
///
/// The `Animation` struct provides a simple linear interpolation mechanism for animating UI properties (e.g., hover, press, progress).
/// It tracks a `current` value, a `target` value, and a `speed` factor for smooth transitions.
///
/// # Fields
/// - `current`: The current value of the animation (0.0 to 1.0 typical).
/// - `target`: The value the animation is moving toward.
/// - `speed`: The interpolation speed (0.0 = no movement, 1.0 = instant).
#[derive(Clone)]
pub struct Animation {
    pub current: f32,
    pub target: f32,
    pub speed: f32,
}

impl Animation {
    /// Create a new animation with an initial value and speed.
    ///
    /// # Arguments
    /// * `initial` - The starting value for the animation.
    /// * `speed` - The interpolation speed (0.0 to 1.0).
    pub fn new(initial: f32, speed: f32) -> Self {
        Self {
            current: initial,
            target: initial,
            speed,
        }
    }

    /// Update the animation by interpolating the current value toward the target value.
    ///
    /// This should be called every frame to animate UI transitions.
    pub fn update(&mut self) {
        self.current += (self.target - self.current) * self.speed;
    }

    /// Set a new target value for the animation.
    ///
    /// # Arguments
    /// * `target` - The value to animate toward.
    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    /// Get the current value of the animation.
    pub fn get_current(&self) -> f32 {
        self.current
    }
}

/// UI element styles
///
/// The `UiStyle` struct encapsulates all visual style parameters for a UI element, allowing for fine-grained customization.
/// Styles can be defined globally or per-element, and are used to override or extend the base [`Theme`] for specific widgets.
///
/// # Fields
/// - `background_color`: The background color of the element.
/// - `hover_color`: The color used when the element is hovered.
/// - `active_color`: The color used when the element is pressed/active.
/// - `text_color`: The color of the element's text.
/// - `border_color`: The color of the element's border.
/// - `border_width`: The width of the border in pixels.
/// - `corner_radius`: The radius for rounded corners.
/// - `padding`: The padding inside the element.
/// - `font_size`: The font size for text.
/// - `shadow_color`: The color of the element's drop shadow.
/// - `shadow_offset`: The offset of the shadow.
/// - `shadow_blur`: The blur radius for the shadow.
/// - `transition_speed`: The speed of style transitions (e.g., hover, press).
#[derive(Clone, Copy)]
pub struct UiStyle {
    pub background_color: Color,
    pub hover_color: Color,
    pub active_color: Color,
    pub text_color: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub corner_radius: f32,
    pub padding: f32,
    pub font_size: f32,
    pub shadow_color: Color,
    pub shadow_offset: Vec2,
    pub shadow_blur: f32,
    pub transition_speed: f32,
}

impl Default for UiStyle {
    /// Returns a default style suitable for most UI elements, with a modern dark theme and subtle transitions.
    fn default() -> Self {
        Self {
            background_color: Color::from_rgba(40, 40, 40, 255),
            hover_color: Color::from_rgba(60, 60, 60, 255),
            active_color: Color::from_rgba(80, 80, 80, 255),
            text_color: WHITE,
            border_color: Color::from_rgba(100, 100, 100, 255),
            border_width: 2.0,
            corner_radius: 8.0,
            padding: 10.0,
            font_size: 20.0,
            shadow_color: Color::from_rgba(0, 0, 0, 100),
            shadow_offset: Vec2::new(2.0, 2.0),
            shadow_blur: 4.0,
            transition_speed: 0.2,
        }
    }
}

/// UI element states
///
/// The `UiState` enum represents the interaction state of a UI element, which can affect its appearance and behavior.
///
/// # Variants
/// - `Normal`: The default, idle state.
/// - `Hover`: The element is being hovered by the mouse.
/// - `Active`: The element is being pressed or is active.
/// - `Disabled`: The element is disabled and cannot be interacted with.
#[derive(Clone, Copy, PartialEq)]
pub enum UiState {
    Normal,
    Hover,
    Active,
    Disabled,
}

/// Trait for all UI elements in the system.
///
/// The `UiElement` trait defines the required interface for any UI widget, enabling polymorphic management and rendering.
/// All UI components (buttons, sliders, dropdowns, etc.) must implement this trait.
///
/// # Required Methods
/// - `draw(&self, theme: &Theme)`: Draw the element using the provided theme.
/// - `update(&mut self, theme: &Theme, manager: Option<&mut UiManager>)`: Update the element's state, optionally interacting with the UI manager.
/// - `get_bounds(&self) -> (f32, f32, f32, f32)`: Return the bounding box of the element.
/// - `contains_point(&self, point: Vec2) -> bool`: Returns true if the point is inside the element's bounds (default implementation provided).
/// - `as_any(&self) -> &dyn Any`: For downcasting to concrete types.
/// - `as_any_mut(&mut self) -> &mut dyn Any`: For downcasting to concrete types (mutable).
pub trait UiElement {
    /// Draw the UI element
    fn draw(&self, theme: &Theme);
    
    /// Update the UI element's state
    fn update(&mut self, theme: &Theme, manager: Option<&mut UiManager>);
    
    /// Get the bounds of the UI element
    fn get_bounds(&self) -> (f32, f32, f32, f32);
    
    /// Check if a point is within the element's bounds
    fn contains_point(&self, point: Vec2) -> bool {
        let (x, y, w, h) = self.get_bounds();
        point.x >= x && point.x <= x + w && point.y >= y && point.y <= y + h
    }
    
    /// Get a reference to the element as Any
    fn as_any(&self) -> &dyn Any;
    
    /// Get a mutable reference to the element as Any
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Helper function to draw rounded rectangles
///
/// Draws a rectangle with rounded corners using a combination of rectangles and circles.
/// This is used throughout the UI system for consistent, modern appearance.
///
/// # Arguments
/// - `x`, `y`: Top-left corner of the rectangle.
/// - `width`, `height`: Size of the rectangle.
/// - `radius`: Corner radius.
/// - `color`: Fill color.
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

/// Text UI element for displaying static or animated text.
///
/// The `UiText` struct represents a single line of text that can be drawn anywhere in the UI.
/// It supports animation, color, font, and alignment customization.
///
/// # Fields
/// - `text`: The string to display.
/// - `x`, `y`: Position of the text anchor (affected by alignment).
/// - `font_size`: Size of the text in pixels.
/// - `color`: Color of the text.
/// - `font`: Font used for rendering.
/// - `animation`: Animation for fade-in/out or other effects.
/// - `alignment`: Horizontal alignment of the text.
pub struct UiText {
    /// The string to display
    pub text: String,
    /// Position of the text anchor (x-coordinate)
    pub x: f32,
    /// Position of the text anchor (y-coordinate)
    pub y: f32,
    /// Size of the text in pixels
    pub font_size: u16,
    /// Color of the text
    pub color: Color,
    /// Font used for rendering
    pub font: Font,
    /// Animation for fade-in/out or other effects
    pub animation: Animation,
    /// Horizontal alignment of the text
    pub alignment: TextAlignment,
}

/// Text alignment options for UI text elements.
///
/// The `TextAlignment` enum specifies how text should be horizontally aligned within its bounding box.
///
/// # Variants
/// - `Left`: Align text to the left edge.
/// - `Center`: Center text horizontally.
/// - `Right`: Align text to the right edge.
#[derive(Clone, Copy)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

impl UiText {
    /// Create a new text element with the given content, position, size, color, and font.
    ///
    /// # Arguments
    /// - `text`: The string to display.
    /// - `x`, `y`: Position of the text anchor.
    /// - `font_size`: Size of the text in pixels.
    /// - `color`: Color of the text.
    /// - `font`: Font used for rendering.
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

    /// Set the text alignment (left, center, right).
    ///
    /// # Arguments
    /// - `alignment`: The desired text alignment.
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

    fn update(&mut self, _theme: &Theme, _manager: Option<&mut UiManager>) {
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

/// Button UI element for clickable actions.
///
/// The `UiButton` struct represents a clickable button with text, supporting hover and press animations, disabled state, and callbacks.
///
/// # Fields
/// - `text`: The label displayed on the button.
/// - `x`, `y`, `w`, `h`: Position and size of the button.
/// - `font_size`: Size of the button text.
/// - `font`: Font used for rendering.
/// - `theme`: Theme for styling.
/// - `on_click`: Optional callback invoked when the button is clicked.
/// - `hover_animation`: Animation for the hover effect.
/// - `press_animation`: Animation for the press effect.
/// - `disabled`: Whether the button is disabled.
pub struct UiButton {
    /// The label displayed on the button
    pub text: String,
    /// The x-coordinate of the button's top-left corner
    pub x: f32,
    /// The y-coordinate of the button's top-left corner
    pub y: f32,
    /// The width of the button
    pub w: f32,
    /// The height of the button
    pub h: f32,
    /// The font size of the button text
    pub font_size: u16,
    /// The font used for rendering the button text
    pub font: Font,
    /// The theme used for styling the button
    pub theme: Theme,
    /// Optional callback that is invoked when the button is clicked
    pub on_click: Option<Box<dyn FnMut() + Send + Sync>>,
    /// Animation for the hover effect
    pub hover_animation: Animation,
    /// Animation for the press effect
    pub press_animation: Animation,
    /// Indicates whether the button is disabled
    pub disabled: bool,
}

impl UiButton {
    /// Create a new button with the given label, position, size, font, theme, and click callback.
    ///
    /// # Arguments
    /// - `text`: The label to display.
    /// - `x`, `y`, `w`, `h`: Position and size.
    /// - `font_size`: Text size.
    /// - `font`: Font for rendering.
    /// - `theme`: Theme for styling.
    /// - `on_click`: Optional callback for click events.
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

    /// Returns true if the mouse is currently over the button.
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

    fn update(&mut self, _theme: &Theme, _manager: Option<&mut UiManager>) {
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

/// Input field UI element for text entry.
///
/// The `UiInput` struct provides a single-line text input box with support for focus, cursor, placeholder, and callbacks on change.
///
/// # Fields
/// - `text`: The current text in the input field.
/// - `x`, `y`, `w`, `h`: Position and size of the input box.
/// - `font_size`: Size of the input text.
/// - `font`: Font used for rendering.
/// - `theme`: Theme for styling.
/// - `placeholder`: Placeholder text when input is empty.
/// - `focused`: Whether the input is currently focused.
/// - `cursor_position`: Current position of the cursor in the text.
/// - `cursor_animation`: Animation for cursor blinking.
/// - `on_change`: Optional callback invoked when the text changes.
pub struct UiInput {
    /// The current text in the input field
    pub text: String,
    /// The x-coordinate of the input field's top-left corner
    pub x: f32,
    /// The y-coordinate of the input field's top-left corner
    pub y: f32,
    /// The width of the input field
    pub w: f32,
    /// The height of the input field
    pub h: f32,
    /// The font size of the input text
    pub font_size: u16,
    /// The font used for rendering the input text
    pub font: Font,
    /// The theme used for styling the input field
    pub theme: Theme,
    /// The placeholder text displayed when the input is empty
    pub placeholder: String,
    /// Indicates whether the input field is focused
    pub focused: bool,
    /// The current position of the cursor in the text
    pub cursor_position: usize,
    /// Animation for the cursor blinking
    pub cursor_animation: Animation,
    /// Optional callback that is invoked when the text changes
    pub on_change: Option<Box<dyn FnMut(&str) + Send + Sync>>,
}

impl UiInput {
    /// Create a new input field with the given position, size, font, theme, and placeholder.
    ///
    /// # Arguments
    /// - `x`, `y`, `w`, `h`: Position and size.
    /// - `font_size`: Text size.
    /// - `font`: Font for rendering.
    /// - `theme`: Theme for styling.
    /// - `placeholder`: Placeholder text.
    /// - `on_change`: Optional callback for text changes.
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

    /// Returns true if the mouse is currently over the input field.
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

    fn update(&mut self, theme: &Theme, _manager: Option<&mut UiManager>) {
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

/// Slider UI element for selecting a value in a range.
///
/// The `UiSlider` struct provides a horizontal slider with a draggable handle, supporting value change callbacks and animation.
///
/// # Fields
/// - `x`, `y`, `w`, `h`: Position and size of the slider track.
/// - `value`: Current value of the slider.
/// - `min`, `max`: Minimum and maximum values.
/// - `theme`: Theme for styling.
/// - `dragging`: Whether the handle is currently being dragged.
/// - `on_change`: Optional callback invoked when the value changes.
pub struct UiSlider {
    /// The x-coordinate of the slider's top-left corner
    pub x: f32,
    /// The y-coordinate of the slider's top-left corner
    pub y: f32,
    /// The width of the slider track
    pub w: f32,
    /// The height of the slider track
    pub h: f32,
    /// The current value of the slider
    pub value: f32,
    /// The minimum value of the slider
    pub min: f32,
    /// The maximum value of the slider
    pub max: f32,
    /// The theme used for styling the slider
    pub theme: Theme,
    /// Indicates whether the slider handle is being dragged
    pub dragging: bool,
    /// Optional callback that is invoked when the slider value changes
    pub on_change: Option<Box<dyn FnMut(f32) + Send + Sync>>,
}

impl UiSlider {
    /// Create a new slider with the given position, size, range, initial value, theme, and change callback.
    ///
    /// # Arguments
    /// - `x`, `y`, `w`, `h`: Position and size.
    /// - `min`, `max`: Value range.
    /// - `initial`: Initial value.
    /// - `theme`: Theme for styling.
    /// - `on_change`: Optional callback for value changes.
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

    /// Returns true if the mouse is currently over the slider handle.
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

    fn update(&mut self, _theme: &Theme, _manager: Option<&mut UiManager>) {
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

/// Checkbox UI element for boolean toggles.
///
/// The `UiCheckbox` struct provides a clickable checkbox with animation and callback support.
///
/// # Fields
/// - `x`, `y`: Position of the checkbox.
/// - `size`: Size of the checkbox (width and height).
/// - `checked`: Whether the checkbox is currently checked.
/// - `theme`: Theme for styling.
/// - `on_change`: Optional callback invoked when the checked state changes.
/// - `animation`: Animation for checkmark appearance.
pub struct UiCheckbox {
    /// The x-coordinate of the checkbox's top-left corner
    pub x: f32,
    /// The y-coordinate of the checkbox's top-left corner
    pub y: f32,
    /// The size of the checkbox (width and height)
    pub size: f32,
    /// Indicates whether the checkbox is checked
    pub checked: bool,
    /// The theme used for styling the checkbox
    pub theme: Theme,
    /// Optional callback that is invoked when the checkbox state changes
    pub on_change: Option<Box<dyn FnMut(bool) + Send + Sync>>,
    /// Animation for the checkmark appearance
    pub animation: Animation,
}

impl UiCheckbox {
    /// Create a new checkbox with the given position, size, checked state, theme, and change callback.
    ///
    /// # Arguments
    /// - `x`, `y`: Position.
    /// - `size`: Size of the checkbox.
    /// - `checked`: Initial checked state.
    /// - `theme`: Theme for styling.
    /// - `on_change`: Optional callback for state changes.
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

    /// Returns true if the mouse is currently over the checkbox.
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

    fn update(&mut self, _theme: &Theme, _manager: Option<&mut UiManager>) {
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

/// Panel container for grouping and managing multiple UI elements.
///
/// `UiPanel` provides a rectangular region with an optional title bar and a list of child UI elements.
/// It is commonly used to organize related UI controls (buttons, sliders, dropdowns, etc.) into a single visual group.
///
/// # Drawing and Stacking Logic
///
/// The `draw` method of `UiPanel` implements a two-phase rendering strategy to ensure correct visual stacking of dropdown menus:
///
/// 1. **Non-Dropdown Elements First:**
///    All child elements that are *not* open dropdowns are drawn first, in their order of addition. This includes buttons, sliders, text, and closed dropdowns.
///
/// 2. **Open Dropdowns Last (On Top):**
///    Any child element that is an open dropdown (`UiDropdown` with `is_open == true`) is drawn *after* all other children. This guarantees that dropdown menus always appear visually above all other panel contents, regardless of their position in the `elements` vector or their nesting within other panels.
///
/// This approach solves the classic UI stacking problem where dropdown menus could be obscured by other UI elements due to z-order or draw order. By always rendering open dropdowns last, the panel ensures that dropdown menus are never hidden by other controls, even if deeply nested.
///
/// # Example
///
/// ```rust
/// let mut panel = UiPanel::new(10.0, 10.0, 300.0, 400.0, theme.clone(), Some("Settings".to_string()));
/// panel.add_element(Box::new(UiButton::new(...)));
/// panel.add_element(Box::new(UiDropdown::new(...))); // Dropdown will always draw on top if open
/// ```
///
/// # Usage Notes
/// - You can add any type implementing `UiElement` to a panel.
/// - Open dropdowns are always drawn above all other children, regardless of their order in the `elements` vector.
/// - The panel itself does not manage input focus or event routing; this is handled by the UI manager or parent container.
/// - The panel's background and optional title bar are drawn before any children.
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
    /// Creates a new `UiPanel` with the given position, size, theme, and optional title.
    ///
    /// # Arguments
    /// * `x` - The x-coordinate of the panel's top-left corner.
    /// * `y` - The y-coordinate of the panel's top-left corner.
    /// * `w` - The width of the panel.
    /// * `h` - The height of the panel.
    /// * `theme` - The theme to use for styling the panel and its children.
    /// * `title` - An optional title string to display in the panel's title bar.
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

    /// Adds a child UI element to the panel.
    ///
    /// The element will be managed and drawn by the panel. If the element is a dropdown,
    /// it will be drawn above all other children when open.
    ///
    /// # Arguments
    /// * `element` - The UI element to add (boxed trait object).
    pub fn add_element(&mut self, element: Box<dyn UiElement>) {
        self.elements.push(element);
    }
}

impl UiElement for UiPanel {
    /// Draws the panel and its children, ensuring correct stacking of dropdowns.
    ///
    /// The draw order is:
    /// 1. Panel background and title bar (if present)
    /// 2. All non-dropdown children (in order)
    /// 3. All open dropdowns (in order, on top of everything else)
    ///
    /// This guarantees that dropdown menus are never visually obscured by other panel contents.
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

        // Draw elements: non-dropdowns first, then open dropdowns last
        let mut open_dropdowns = Vec::new();
        let mut others = Vec::new();
        for element in &self.elements {
            if let Some(dropdown) = element.as_any().downcast_ref::<UiDropdown>() {
                if dropdown.is_open {
                    open_dropdowns.push(element);
                    continue;
                }
            }
            others.push(element);
        }
        for element in others {
            element.draw(theme);
        }
        for element in open_dropdowns {
            element.draw(theme);
        }
    }

    /// Updates all child elements in the panel.
    ///
    /// This method simply forwards the update call to each child element.
    ///
    /// # Arguments
    /// * `theme` - The current UI theme.
    /// * `manager` - Optional mutable reference to the UI manager for context.
    fn update(&mut self, _theme: &Theme, mut manager: Option<&mut UiManager>) {
        for element in &mut self.elements {
            element.update(_theme, manager.as_deref_mut());
        }
    }

    /// Returns the bounds of the panel as (x, y, w, h).
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

/// Progress bar UI element for indicating progress.
///
/// The `UiProgressBar` struct provides a horizontal bar that visually represents a progress value between 0.0 and 1.0.
///
/// # Fields
/// - `x`, `y`, `w`, `h`: Position and size of the progress bar.
/// - `progress`: Current progress value (0.0 to 1.0).
/// - `theme`: Theme for styling.
/// - `animation`: Animation for smooth progress transitions.
pub struct UiProgressBar {
    /// The x-coordinate of the progress bar's top-left corner
    pub x: f32,
    /// The y-coordinate of the progress bar's top-left corner
    pub y: f32,
    /// The width of the progress bar
    pub w: f32,
    /// The height of the progress bar
    pub h: f32,
    /// The current progress value (0.0 to 1.0)
    pub progress: f32,
    /// The theme used for styling the progress bar
    pub theme: Theme,
    /// Animation for smooth progress transitions
    pub animation: Animation,
}

impl UiProgressBar {
    /// Create a new progress bar with the given position, size, initial progress, and theme.
    ///
    /// # Arguments
    /// - `x`, `y`, `w`, `h`: Position and size.
    /// - `initial_progress`: Initial progress value (clamped between 0.0 and 1.0).
    /// - `theme`: Theme for styling.
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

    /// Set the progress value (clamped between 0.0 and 1.0).
    ///
    /// # Arguments
    /// - `progress`: New progress value.
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

    fn update(&mut self, _theme: &Theme, _manager: Option<&mut UiManager>) {
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

/// Dropdown menu UI element for selecting from a list of options.
///
/// The `UiDropdown` struct provides a clickable dropdown menu with animated open/close, option highlighting, and selection callbacks.
///
/// # Fields
/// - `x`, `y`: Position of the dropdown button.
/// - `width`, `height`: Size of the dropdown button.
/// - `options`: List of selectable string options.
/// - `selected_index`: Index of the currently selected option.
/// - `is_open`: Whether the dropdown menu is currently open.
/// - `theme`: Theme for styling.
/// - `font`: Font used for rendering options.
/// - `font_size`: Size of the option text.
/// - `on_select`: Optional callback invoked when an option is selected.
/// - `hover_index`: Index of the option currently hovered by the mouse.
/// - `hover_color`: Color for hovered options.
/// - `active_color`: Color for active/pressed state.
/// - `option_height`: Height of each option row.
/// - `option_padding`: Padding inside each option row.
/// - `animation_speed`: Speed of open/close and hover animations.
/// - `hover_animation`: Animation for hover effect.
/// - `press_animation`: Animation for press effect.
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
    // New customization options
    pub hover_color: Color,
    pub active_color: Color,
    pub option_height: f32,
    pub option_padding: f32,
    pub animation_speed: f32,
    pub hover_animation: Animation,
    pub press_animation: Animation,
}

impl UiDropdown {
    /// Create a new dropdown menu with the given position, size, options, theme, font, and selection callback.
    ///
    /// # Arguments
    /// - `x`, `y`: Position of the dropdown button.
    /// - `width`, `height`: Size of the dropdown button.
    /// - `options`: List of selectable options.
    /// - `theme`: Theme for styling.
    /// - `font`: Font for rendering.
    /// - `font_size`: Size of the option text.
    /// - `on_select`: Optional callback for selection events.
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
            hover_color: Color::from_rgba(60, 60, 60, 255),
            active_color: Color::from_rgba(80, 80, 80, 255),
            option_height: height,
            option_padding: 5.0,
            animation_speed: 0.2,
            hover_animation: Animation::new(0.0, 0.2),
            press_animation: Animation::new(0.0, 0.3),
        }
    }

    /// Set the color used for hovered options.
    pub fn set_hover_color(&mut self, color: Color) {
        self.hover_color = color;
    }

    /// Set the color used for the active/pressed state.
    pub fn set_active_color(&mut self, color: Color) {
        self.active_color = color;
    }

    /// Set the height of each option row.
    pub fn set_option_height(&mut self, height: f32) {
        self.option_height = height;
    }

    /// Set the padding inside each option row.
    pub fn set_option_padding(&mut self, padding: f32) {
        self.option_padding = padding;
    }

    /// Set the animation speed for open/close and hover transitions.
    pub fn set_animation_speed(&mut self, speed: f32) {
        self.animation_speed = speed;
        self.hover_animation.speed = speed;
        self.press_animation.speed = speed;
    }
}

impl UiElement for UiDropdown {
    fn draw(&self, theme: &Theme) {
        let hover = self.hover_animation.current;
        let press = self.press_animation.current;
        
        let button_color = if press > 0.0 {
            Color::new(
                self.active_color.r + (theme.primary.r - self.active_color.r) * press,
                self.active_color.g + (theme.primary.g - self.active_color.g) * press,
                self.active_color.b + (theme.primary.b - self.active_color.b) * press,
                self.active_color.a,
            )
        } else {
            Color::new(
                theme.primary.r + (self.hover_color.r - theme.primary.r) * hover,
                theme.primary.g + (self.hover_color.g - theme.primary.g) * hover,
                theme.primary.b + (self.hover_color.b - theme.primary.b) * hover,
                theme.primary.a,
            )
        };

        // Draw button
        draw_rounded_rectangle(
            self.x,
            self.y,
            self.width,
            self.height,
            theme.border_radius,
            button_color,
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
        
        let points = if self.is_open {
            [
                Vec2::new(arrow_x, arrow_y + arrow_size),
                Vec2::new(arrow_x + arrow_size, arrow_y + arrow_size),
                Vec2::new(arrow_x + arrow_size/2.0, arrow_y),
            ]
        } else {
            [
                Vec2::new(arrow_x, arrow_y),
                Vec2::new(arrow_x + arrow_size, arrow_y),
                Vec2::new(arrow_x + arrow_size/2.0, arrow_y + arrow_size),
            ]
        };
        
        draw_triangle(points[0], points[1], points[2], theme.text);

        // If open, draw options
        if self.is_open {
            // Draw semi-transparent overlay
            draw_rectangle(
                0.0, 0.0,
                screen_width(),
                screen_height(),
                Color::new(0.0, 0.0, 0.0, 0.3)
            );
            
            // Draw options background
            let total_height = self.option_height * (self.options.len() as f32);
            draw_rounded_rectangle(
                self.x,
                self.y + self.height,
                self.width,
                total_height,
                theme.border_radius,
                theme.background,
            );
            
            // Draw each option
            for (i, option) in self.options.iter().enumerate() {
                let option_y = self.y + self.height + (i as f32 * self.option_height);
                
                // Draw option background with hover effect
                let bg_color = if Some(i) == self.hover_index {
                    self.hover_color
                } else {
                    theme.secondary
                };
                
                draw_rounded_rectangle(
                    self.x,
                    option_y,
                    self.width,
                    self.option_height,
                    theme.border_radius,
                    bg_color,
                );

                // Draw option text
                let text_size = measure_text(option, Some(&self.font), self.font_size, 1.0);
                draw_text_ex(
                    option,
                    self.x + self.option_padding,
                    option_y + (self.option_height + text_size.height) / 2.0,
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

    fn update(&mut self, theme: &Theme, manager: Option<&mut UiManager>) {
        let (mouse_x, mouse_y) = mouse_position();
        
        // Check if mouse is over the main button
        let is_over_button = mouse_x >= self.x && mouse_x <= self.x + self.width &&
                            mouse_y >= self.y && mouse_y <= self.y + self.height;

       // println!("Mouse position: ({}, {}), is_over_button: {}", mouse_x, mouse_y, is_over_button);

        // Update hover animation
        self.hover_animation.set_target(if is_over_button { 1.0 } else { 0.0 });
        self.hover_animation.update();

        // Handle button click
        if is_over_button && is_mouse_button_pressed(MouseButton::Left) {
            println!("Dropdown button clicked, current is_open: {}", self.is_open);
            self.is_open = !self.is_open;
            println!("New is_open state: {}", self.is_open);
            
            if self.is_open {
                // Bring to front when opened
                if let Some(manager) = manager {
                    if let Some(index) = manager.get_element_index(self) {
                        manager.bring_to_front(index);
                    }
                }
            }
            return; // Prevent immediate closing
        }

        // Handle options if dropdown is open
        if self.is_open {
            let total_height = self.option_height * (self.options.len() as f32);
            
            // Check if mouse is over the entire dropdown area (button + options)
            let is_over_dropdown = mouse_x >= self.x && mouse_x <= self.x + self.width &&
                                 mouse_y >= self.y && mouse_y <= self.y + self.height + total_height;

            println!("is_over_dropdown: {}, total_height: {}", is_over_dropdown, total_height);

            if is_over_dropdown {
                // Check if mouse is over options area
                let is_over_options = mouse_y > self.y + self.height;
                
                if is_over_options {
                    let option_index = ((mouse_y - (self.y + self.height)) / self.option_height) as usize;
                    if option_index < self.options.len() {
                        self.hover_index = Some(option_index);
                        if is_mouse_button_pressed(MouseButton::Left) {
                            println!("Option selected: {}", option_index);
                            self.selected_index = option_index;
                            self.is_open = false;
                            if let Some(callback) = &self.on_select {
                                callback(option_index);
                            }
                        }
                    }
                }
            } else if is_mouse_button_pressed(MouseButton::Left) {
                // Click outside closes the dropdown
                println!("Click outside, closing dropdown");
                self.is_open = false;
            }
        }
    }

    fn get_bounds(&self) -> (f32, f32, f32, f32) {
        let total_height = if self.is_open {
            self.height + (self.option_height * (self.options.len() as f32))
        } else {
            self.height
        };
        (self.x, self.y, self.width, total_height)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// UI Manager to handle multiple UI elements, z-ordering, and drawing.
///
/// The `UiManager` struct is responsible for managing a collection of UI elements, their stacking order (z-index), and rendering.
/// It provides methods for adding, updating, and drawing elements, as well as for managing styles and bringing elements to the front.
///
/// # Fields
/// - `elements`: Vector of (z-index, element) pairs, sorted by stacking order.
/// - `next_z`: The next available z-index for new elements.
/// - `styles`: Named styles for customizing UI appearance.
pub struct UiManager {
    elements: Vec<(usize, Box<dyn UiElement>)>, // (z, element)
    next_z: usize,
    styles: HashMap<String, UiStyle>,
}

impl UiManager {
    /// Create a new, empty UI manager.
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            next_z: 0,
            styles: HashMap::new(),
        }
    }

    /// Add a UI element to the manager, always on top (highest z-index).
    /// Returns the z-index assigned to the element.
    pub fn add_element(&mut self, element: Box<dyn UiElement>) -> usize {
        let z = self.next_z;
        self.next_z += 1;
        self.elements.push((z, element));
        z
    }

    /// Add a named style to the manager for later retrieval.
    pub fn add_style(&mut self, name: &str, style: UiStyle) {
        self.styles.insert(name.to_string(), style);
    }

    /// Retrieve a style by name, if it exists.
    pub fn get_style(&self, name: &str) -> Option<&UiStyle> {
        self.styles.get(name)
    }

    /// Bring the element with the given z-index to the front (top of stack).
    pub fn bring_to_front(&mut self, z: usize) {
        if let Some(pos) = self.elements.iter().position(|(ez, _)| *ez == z) {
            let (z, elem) = self.elements.remove(pos);
            self.elements.push((z, elem));
        }
    }

    /// Get the index of a given element in the manager, if present.
    pub fn get_element_index(&self, element: &dyn UiElement) -> Option<usize> {
        self.elements.iter().position(|(_, e)| std::ptr::eq(e.as_ref(), element))
    }

    /// Get a reference to an element by z-index, if present.
    pub fn get_element(&self, z: usize) -> Option<&Box<dyn UiElement>> {
        self.elements.iter().find(|(ez, _)| *ez == z).map(|(_, e)| e)
    }

    /// Get a mutable reference to an element by z-index, if present.
    pub fn get_element_mut(&mut self, z: usize) -> Option<&mut Box<dyn UiElement>> {
        self.elements.iter_mut().find(|(ez, _)| *ez == z).map(|(_, e)| e)
    }

    /// Update all UI elements, and bring open dropdowns to the front.
    ///
    /// This ensures that dropdown menus are always visually on top, regardless of their original stacking order.
    pub fn update(&mut self) {
        let theme = Theme::default();
        // Update all elements
        for (_, element) in self.elements.iter_mut() {
            element.update(&theme, None);
        }
        // Bring all open dropdowns to front (preserving their order)
        let mut open_dropdowns = Vec::new();
        for (z, element) in self.elements.iter() {
            if let Some(dropdown) = element.as_any().downcast_ref::<UiDropdown>() {
                if dropdown.is_open {
                    open_dropdowns.push(*z);
                }
            }
        }
        for z in open_dropdowns {
            self.bring_to_front(z);
        }
    }

    /// Draw all UI elements in z-order, with open dropdowns always drawn last (on top).
    ///
    /// This guarantees that dropdown menus are never visually obscured by other UI elements.
    pub fn draw(&self) {
        let theme = Theme::default();
        // Always draw in z-order (lowest to highest), but open dropdowns last
        let mut sorted = self.elements.iter().collect::<Vec<_>>();
        // Separate open dropdowns and others
        let mut open_dropdowns = Vec::new();
        let mut others = Vec::new();
        for (z, element) in sorted {
            if let Some(dropdown) = element.as_any().downcast_ref::<UiDropdown>() {
                if dropdown.is_open {
                    open_dropdowns.push((z, element));
                    continue;
                }
            }
            others.push((z, element));
        }
        // Draw all non-dropdowns first
        others.sort_by_key(|(z, _)| **z);
        for (_, element) in others {
            element.draw(&theme);
        }
        // Draw all open dropdowns last (on top)
        open_dropdowns.sort_by_key(|(z, _)| **z);
        for (_, element) in open_dropdowns {
            element.draw(&theme);
        }
    }
}
