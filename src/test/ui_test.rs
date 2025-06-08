use crate::objects::ui::{
    Theme, UiText, UiButton, UiInput, UiSlider, UiCheckbox, 
    UiPanel, UiProgressBar, UiDropdown, TextAlignment, UiElement
};
use crate::utils::font_text::FontText;
use macroquad::color::BLACK;
use macroquad::input::{is_mouse_button_pressed, mouse_position, MouseButton};
use macroquad::window::{clear_background, next_frame};

pub async fn run_ui_test() {
    // Load a custom font (async)
    let font_text = FontText::load("rsrcs/icon.ttf").await;
    let theme = Theme::default();

    // Create UI elements
    let mut main_panel = UiPanel::new(
        20.0,
        20.0,
        300.0,
        600.0, // Increased height to accommodate all elements
        theme.clone(),
        Some("UI Test Panel".to_string()),
    );

    // Title text
    let mut title_text = UiText::new(
        "UI Components Test",
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
        "Testing all UI components",
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
        "Type something...",
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
        "Test checkbox",
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
        vec!["Test Option 1".to_string(), "Test Option 2".to_string(), "Test Option 3".to_string()],
        theme.clone(),
        font_text.font.clone(),
        16,
        Some(Box::new(|index| println!("Selected option: {}", index))),
    );
    main_panel.add_element(Box::new(dropdown));

    // Buttons
    let test_button = UiButton::new(
        "Test Button",
        40.0,
        350.0,
        220.0,
        40.0,
        18,
        font_text.font.clone(),
        theme.clone(),
        Some(Box::new(|| println!("Test button clicked!"))),
    );
    main_panel.add_element(Box::new(test_button));

    let exit_button = UiButton::new(
        "Exit Test",
        40.0,
        400.0,
        220.0,
        40.0,
        18,
        font_text.font.clone(),
        theme.clone(),
        Some(Box::new(|| std::process::exit(0))),
    );
    main_panel.add_element(Box::new(exit_button));

    let mut progress = 0.0;
    let mut last_clicked_element: Option<usize> = None;

    loop {
        clear_background(BLACK);

        // Handle click outside behavior
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let mut clicked_inside = false;
            
            // Check if click is inside any interactive element
            for (i, element) in main_panel.elements.iter().enumerate() {
                let bounds = element.get_bounds();
                if mouse_x >= bounds.0 && mouse_x <= bounds.0 + bounds.2 &&
                   mouse_y >= bounds.1 && mouse_y <= bounds.1 + bounds.3 {
                    clicked_inside = true;
                    last_clicked_element = Some(i);
                    break;
                }
            }

            // If clicked outside, reset last clicked element
            if !clicked_inside {
                last_clicked_element = None;
            }
        }

        // Update and draw UI elements in correct order
        main_panel.update(&theme);
        
        // Draw non-interactive elements first
        for (i, element) in main_panel.elements.iter().enumerate() {
            if last_clicked_element != Some(i) {
                element.draw(&theme);
            }
        }

        // Draw the last clicked element on top
        if let Some(index) = last_clicked_element {
            if let Some(element) = main_panel.elements.get(index) {
                element.draw(&theme);
            }
        }

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