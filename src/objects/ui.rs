use macroquad::prelude::*;

pub trait UiElement {
    fn draw(&mut self);
    fn update(&mut self);
}

pub struct UiText {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub font_size: u16,
    pub color: Color,
    pub font: Font,
}

impl UiText {
    pub fn new(text: &str, x: f32, y: f32, font_size: u16, color: Color, font: Font) -> Self {
        Self {
            text: text.to_string(),
            x,
            y,
            font_size,
            color,
            font,
        }
    }
}

impl UiElement for UiText {
    fn draw(&mut self) {
        draw_text_ex(
            &self.text,
            self.x,
            self.y,
            TextParams {
                font: Some(&self.font),
                font_size: self.font_size,
                color: self.color,
                ..Default::default()
            },
        );
    }
    fn update(&mut self) {}
}

pub struct UiButton {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub font_size: u16,
    pub font: Font,
    pub color: Color,
    pub color_hover: Color,
    pub color_pressed: Color,
    pub text_color: Color,
    pub on_click: Option<Box<dyn FnMut() + Send + Sync>>,
    pub hovered: bool,
    pub pressed: bool,
}

impl UiButton {
    pub fn new(
        text: &str,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        font_size: u16,
        font: Font,
        color: Color,
        color_hover: Color,
        color_pressed: Color,
        text_color: Color,
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
            color,
            color_hover,
            color_pressed,
            text_color,
            on_click,
            hovered: false,
            pressed: false,
        }
    }
    pub fn is_mouse_over(&self) -> bool {
        let (mx, my) = mouse_position();
        mx >= self.x && mx <= self.x + self.w && my >= self.y && my <= self.y + self.h
    }
}

impl UiElement for UiButton {
    fn draw(&mut self) {
        let draw_color = if self.pressed {
            self.color_pressed
        } else if self.hovered {
            self.color_hover
        } else {
            self.color
        };
        draw_rectangle(self.x, self.y, self.w, self.h, draw_color);
        let text_dim = measure_text(&self.text, Some(&self.font), self.font_size, 1.0);
        let tx = self.x + (self.w - text_dim.width) / 2.0;
        let ty = self.y + (self.h + text_dim.height) / 2.0 - 4.0;
        draw_text_ex(
            &self.text,
            tx,
            ty,
            TextParams {
                font: Some(&self.font),
                font_size: self.font_size,
                color: self.text_color,
                ..Default::default()
            },
        );
    }
    fn update(&mut self) {
        self.hovered = self.is_mouse_over();
        let mouse_down = is_mouse_button_down(MouseButton::Left);
        let mouse_pressed = is_mouse_button_pressed(MouseButton::Left);
        if self.hovered && mouse_down {
            self.pressed = true;
        } else {
            self.pressed = false;
        }
        if self.hovered && mouse_pressed {
            if let Some(cb) = &mut self.on_click {
                cb();
            }
        }
    }
}
