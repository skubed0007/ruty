pub mod basics;
pub mod objects;
pub mod utils;

use macroquad::color::{BLACK, WHITE};
use macroquad::input::{KeyCode, is_key_down};
use macroquad::window::clear_background;
use macroquad::window::next_frame;

use crate::basics::collision::Collision;
use crate::basics::force::Force;
use crate::basics::friction::Friction;
use crate::basics::gravity::Gravity;
use crate::objects::quad::Quad;
use crate::objects::{UiText, UiButton, UiElement};
use crate::utils::screen;
use crate::utils::font_text::FontText;
use screen::{get_ground_y, get_screen_width};
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

#[macroquad::main("Ruty Game Engine")]
async fn main() {
    // Load a custom font (async)
    let font_text = FontText::load("rsrcs/icon.ttf").await;

    // Player cube positioned somewhere near top-left
    let mut cube = Quad::new(200.0, 0.0, 50.0, 50.0, WHITE);
    // Add persistent components: gravity, collision, friction
    cube.add_component(Box::new(Gravity::new(0.5)));
    cube.add_component(Box::new(Collision::new()));
    cube.add_component(Box::new(Friction::new(0.85)));

    // Create a sample UI text object (replaces direct font_text.draw)
    let mut ui_text = UiText::new(
        "Ruty Game Engine Demo, a GUI editor shall be added soon",
        20.0,
        40.0,
        40,
        WHITE,
        font_text.font.clone(),
    );

    // Create a sample button to exit
    let should_exit = Arc::new(Mutex::new(false));
    let exit_flag = should_exit.clone();
    let mut exit_button = UiButton::new(
        "Exit",
        20.0,
        90.0,
        120.0,
        50.0,
        32,
        font_text.font.clone(),
        macroquad::color::Color::from_rgba(60, 60, 60, 255),
        macroquad::color::Color::from_rgba(100, 100, 100, 255),
        macroquad::color::Color::from_rgba(200, 60, 60, 255),
        WHITE,
        Some(Box::new(move || {
            let mut flag = exit_flag.lock().unwrap();
            *flag = true;
        })),
    );

    let mut on_ground = false;

    loop {
        let ground_height = 50.0;
        let ground_y = get_ground_y(ground_height);

        // Ground Quad positioned at bottom of the screen
        let ground = Quad::new(0.0, ground_y, get_screen_width(), ground_height, WHITE);

        clear_background(BLACK);

        // Draw the ground at the bottom of the screen
        ground.draw();

        // Draw and update UI text and button
        ui_text.draw();
        exit_button.update();
        exit_button.draw();
        if *should_exit.lock().unwrap() {
            break;
        }

        // Remove old forces each frame to prevent accumulation
        cube.remove_component::<Force>();

        // Apply input-based forces independently (allows moving + jumping simultaneously)
        // Check if Shift is held for sprinting
        let sprint_multiplier =
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                2.0
            } else {
                1.0
            };

        if is_key_down(KeyCode::D) {
            cube.add_component(Box::new(Force::new(1.0 * sprint_multiplier, 0.0)));
        }
        if is_key_down(KeyCode::A) {
            cube.add_component(Box::new(Force::new(-1.0 * sprint_multiplier, 0.0)));
        }
        if is_key_down(KeyCode::Space) && on_ground {
            cube.add_component(Box::new(Force::new(0.0, -10.0))); // jump force stays same
        }

        // Update all components and apply their effects on cube
        let mut components = std::mem::take(&mut cube.components);
        for comp in components.iter_mut() {
            comp.update(&mut cube);
        }

        // Move cube according to velocity calculated by components
        cube.position.0 += cube.velocity_x;
        cube.position.1 += cube.velocity_y;

        // Collision detection and resolution
        if cube.is_colliding_with(&ground) {
            on_ground = true;
            for comp in components.iter_mut() {
                comp.on_collide(&mut cube, &ground);
            }
        } else {
            on_ground = false;
        }

        // Restore updated components back to cube
        cube.components = components;

        // Draw the cube on the screen
        cube.draw();

        next_frame().await;
    }
}
