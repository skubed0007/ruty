use macroquad::prelude::*;

/// Utility for loading and drawing text with custom fonts.
pub struct FontText {
    pub font: Font,
}

impl FontText {
    /// Loads a font from the given file path (relative to project root or absolute).
    pub async fn load(path: &str) -> Self {
        let font = load_ttf_font(path).await.expect("Failed to load font");
        Self { font }
    }

    /// Draws text at the given position, size, and color using the loaded font.
    pub fn draw(&self, text: &str, x: f32, y: f32, font_size: u16, color: Color) {
        draw_text_ex(
            text,
            x,
            y,
            TextParams {
                font: Some(&self.font),
                font_size,
                color,
                ..Default::default()
            },
        );
    }
}
