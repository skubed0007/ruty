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
use crate::utils::screen;
use screen::{get_ground_y, get_screen_width};

#[macroquad::main("Ruty Game Engine")]
async fn main() {
    // Player cube positioned somewhere near top-left
    let mut cube = Quad::new(200.0, 0.0, 50.0, 50.0, WHITE);
    // Add persistent components: gravity, collision, friction
    cube.add_component(Box::new(Gravity::new(0.5)));
    cube.add_component(Box::new(Collision::new()));
    cube.add_component(Box::new(Friction::new(0.85)));

    let mut on_ground = false;

    loop {
        let ground_height = 50.0;
        let ground_y = get_ground_y(ground_height);

        // Ground Quad positioned at bottom of the screen
        let ground = Quad::new(0.0, ground_y, get_screen_width(), ground_height, WHITE);

        clear_background(BLACK);

        // Draw the ground at the bottom of the screen
        ground.draw();

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
