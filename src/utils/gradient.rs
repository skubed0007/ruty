use macroquad::prelude::*;

/// Represents a gradient with multiple color stops
#[derive(Clone)]
pub struct Gradient {
    stops: Vec<(f32, Color)>,
}

impl Gradient {
    /// Create a new gradient with two colors
    pub fn new(start: Color, end: Color) -> Self {
        Self {
            stops: vec![(0.0, start), (1.0, end)],
        }
    }

    /// Create a new gradient with multiple color stops
    pub fn with_stops(stops: Vec<(f32, Color)>) -> Self {
        let mut sorted_stops = stops;
        sorted_stops.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        Self { stops: sorted_stops }
    }

    /// Get the color at a specific position (0.0 to 1.0)
    pub fn get_color(&self, position: f32) -> Color {
        let position = position.clamp(0.0, 1.0);
        
        // Find the surrounding stops
        let mut lower_stop = &self.stops[0];
        let mut upper_stop = &self.stops[self.stops.len() - 1];
        
        for i in 0..self.stops.len() - 1 {
            if position >= self.stops[i].0 && position <= self.stops[i + 1].0 {
                lower_stop = &self.stops[i];
                upper_stop = &self.stops[i + 1];
                break;
            }
        }

        // Interpolate between the stops
        let t = (position - lower_stop.0) / (upper_stop.0 - lower_stop.0);
        Color::new(
            lower_stop.1.r + (upper_stop.1.r - lower_stop.1.r) * t,
            lower_stop.1.g + (upper_stop.1.g - lower_stop.1.g) * t,
            lower_stop.1.b + (upper_stop.1.b - lower_stop.1.b) * t,
            lower_stop.1.a + (upper_stop.1.a - lower_stop.1.a) * t,
        )
    }

    /// Draw a horizontal gradient
    pub fn draw_horizontal(&self, x: f32, y: f32, width: f32, height: f32) {
        for i in 0..width as i32 {
            let t = i as f32 / width;
            let color = self.get_color(t);
            draw_line(x + i as f32, y, x + i as f32, y + height, 1.0, color);
        }
    }

    /// Draw a vertical gradient
    pub fn draw_vertical(&self, x: f32, y: f32, width: f32, height: f32) {
        for i in 0..height as i32 {
            let t = i as f32 / height;
            let color = self.get_color(t);
            draw_line(x, y + i as f32, x + width, y + i as f32, 1.0, color);
        }
    }

    /// Draw a radial gradient
    pub fn draw_radial(&self, center_x: f32, center_y: f32, radius: f32) {
        let steps = (radius * 2.0) as i32;
        for i in 0..steps {
            let t = i as f32 / steps as f32;
            let color = self.get_color(t);
            let current_radius = radius * t;
            draw_circle_lines(center_x, center_y, current_radius, 1.0, color);
        }
    }
}

/// Predefined gradients
impl Gradient {
    /// Create a sunset gradient
    pub fn sunset() -> Self {
        Self::with_stops(vec![
            (0.0, Color::from_rgba(255, 200, 100, 255)),  // Orange
            (0.5, Color::from_rgba(255, 100, 100, 255)),  // Red
            (1.0, Color::from_rgba(100, 50, 150, 255)),   // Purple
        ])
    }

    /// Create an ocean gradient
    pub fn ocean() -> Self {
        Self::with_stops(vec![
            (0.0, Color::from_rgba(0, 100, 200, 255)),    // Light blue
            (0.5, Color::from_rgba(0, 50, 150, 255)),     // Medium blue
            (1.0, Color::from_rgba(0, 0, 100, 255)),      // Dark blue
        ])
    }

    /// Create a forest gradient
    pub fn forest() -> Self {
        Self::with_stops(vec![
            (0.0, Color::from_rgba(100, 200, 100, 255)),  // Light green
            (0.5, Color::from_rgba(50, 150, 50, 255)),    // Medium green
            (1.0, Color::from_rgba(0, 100, 0, 255)),      // Dark green
        ])
    }

    /// Create a fire gradient
    pub fn fire() -> Self {
        Self::with_stops(vec![
            (0.0, Color::from_rgba(255, 255, 0, 255)),    // Yellow
            (0.5, Color::from_rgba(255, 100, 0, 255)),    // Orange
            (1.0, Color::from_rgba(255, 0, 0, 255)),      // Red
        ])
    }
} 