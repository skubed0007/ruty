use macroquad::prelude::*;
use crate::utils::gradient::Gradient;

pub async fn run_gradient_test() {
    let mut current_gradient = 0;
    let gradients = vec![
        ("Sunset", Gradient::sunset()),
        ("Ocean", Gradient::ocean()),
        ("Forest", Gradient::forest()),
        ("Fire", Gradient::fire()),
    ];

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Space) {
            current_gradient = (current_gradient + 1) % gradients.len();
        }

        clear_background(BLACK);

        // Draw the current gradient
        let (name, gradient) = &gradients[current_gradient];
        gradient.draw_vertical(0.0, 0.0, screen_width(), screen_height());

        // Draw instructions
        draw_text(
            &format!("Current: {}\nPress SPACE to change gradient\nPress ESC to exit", name),
            10.0,
            30.0,
            20.0,
            WHITE,
        );

        next_frame().await;
    }
} 