pub mod basics;
pub mod objects;
pub mod utils;
pub mod test;

use macroquad::color::{BLACK, WHITE};
use macroquad::input::{KeyCode, is_key_down};
use macroquad::window::{clear_background, next_frame};

use crate::basics::collision::Collision;
use crate::basics::force::Force;
use crate::basics::friction::Friction;
use crate::basics::gravity::Gravity;
use crate::objects::quad::Quad;
use crate::objects::ui::{
    Theme, UiText, UiButton, UiInput, UiSlider, UiCheckbox, 
    UiPanel, UiProgressBar, UiDropdown, TextAlignment, UiElement
};
use crate::utils::screen;
use crate::utils::font_text::FontText;
use screen::{get_ground_y, get_screen_width};

#[macroquad::main("Ruty Game Engine")]
async fn main() {
    crate::test::ui_test::run_ui_test().await;

    // Load a custom font (async)
    let font_text = FontText::load("rsrcs/icon.ttf").await;
    let theme = Theme::default();

    // Player cube positioned somewhere near top-left
    let mut cube = Quad::new(200.0, 0.0, 50.0, 50.0, WHITE);
    // Add persistent components: gravity, collision, friction
    cube.add_component(Box::new(Gravity::new(0.5)));
    cube.add_component(Box::new(Collision::new()));
    cube.add_component(Box::new(Friction::new(0.85)));

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

    // Slider
    let slider = UiSlider::new(
        40.0,
        180.0,
        220.0,
        20.0,
        0.0,
        100.0,
        50.0,
        theme.clone(),
        Some(Box::new(|value| println!("Slider value: {}", value))),
    );
    main_panel.add_element(Box::new(slider));

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

    // Dropdown
    let dropdown = UiDropdown::new(
        40.0,
        300.0,
        220.0,
        30.0,
        vec!["Option 1".to_string(), "Option 2".to_string(), "Option 3".to_string()],
        theme.clone(),
        font_text.font.clone(),
        16,
        Some(Box::new(|index| println!("Selected option: {}", index))),
    );
    main_panel.add_element(Box::new(dropdown));

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

    loop {
        let ground_height = 50.0;
        let ground_y = get_ground_y(ground_height);

        // Ground Quad positioned at bottom of the screen
        let ground = Quad::new(0.0, ground_y, get_screen_width(), ground_height, WHITE);

        clear_background(BLACK);

        // Draw the ground at the bottom of the screen
        ground.draw();

        // Update and draw UI
        main_panel.update(&theme);
        main_panel.draw(&theme);

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

        // Update progress bar for demo
        progress = (progress + 0.001) % 1.0;
        for element in &mut main_panel.elements {
            if let Some(progress_bar) = element.as_any_mut().downcast_mut::<UiProgressBar>() {
                progress_bar.set_progress(progress);
                break;
            }
        }

        next_frame().await;
    }
}
