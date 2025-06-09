pub mod basics;
pub mod objects;
pub mod utils;
pub mod test;

use macroquad::color::{Color, BLACK, WHITE};
use macroquad::input::{KeyCode, is_key_down};
use macroquad::miniquad::window::set_window_size;
use macroquad::window::{clear_background, next_frame, set_fullscreen};
use std::sync::{Arc, Mutex};

use crate::basics::collision::Collision;
use crate::basics::friction::Friction;
use crate::basics::gravity::Gravity;
use crate::basics::physics_config::{PhysicsConfig, PhysicsPreset};
use crate::objects::quad::Quad;
use crate::objects::ui::{
    Theme, UiText, UiButton, UiInput, UiSlider, UiCheckbox, 
    UiPanel, UiProgressBar, UiDropdown, TextAlignment, UiElement
};
use crate::test::gradient_test;
use crate::test::ui_test::run_ui_example;
use crate::utils::screen;
use crate::utils::font_text::FontText;
use crate::utils::window::WindowBuilder;
use screen::{get_ground_y, get_screen_width};
use crate::objects::ui::UiManager;

#[macroquad::main("Ruty Game Engine")]
async fn main() {
    gradient_test::run_gradient_test().await;
    // Configure the window
    set_window_size(1280, 720);
    set_fullscreen(false);

    // Configure the window
    WindowBuilder::new()
        .title("Ruty Game Engine")
        .size(1280, 720)
        .target_fps(60)
        .build()
        .await;

    // Configure physics
    let physics_config = PhysicsConfig::new()
        .gravity(9.81)
        .friction(0.8)
        .bounce(0.5)
        .air_resistance(0.1)
        .add_preset("low_gravity", PhysicsConfig::low_gravity())
        .add_preset("high_friction", PhysicsConfig::high_friction())
        .add_preset("bouncy", PhysicsConfig::bouncy())
        .add_preset("space_like", PhysicsConfig::space_like());

    run_ui_example().await;
    // Run the point physics example
//    crate::test::point_example::run_point_example().await;

    // The rest of the main function...
    // Load a custom font (async)
    let font_text = FontText::load("rsrcs/icon.ttf").await;
    let theme = Theme::default();

    // Player cube positioned somewhere near top-left
    let mut cube = Quad::new(200.0, 0.0, 50.0, 50.0, WHITE);
    // Add persistent components: gravity, collision, friction
    cube.add_component(Box::new(Gravity::new(physics_config.gravity)));
    cube.add_component(Box::new(Collision::new(physics_config.bounce, physics_config.friction)));
    cube.add_component(Box::new(Friction::new(physics_config.friction)));

    // Create UI elements
    let mut main_panel = UiPanel::new(
        20.0,
        20.0,
        300.0,
        500.0,
        theme.clone(),
        Some("Ruty Game Engine".to_string()),
    );

    // Title text
    let mut title_text = UiText::new(
        "Welcome to Ruty!",
        40.0,
        60.0,
        24,
        theme.text,
        font_text.font.clone(),
    );
    title_text.set_alignment(TextAlignment::Center);
    main_panel.add_element(Box::new(title_text));

    // Description text
    let mut desc_text = UiText::new(
        "A modern game engine with beautiful UI",
        40.0,
        90.0,
        16,
        theme.text,
        font_text.font.clone(),
    );
    desc_text.set_alignment(TextAlignment::Center);
    main_panel.add_element(Box::new(desc_text));

    // Input field
    let input = UiInput::new(
        40.0,
        130.0,
        220.0,
        30.0,
        16,
        font_text.font.clone(),
        theme.clone(),
        "Enter your name...",
        Some(Box::new(|text| println!("Input changed: {}", text))),
    );
    main_panel.add_element(Box::new(input));

    // Physics preset dropdown
    let physics_presets = vec![
        "Default".to_string(),
        "Low Gravity".to_string(),
        "High Friction".to_string(),
        "Bouncy".to_string(),
        "Space-like".to_string(),
    ];
    let dropdown = UiDropdown::new(
        40.0,
        300.0,
        220.0,
        30.0,
        physics_presets,
        theme.clone(),
        font_text.font.clone(),
        16,
        Some(Box::new(|index| {
            // Apply physics preset based on selection
            let preset = match index {
                1 => PhysicsConfig::low_gravity(),
                2 => PhysicsConfig::high_friction(),
                3 => PhysicsConfig::bouncy(),
                4 => PhysicsConfig::space_like(),
                _ => PhysicsPreset::new(9.81, 0.8, 0.5, 0.1),
            };
            // TODO: Apply preset to physics system
        })),
    );
    main_panel.add_element(Box::new(dropdown));

    // Checkbox
    let checkbox = UiCheckbox::new(
        40.0,
        220.0,
        20.0,
        false,
        theme.clone(),
        Some(Box::new(|checked| println!("Checkbox: {}", checked))),
    );
    main_panel.add_element(Box::new(checkbox));
    let mut checkbox_label = UiText::new(
        "Enable features",
        70.0,
        225.0,
        16,
        theme.text,
        font_text.font.clone(),
    );
    main_panel.add_element(Box::new(checkbox_label));

    // Progress bar
    let progress_bar = UiProgressBar::new(
        40.0,
        260.0,
        220.0,
        20.0,
        0.5,
        theme.clone(),
    );
    main_panel.add_element(Box::new(progress_bar));

    // Buttons
    let start_button = UiButton::new(
        "Start Game",
        40.0,
        350.0,
        220.0,
        40.0,
        18,
        font_text.font.clone(),
        theme.clone(),
        Some(Box::new(|| println!("Start game clicked!"))),
    );
    main_panel.add_element(Box::new(start_button));

    let settings_button = UiButton::new(
        "Settings",
        40.0,
        400.0,
        220.0,
        40.0,
        18,
        font_text.font.clone(),
        theme.clone(),
        Some(Box::new(|| println!("Settings clicked!"))),
    );
    main_panel.add_element(Box::new(settings_button));

    let exit_button = UiButton::new(
        "Exit",
        40.0,
        450.0,
        220.0,
        40.0,
        18,
        font_text.font.clone(),
        theme.clone(),
        Some(Box::new(|| std::process::exit(0))),
    );
    main_panel.add_element(Box::new(exit_button));

    let mut on_ground = false;
    let mut progress = 0.0;
    let mut ui_manager = UiManager::new();
    let panel_index = ui_manager.add_element(Box::new(main_panel));

    loop {
        let ground_height = 50.0;
        let ground_y = get_ground_y(ground_height);

        // Ground Quad positioned at bottom of the screen
        let ground = Quad::new(0.0, ground_y, get_screen_width(), ground_height, WHITE);

        clear_background(BLACK);

        // Draw the ground at the bottom of the screen
        ground.draw();

        // Update and draw UI
        ui_manager.update();
        ui_manager.draw();

        // Update progress bar
        progress = (progress + 0.001) % 1.0;
        if let Some(element) = ui_manager.get_element_mut(panel_index) {
            if let Some(panel) = element.as_any_mut().downcast_mut::<UiPanel>() {
                for element in &mut panel.elements {
                    if let Some(progress_bar) = element.as_any_mut().downcast_mut::<UiProgressBar>() {
                        progress_bar.set_progress(progress);
                        break;
                    }
                }
            }
        }

        // Update and draw the player cube
        cube.update_components();
        cube.draw();

        // Handle player movement
        if is_key_down(KeyCode::Left) {
            cube.velocity_x = -5.0;
        } else if is_key_down(KeyCode::Right) {
            cube.velocity_x = 5.0;
        } else {
            cube.velocity_x = 0.0;
        }

        // Handle jumping
        if is_key_down(KeyCode::Space) && on_ground {
            cube.velocity_y = -10.0;
            on_ground = false;
        }

        // Check for ground collision
        if cube.position.1 + cube.size.1 >= ground_y {
            cube.position.1 = ground_y - cube.size.1;
            cube.velocity_y = 0.0;
            on_ground = true;
        }

        next_frame().await;
    }
}
