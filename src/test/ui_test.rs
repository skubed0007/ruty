use macroquad::prelude::*;
use crate::objects::ui::*;
use std::path::Path;

pub async fn run_ui_example() {
    // Load custom font
    let font = load_ttf_font("rsrcs/font.ttf").await.unwrap();
    
    // Create UI manager
    let mut ui_manager = UiManager::new();
    
    // Create modern theme
    let modern_theme = Theme {
        primary: Color::from_rgba(41, 128, 185, 255),    // Blue
        secondary: Color::from_rgba(52, 73, 94, 255),    // Dark blue
        accent: Color::from_rgba(231, 76, 60, 255),      // Red
        background: Color::from_rgba(44, 62, 80, 255),   // Darker blue
        text: Color::from_rgba(236, 240, 241, 255),      // Light gray
        error: Color::from_rgba(231, 76, 60, 255),       // Red
        success: Color::from_rgba(46, 204, 113, 255),    // Green
        border_radius: 8.0,
        padding: 12.0,
        animation_speed: 0.2,
    };

    // Create main panel
    let mut main_panel = UiPanel::new(
        50.0,
        50.0,
        800.0,
        600.0,
        modern_theme.clone(),
        Some("Modern UI Example".to_string()),
    );

    // Create header text
    let header = UiText::new(
        "Welcome to Ruty UI",
        100.0,
        100.0,
        32,
        modern_theme.text,
        font.clone(),
    );
    main_panel.add_element(Box::new(header));

    // Create dropdown menu
    let dropdown = UiDropdown::new(
        100.0,
        150.0,
        200.0,
        40.0,
        vec![
            "Option 1".to_string(),
            "Option 2".to_string(),
            "Option 3".to_string(),
            "Option 4".to_string(),
        ],
        modern_theme.clone(),
        font.clone(),
        18,
        Some(Box::new(|index| {
            println!("Selected option: {}", index);
        })),
    );
    main_panel.add_element(Box::new(dropdown));

    // Create buttons
    let primary_button = UiButton::new(
        "Primary Action",
        100.0,
        210.0,
        200.0,
        40.0,
        18,
        font.clone(),
        modern_theme.clone(),
        Some(Box::new(|| {
            println!("Primary button clicked!");
        })),
    );
    main_panel.add_element(Box::new(primary_button));

    let secondary_button = UiButton::new(
        "Secondary Action",
        100.0,
        270.0,
        200.0,
        40.0,
        18,
        font.clone(),
        modern_theme.clone(),
        Some(Box::new(|| {
            println!("Secondary button clicked!");
        })),
    );
    main_panel.add_element(Box::new(secondary_button));

    // Create input field
    let input = UiInput::new(
        100.0,
        330.0,
        200.0,
        40.0,
        18,
        font.clone(),
        modern_theme.clone(),
        "Enter text here...",
        Some(Box::new(|text| {
            println!("Input changed: {}", text);
        })),
    );
    main_panel.add_element(Box::new(input));

    // Create slider
    let slider = UiSlider::new(
        100.0,
        390.0,
        200.0,
        40.0,
        0.0,
        100.0,
        50.0,
        modern_theme.clone(),
        Some(Box::new(|value| {
            println!("Slider value: {}", value);
        })),
    );
    main_panel.add_element(Box::new(slider));

    // Create checkbox
    let checkbox = UiCheckbox::new(
        100.0,
        450.0,
        24.0,
        false,
        modern_theme.clone(),
        Some(Box::new(|checked| {
            println!("Checkbox state: {}", checked);
        })),
    );
    main_panel.add_element(Box::new(checkbox));

    // Create progress bar
    let mut progress_bar = UiProgressBar::new(
        100.0,
        500.0,
        200.0,
        20.0,
        0.0,
        modern_theme.clone(),
    );
    main_panel.add_element(Box::new(progress_bar));

    // Add main panel to UI manager
    ui_manager.add_element(Box::new(main_panel));

    // Main loop
    let mut progress = 0.0;
    loop {
        clear_background(Color::from_rgba(52, 73, 94, 255));

        // Update progress bar
        progress = (progress + 0.01) % 1.0;
        if let Some(element) = ui_manager.get_element_mut(0) {
            if let Some(panel) = element.as_any_mut().downcast_mut::<UiPanel>() {
                if let Some(progress_bar) = panel.elements.last_mut() {
                    if let Some(progress_bar) = progress_bar.as_any_mut().downcast_mut::<UiProgressBar>() {
                        progress_bar.set_progress(progress);
                    }
                }
            }
        }

        // Update and draw UI
        ui_manager.update();
        ui_manager.draw();

        // Draw FPS
        draw_text(
            &format!("FPS: {}", get_fps()),
            10.0,
            20.0,
            20.0,
            WHITE,
        );

        next_frame().await;
    }
} 