# Ruty Game Engine Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Core Components](#core-components)
4. [Physics System](#physics-system)
5. [UI System](#ui-system)
6. [Utility Systems](#utility-systems)
7. [Testing Framework](#testing-framework)
8. [Examples](#examples)
9. [Best Practices](#best-practices)
10. [API Reference](#api-reference)

## Overview

Ruty is a modern 2D game engine built in Rust using the Macroquad framework. It provides a comprehensive set of features for game development, including:

- Component-based architecture
- Physics simulation
- UI system with modern design
- Input handling
- Resource management
- Testing framework

### Key Features

- **Component System**: Flexible and extensible component-based architecture
- **Physics Engine**: Built-in physics with gravity, collision, and friction
- **UI Framework**: Modern, animated UI components with theme support
- **Input System**: Robust keyboard and mouse input handling
- **Resource Management**: Efficient font and asset loading
- **Testing**: Comprehensive test framework for all components

## Architecture

### Project Structure

```
src/
├── basics/           # Core game mechanics
│   ├── collision.rs  # Collision detection and response
│   ├── force.rs      # Force application system
│   ├── friction.rs   # Friction simulation
│   └── gravity.rs    # Gravity implementation
├── objects/          # Game objects
│   ├── quad.rs       # Basic rectangular object
│   └── ui.rs         # UI component system
├── utils/            # Utility functions
│   ├── font_text.rs  # Font loading and text rendering
│   └── screen.rs     # Screen management
├── test/             # Test framework
│   ├── mod.rs        # Test module definitions
│   └── ui_test.rs    # UI component tests
└── main.rs           # Main application entry
```

### Core Design Principles

1. **Component-Based Architecture**
   - Objects are composed of reusable components
   - Components can be added/removed at runtime
   - Clear separation of concerns

2. **Event-Driven System**
   - Components communicate through events
   - Decoupled design for better maintainability
   - Easy to extend with new features

3. **Resource Management**
   - Efficient asset loading
   - Memory management
   - Resource pooling

## Core Components

### Quad

The `Quad` struct is the fundamental building block for game objects.

```rust
pub struct Quad {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub color: Color,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub components: Vec<Box<dyn Component>>,
}
```

#### Key Features:
- Position and size management
- Color customization
- Velocity tracking
- Component system integration

#### Methods:
- `new(x, y, width, height, color)`: Creates a new Quad
- `add_component(component)`: Adds a component to the Quad
- `remove_component<T>()`: Removes a component of type T
- `draw()`: Renders the Quad
- `is_colliding_with(other)`: Checks collision with another Quad

### Component System

The component system allows for flexible object behavior through composition.

```rust
pub trait Component {
    fn update(&mut self, quad: &mut Quad);
    fn on_collide(&mut self, quad: &mut Quad, other: &Quad);
}
```

#### Available Components:

1. **Gravity**
   ```rust
   pub struct Gravity {
       pub strength: f32,
   }
   ```
   - Applies downward force
   - Configurable strength
   - Affects vertical velocity

2. **Collision**
   ```rust
   pub struct Collision {
       pub is_colliding: bool,
   }
   ```
   - Handles collision detection
   - Manages collision response
   - Prevents object overlap

3. **Friction**
   ```rust
   pub struct Friction {
       pub coefficient: f32,
   }
   ```
   - Applies resistance to movement
   - Configurable coefficient
   - Affects both axes

4. **Force**
   ```rust
   pub struct Force {
       pub x: f32,
       pub y: f32,
   }
   ```
   - Applies directional force
   - Temporary effect
   - Used for movement and jumping

### Force System

The Force system in Ruty supports both permanent and temporary forces, allowing for complex movement patterns and physics simulations.

#### Force Types

1. **Permanent Force**
   ```rust
   pub struct PermanentForce {
       pub x: f32,
       pub y: f32,
   }
   ```
   - Continuously applies force
   - Stays active until removed
   - Used for constant effects like wind or magnetic fields

   Example:
   ```rust
   // Create a permanent force pushing right
   let wind = PermanentForce::new(0.5, 0.0);
   cube.add_component(Box::new(wind));
   ```

2. **Temporary Force**
   ```rust
   pub struct Force {
       pub x: f32,
       pub y: f32,
   }
   ```
   - Applies force for one frame
   - Automatically removed after application
   - Used for instant effects like jumps or impacts

   Example:
   ```rust
   // Apply a jump force
   if is_key_down(KeyCode::Space) && on_ground {
       cube.add_component(Box::new(Force::new(0.0, -10.0)));
   }
   ```

#### Force Application

Forces are applied in the following order:
1. Permanent forces (e.g., gravity, wind)
2. Temporary forces (e.g., jumps, impacts)
3. Friction and damping

Example of complex force interaction:
```rust
// Create a player with multiple forces
let mut player = Quad::new(200.0, 0.0, 50.0, 50.0, WHITE);

// Add permanent forces
player.add_component(Box::new(Gravity::new(0.5)));  // Constant downward force
player.add_component(Box::new(PermanentForce::new(0.2, 0.0)));  // Constant wind

// Game loop
loop {
    // Handle movement
    if is_key_down(KeyCode::D) {
        // Add temporary force for movement
        player.add_component(Box::new(Force::new(1.0, 0.0)));
    }
    if is_key_down(KeyCode::A) {
        player.add_component(Box::new(Force::new(-1.0, 0.0)));
    }
    if is_key_down(KeyCode::Space) && on_ground {
        // Add temporary force for jumping
        player.add_component(Box::new(Force::new(0.0, -10.0)));
    }

    // Update components
    for comp in player.components.iter_mut() {
        comp.update(&mut player);
    }

    // Apply forces to velocity
    player.velocity_x += player.force_x;
    player.velocity_y += player.force_y;

    // Reset temporary forces
    player.remove_component::<Force>();
}
```

#### Force Combinations

Different types of forces can be combined for complex behaviors:

1. **Movement with Momentum**
   ```rust
   // Player movement with momentum
   let mut player = Quad::new(200.0, 0.0, 50.0, 50.0, WHITE);
   player.add_component(Box::new(Gravity::new(0.5)));
   player.add_component(Box::new(Friction::new(0.95)));  // High friction for momentum

   // Movement with momentum
   if is_key_down(KeyCode::D) {
       player.add_component(Box::new(Force::new(0.5, 0.0)));  // Smaller force for momentum
   }
   ```

2. **Wind and Gravity**
   ```rust
   // Object affected by wind and gravity
   let mut object = Quad::new(200.0, 0.0, 50.0, 50.0, WHITE);
   object.add_component(Box::new(Gravity::new(0.5)));
   object.add_component(Box::new(PermanentForce::new(0.3, 0.0)));  // Wind force
   object.add_component(Box::new(Friction::new(0.98)));  // Air resistance
   ```

3. **Bouncing with Damping**
   ```rust
   // Bouncing ball with damping
   let mut ball = Quad::new(200.0, 0.0, 30.0, 30.0, WHITE);
   ball.add_component(Box::new(Gravity::new(0.5)));
   ball.add_component(Box::new(Friction::new(0.99)));  // Slight damping

   // On collision with ground
   if ball.is_colliding_with(&ground) {
       ball.velocity_y = -ball.velocity_y * 0.8;  // Bounce with damping
   }
   ```

#### Force Debugging

To debug force applications, you can add visualization:

```rust
impl Quad {
    pub fn draw_forces(&self) {
        // Draw force vectors
        let force_length = 50.0;  // Scale factor for visualization
        draw_line(
            self.position.0 + self.size.0 / 2.0,
            self.position.1 + self.size.1 / 2.0,
            self.position.0 + self.size.0 / 2.0 + self.force_x * force_length,
            self.position.1 + self.size.1 / 2.0 + self.force_y * force_length,
            2.0,
            RED,
        );
    }
}
```

### Component System Details

#### Component Lifecycle

1. **Initialization**
   ```rust
   impl Component for MyComponent {
       fn new() -> Self {
           Self {
               // Initialize component state
           }
       }
   }
   ```

2. **Update Cycle**
   ```rust
   impl Component for MyComponent {
       fn update(&mut self, quad: &mut Quad) {
           // Update component state
           // Apply effects to quad
       }
   }
   ```

3. **Collision Response**
   ```rust
   impl Component for MyComponent {
       fn on_collide(&mut self, quad: &mut Quad, other: &Quad) {
           // Handle collision
           // Modify quad state
       }
   }
   ```

#### Component Communication

Components can communicate through the Quad:

```rust
// Component that tracks health
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Component for Health {
    fn update(&mut self, quad: &mut Quad) {
        // Update health
    }
}

// Component that applies damage
pub struct Damage {
    pub amount: f32,
}

impl Component for Damage {
    fn on_collide(&mut self, quad: &mut Quad, other: &Quad) {
        if let Some(health) = quad.get_component_mut::<Health>() {
            health.current -= self.amount;
        }
    }
}
```

## Physics System

### Gravity Implementation

```rust
impl Component for Gravity {
    fn update(&mut self, quad: &mut Quad) {
        quad.velocity_y += self.strength;
    }
}
```

- Applies constant downward acceleration
- Configurable strength parameter
- Affects vertical movement

### Collision Detection

```rust
impl Quad {
    pub fn is_colliding_with(&self, other: &Quad) -> bool {
        self.position.0 < other.position.0 + other.size.0 &&
        self.position.0 + self.size.0 > other.position.0 &&
        self.position.1 < other.position.1 + other.size.1 &&
        self.position.1 + self.size.1 > other.position.1
    }
}
```

- AABB (Axis-Aligned Bounding Box) collision
- Efficient implementation
- Handles all collision cases

### Friction System

```rust
impl Component for Friction {
    fn update(&mut self, quad: &mut Quad) {
        quad.velocity_x *= self.coefficient;
        quad.velocity_y *= self.coefficient;
    }
}
```

- Applies resistance to movement
- Configurable coefficient
- Affects both axes independently

## UI System

### Theme System

The theme system in Ruty provides a comprehensive way to manage UI styling and appearance. It supports dynamic theme switching, custom themes, and consistent styling across all UI components.

#### Theme Structure

```rust
pub struct Theme {
    pub primary: Color,      // Main color for primary elements
    pub secondary: Color,    // Secondary color for less prominent elements
    pub accent: Color,       // Accent color for highlights and interactions
    pub background: Color,   // Background color
    pub text: Color,         // Text color
    pub error: Color,        // Error state color
    pub success: Color,      // Success state color
    pub border_radius: f32,  // Corner radius for rounded elements
    pub padding: f32,        // Default padding for elements
    pub animation_speed: f32, // Default animation speed
}
```

#### Built-in Themes

1. **Dark Theme**
   ```rust
   impl Theme {
       pub fn dark() -> Self {
           Self {
               primary: Color::from_rgba(41, 41, 41, 255),
               secondary: Color::from_rgba(61, 61, 61, 255),
               accent: Color::from_rgba(0, 122, 255, 255),
               background: Color::from_rgba(28, 28, 30, 255),
               text: Color::from_rgba(255, 255, 255, 255),
               error: Color::from_rgba(255, 59, 48, 255),
               success: Color::from_rgba(52, 199, 89, 255),
               border_radius: 8.0,
               padding: 10.0,
               animation_speed: 0.2,
           }
       }
   }
   ```

2. **Light Theme**
   ```rust
   impl Theme {
       pub fn light() -> Self {
           Self {
               primary: Color::from_rgba(255, 255, 255, 255),
               secondary: Color::from_rgba(242, 242, 247, 255),
               accent: Color::from_rgba(0, 122, 255, 255),
               background: Color::from_rgba(255, 255, 255, 255),
               text: Color::from_rgba(0, 0, 0, 255),
               error: Color::from_rgba(255, 59, 48, 255),
               success: Color::from_rgba(52, 199, 89, 255),
               border_radius: 8.0,
               padding: 10.0,
               animation_speed: 0.2,
           }
       }
   }
   ```

3. **Custom Theme**
   ```rust
   impl Theme {
       pub fn custom(primary: Color, secondary: Color, accent: Color) -> Self {
           Self {
               primary,
               secondary,
               accent,
               background: Color::from_rgba(28, 28, 30, 255),
               text: Color::from_rgba(255, 255, 255, 255),
               error: Color::from_rgba(255, 59, 48, 255),
               success: Color::from_rgba(52, 199, 89, 255),
               border_radius: 8.0,
               padding: 10.0,
               animation_speed: 0.2,
           }
       }
   }
   ```

#### Theme Usage Examples

1. **Basic Theme Application**
   ```rust
   // Create a panel with dark theme
   let theme = Theme::dark();
   let mut panel = UiPanel::new(
       20.0,
       20.0,
       300.0,
       500.0,
       theme.clone(),
       Some("Dark Panel".to_string()),
   );
   ```

2. **Theme Switching**
   ```rust
   // Toggle between light and dark themes
   let mut is_dark = true;
   let mut theme = Theme::dark();

   // In your update loop
   if is_key_pressed(KeyCode::T) {
       is_dark = !is_dark;
       theme = if is_dark { Theme::dark() } else { Theme::light() };
   }
   ```

3. **Custom Theme Creation**
   ```rust
   // Create a custom theme with specific colors
   let theme = Theme::custom(
       Color::from_rgba(255, 0, 0, 255),    // Red primary
       Color::from_rgba(0, 255, 0, 255),    // Green secondary
       Color::from_rgba(0, 0, 255, 255),    // Blue accent
   );
   ```

### Animation System

The animation system provides smooth transitions and effects for UI elements. It supports various animation types and easing functions.

#### Animation Structure

```rust
pub struct Animation {
    pub current: f32,    // Current animation value
    pub target: f32,     // Target value to animate to
    pub speed: f32,      // Animation speed
}
```

#### Animation Types

1. **Linear Interpolation**
   ```rust
   impl Animation {
       pub fn lerp(&mut self) {
           self.current += (self.target - self.current) * self.speed;
       }
   }
   ```

2. **Ease Out**
   ```rust
   impl Animation {
       pub fn ease_out(&mut self) {
           let diff = self.target - self.current;
           self.current += diff * (1.0 - (-self.speed * 2.0).exp());
       }
   }
   ```

3. **Bounce**
   ```rust
   impl Animation {
       pub fn bounce(&mut self) {
           let diff = self.target - self.current;
           self.current += diff * self.speed;
           if diff.abs() < 0.01 {
               self.current = self.target;
           }
       }
   }
   ```

4. **Spring**
   ```rust
   impl Animation {
       pub fn spring(&mut self) {
           let diff = self.target - self.current;
           self.current += diff * self.speed;
           if diff.abs() < 0.01 {
               self.current = self.target;
           }
       }
   }
   ```

#### Animation Usage Examples

1. **Button Hover Effect**
   ```rust
   impl UiButton {
       pub fn update(&mut self, theme: &Theme) {
           let (mouse_x, mouse_y) = mouse_position();
           let bounds = self.get_bounds();

           // Update hover animation
           if mouse_x >= bounds.0 && mouse_x <= bounds.0 + bounds.2 &&
              mouse_y >= bounds.1 && mouse_y <= bounds.1 + bounds.3 {
               self.hover_animation.set_target(1.0);
           } else {
               self.hover_animation.set_target(0.0);
           }
           self.hover_animation.ease_out();
       }

       pub fn draw(&self, theme: &Theme) {
           // Calculate animated scale
           let scale = 1.0 + self.hover_animation.current * 0.1;
           
           // Draw button with animation
           draw_rounded_rectangle(
               self.x,
               self.y,
               self.width * scale,
               self.height * scale,
               theme.border_radius,
               theme.primary,
           );
       }
   }
   ```

2. **Progress Bar Animation**
   ```rust
   impl UiProgressBar {
       pub fn update(&mut self, theme: &Theme) {
           // Animate progress change
           self.progress_animation.set_target(self.progress);
           self.progress_animation.lerp();
       }

       pub fn draw(&self, theme: &Theme) {
           // Draw background
           draw_rounded_rectangle(
               self.x,
               self.y,
               self.width,
               self.height,
               theme.border_radius,
               theme.secondary,
           );

           // Draw progress with animation
           let progress_width = self.width * self.progress_animation.current;
           draw_rounded_rectangle(
               self.x,
               self.y,
               progress_width,
               self.height,
               theme.border_radius,
               theme.accent,
           );
       }
   }
   ```

3. **Panel Transition**
   ```rust
   impl UiPanel {
       pub fn update(&mut self, theme: &Theme) {
           // Animate panel opening/closing
           self.visibility_animation.set_target(if self.is_visible { 1.0 } else { 0.0 });
           self.visibility_animation.ease_out();
       }

       pub fn draw(&self, theme: &Theme) {
           // Calculate animated properties
           let scale = 0.8 + self.visibility_animation.current * 0.2;
           let alpha = self.visibility_animation.current;
           
           // Draw panel with animation
           draw_rounded_rectangle(
               self.x,
               self.y,
               self.width * scale,
               self.height * scale,
               theme.border_radius,
               Color::from_rgba(
                   theme.background.r,
                   theme.background.g,
                   theme.background.b,
                   (alpha * 255.0) as u8,
               ),
           );
       }
   }
   ```

### UI Components

1. **UiPanel**
   ```rust
   pub struct UiPanel {
       pub x: f32,
       pub y: f32,
       pub width: f32,
       pub height: f32,
       pub title: Option<String>,
       pub elements: Vec<Box<dyn UiElement>>,
       pub theme: Theme,
   }
   ```
   - Container for UI elements
   - Optional title
   - Manages child elements

2. **UiButton**
   ```rust
   pub struct UiButton {
       pub x: f32,
       pub y: f32,
       pub width: f32,
       pub height: f32,
       pub text: String,
       pub font_size: u16,
       pub font: Font,
       pub theme: Theme,
       pub on_click: Option<Box<dyn Fn()>>,
   }
   ```
   - Clickable button
   - Hover and press animations
   - Customizable callback

3. **UiInput**
   ```rust
   pub struct UiInput {
       pub x: f32,
       pub y: f32,
       pub width: f32,
       pub height: f32,
       pub text: String,
       pub placeholder: String,
       pub font_size: u16,
       pub font: Font,
       pub theme: Theme,
       pub on_change: Option<Box<dyn Fn(String)>>,
   }
   ```
   - Text input field
   - Placeholder support
   - Change callback

4. **UiSlider**
   ```rust
   pub struct UiSlider {
       pub x: f32,
       pub y: f32,
       pub width: f32,
       pub height: f32,
       pub min: f32,
       pub max: f32,
       pub value: f32,
       pub theme: Theme,
       pub on_change: Option<Box<dyn Fn(f32)>>,
   }
   ```
   - Range input
   - Configurable min/max
   - Value change callback

5. **UiCheckbox**
   ```rust
   pub struct UiCheckbox {
       pub x: f32,
       pub y: f32,
       pub size: f32,
       pub checked: bool,
       pub theme: Theme,
       pub on_change: Option<Box<dyn Fn(bool)>>,
   }
   ```
   - Toggle input
   - Checked state
   - Change callback

6. **UiProgressBar**
   ```rust
   pub struct UiProgressBar {
       pub x: f32,
       pub y: f32,
       pub width: f32,
       pub height: f32,
       pub progress: f32,
       pub theme: Theme,
   }
   ```
   - Progress visualization
   - Configurable size
   - Smooth updates

7. **UiDropdown**
   ```rust
   pub struct UiDropdown {
       pub x: f32,
       pub y: f32,
       pub width: f32,
       pub height: f32,
       pub options: Vec<String>,
       pub selected_index: usize,
       pub is_open: bool,
       pub theme: Theme,
       pub font: Font,
       pub font_size: u16,
       pub on_select: Option<Box<dyn Fn(usize)>>,
       pub hover_index: Option<usize>,
   }
   ```
   - Dropdown menu
   - Multiple options
   - Selection callback

### UI System Deep Dive

The UI system in Ruty is designed to be flexible, performant, and easy to use. It provides a comprehensive set of components that can be combined to create complex user interfaces.

#### Component Architecture

Each UI component in Ruty follows a consistent architecture:

1. **Base Structure**
   - Every component implements the `UiElement` trait
   - Components maintain their own state and animations
   - Components handle their own input and rendering
   - Components can be nested within panels

2. **State Management**
   - Components maintain internal state (e.g., hover, focus, value)
   - State changes trigger animations and callbacks
   - State can be observed and modified externally

3. **Event System**
   - Components emit events for user interactions
   - Events can be handled through callbacks
   - Events propagate up the component hierarchy

#### Component Types and Usage

1. **Panels (UiPanel)**
   - Container components that group other UI elements
   - Support scrolling and clipping
   - Can be nested to create complex layouts
   - Example:
   ```rust
   // Create a main panel
   let mut main_panel = UiPanel::new(
       20.0,    // x position
       20.0,    // y position
       300.0,   // width
       500.0,   // height
       theme.clone(),
       Some("Main Panel".to_string()),
   );
   ```

2. **Text (UiText)**
   - Displays formatted text with custom fonts
   - Supports multiple alignment options
   - Can be styled with theme colors
   - Example:
   ```rust
   let mut title = UiText::new(
       "Welcome to Ruty!",
       40.0,    // x position
       60.0,    // y position
       24,      // font size
       theme.text,
       font.clone(),
   );
   title.set_alignment(TextAlignment::Center);
   ```

3. **Input Fields (UiInput)**
   - Text input with validation
   - Placeholder text support
   - Change callbacks
   - Example:
   ```rust
   let input = UiInput::new(
       40.0,    // x position
       130.0,   // y position
       220.0,   // width
       30.0,    // height
       16,      // font size
       font.clone(),
       theme.clone(),
       "Enter text...",
       Some(Box::new(|text| println!("Input: {}", text))),
   );
   ```

4. **Sliders (UiSlider)**
   - Range-based input
   - Custom min/max values
   - Smooth animations
   - Example:
   ```rust
   let slider = UiSlider::new(
       40.0,    // x position
       180.0,   // y position
       220.0,   // width
       20.0,    // height
       0.0,     // min value
       100.0,   // max value
       50.0,    // initial value
       theme.clone(),
       Some(Box::new(|value| println!("Value: {}", value))),
   );
   ```

5. **Checkboxes (UiCheckbox)**
   - Boolean input
   - Toggle animations
   - State callbacks
   - Example:
   ```rust
   let checkbox = UiCheckbox::new(
       40.0,    // x position
       220.0,   // y position
       20.0,    // size
       false,   // initial state
       theme.clone(),
       Some(Box::new(|checked| println!("Checked: {}", checked))),
   );
   ```

6. **Progress Bars (UiProgressBar)**
   - Visual progress indication
   - Smooth animations
   - Customizable appearance
   - Example:
   ```rust
   let progress_bar = UiProgressBar::new(
       40.0,    // x position
       260.0,   // y position
       220.0,   // width
       20.0,    // height
       0.5,     // initial progress (0.0 to 1.0)
       theme.clone(),
   );
   ```

7. **Dropdowns (UiDropdown)**
   - Selection from multiple options
   - Customizable options list
   - Selection callbacks
   - Example:
   ```rust
   let dropdown = UiDropdown::new(
       40.0,    // x position
       300.0,   // y position
       220.0,   // width
       30.0,    // height
       vec!["Option 1".to_string(), "Option 2".to_string()],
       theme.clone(),
       font.clone(),
       16,      // font size
       Some(Box::new(|index| println!("Selected: {}", index))),
   );
   ```

8. **Buttons (UiButton)**
   - Clickable elements
   - Hover and press animations
   - Click callbacks
   - Example:
   ```rust
   let button = UiButton::new(
       "Click Me",  // text
       40.0,        // x position
       350.0,       // y position
       220.0,       // width
       40.0,        // height
       18,          // font size
       font.clone(),
       theme.clone(),
       Some(Box::new(|| println!("Button clicked!"))),
   );
   ```

#### Best Practices

1. **Layout Management**
   - Use panels to group related components
   - Maintain consistent spacing between elements
   - Consider screen size and resolution
   - Use relative positioning when possible

2. **Theme Usage**
   - Create consistent themes for your application
   - Use theme colors for all visual elements
   - Consider dark/light mode support
   - Customize theme properties for specific components

3. **Animation Guidelines**
   - Keep animations subtle and purposeful
   - Use appropriate easing functions
   - Consider performance impact
   - Maintain consistent animation speeds

4. **Event Handling**
   - Use callbacks for component interactions
   - Handle errors gracefully
   - Provide feedback for user actions
   - Consider accessibility

5. **Performance Optimization**
   - Minimize component updates
   - Use efficient rendering techniques
   - Cache expensive calculations
   - Profile and optimize as needed

#### Common Patterns

1. **Form Handling**
   ```rust
   // Create a form panel
   let mut form_panel = UiPanel::new(
       20.0, 20.0, 300.0, 400.0,
       theme.clone(),
       Some("User Form".to_string()),
   );

   // Add form fields
   let username_input = UiInput::new(
       40.0, 60.0, 220.0, 30.0,
       16, font.clone(), theme.clone(),
       "Username",
       Some(Box::new(|text| println!("Username: {}", text))),
   );

   let password_input = UiInput::new(
       40.0, 100.0, 220.0, 30.0,
       16, font.clone(), theme.clone(),
       "Password",
       Some(Box::new(|text| println!("Password: {}", text))),
   );

   // Add submit button
   let submit_button = UiButton::new(
       "Submit",
       40.0, 140.0, 220.0, 40.0,
       18, font.clone(), theme.clone(),
       Some(Box::new(|| println!("Form submitted!"))),
   );

   // Add components to panel
   form_panel.add_element(Box::new(username_input));
   form_panel.add_element(Box::new(password_input));
   form_panel.add_element(Box::new(submit_button));
   ```

2. **Settings Panel**
   ```rust
   // Create settings panel
   let mut settings_panel = UiPanel::new(
       20.0, 20.0, 300.0, 400.0,
       theme.clone(),
       Some("Settings".to_string()),
   );

   // Add settings controls
   let volume_slider = UiSlider::new(
       40.0, 60.0, 220.0, 20.0,
       0.0, 100.0, 50.0,
       theme.clone(),
       Some(Box::new(|value| println!("Volume: {}", value))),
   );

   let fullscreen_checkbox = UiCheckbox::new(
       40.0, 100.0, 20.0, false,
       theme.clone(),
       Some(Box::new(|checked| println!("Fullscreen: {}", checked))),
   );

   // Add components to panel
   settings_panel.add_element(Box::new(volume_slider));
   settings_panel.add_element(Box::new(fullscreen_checkbox));
   ```

3. **Navigation Menu**
   ```rust
   // Create navigation panel
   let mut nav_panel = UiPanel::new(
       0.0, 0.0, 200.0, 600.0,
       theme.clone(),
       Some("Navigation".to_string()),
   );

   // Add navigation buttons
   let home_button = UiButton::new(
       "Home",
       20.0, 60.0, 160.0, 40.0,
       18, font.clone(), theme.clone(),
       Some(Box::new(|| println!("Home clicked!"))),
   );

   let settings_button = UiButton::new(
       "Settings",
       20.0, 110.0, 160.0, 40.0,
       18, font.clone(), theme.clone(),
       Some(Box::new(|| println!("Settings clicked!"))),
   );

   // Add components to panel
   nav_panel.add_element(Box::new(home_button));
   nav_panel.add_element(Box::new(settings_button));
   ```

## Utility Systems

### Font Management

```rust
pub struct FontText {
    pub font: Font,
}
```

- Font loading
- Text rendering
- Size management

### Screen Management

```rust
pub fn get_screen_width() -> f32
pub fn get_ground_y(ground_height: f32) -> f32
```

- Screen dimensions
- Ground positioning
- Viewport management

## Testing Framework

### UI Test

```rust
pub async fn run_ui_test()
```

- Tests all UI components
- Interactive demonstration
- Event handling verification

### Component Tests

- Physics system tests
- Collision detection tests
- UI component tests

## Examples

### Basic Game Setup

```rust
#[macroquad::main("Ruty Game Engine")]
async fn main() {
    // Load resources
    let font_text = FontText::load("rsrcs/icon.ttf").await;
    let theme = Theme::default();

    // Create game objects
    let mut cube = Quad::new(200.0, 0.0, 50.0, 50.0, WHITE);
    cube.add_component(Box::new(Gravity::new(0.5)));
    cube.add_component(Box::new(Collision::new()));
    cube.add_component(Box::new(Friction::new(0.85)));

    // Game loop
    loop {
        // Update game state
        // Render game objects
        // Handle input
        next_frame().await;
    }
}
```

### UI Implementation

```rust
// Create UI panel
let mut main_panel = UiPanel::new(
    20.0,
    20.0,
    300.0,
    500.0,
    theme.clone(),
    Some("Game UI".to_string()),
);

// Add UI elements
main_panel.add_element(Box::new(UiButton::new(
    "Start Game",
    40.0,
    350.0,
    220.0,
    40.0,
    18,
    font_text.font.clone(),
    theme.clone(),
    Some(Box::new(|| println!("Game started!"))),
)));
```

## Best Practices

### Component Design

1. **Single Responsibility**
   - Each component should do one thing well
   - Clear separation of concerns
   - Easy to test and maintain

2. **Event Handling**
   - Use callbacks for user interaction
   - Avoid tight coupling
   - Clear event flow

3. **Resource Management**
   - Load resources asynchronously
   - Cache frequently used resources
   - Clean up unused resources

### UI Design

1. **Layout**
   - Consistent spacing
   - Clear visual hierarchy
   - Responsive design

2. **Interaction**
   - Clear feedback
   - Smooth animations
   - Intuitive controls

3. **Performance**
   - Efficient rendering
   - Minimal updates
   - Resource optimization

## API Reference

### Core Types

- `Quad`: Basic game object
- `Component`: Behavior interface
- `Theme`: UI styling
- `UiElement`: UI component interface

### Physics Components

- `Gravity`: Downward force
- `Collision`: Collision detection
- `Friction`: Movement resistance
- `Force`: Directional force

### UI Components

- `UiPanel`: Container
- `UiButton`: Clickable button
- `UiInput`: Text input
- `UiSlider`: Range input
- `UiCheckbox`: Toggle input
- `UiProgressBar`: Progress visualization
- `UiDropdown`: Selection menu

### Utility Functions

- `FontText::load`: Font loading
- `get_screen_width`: Screen width
- `get_ground_y`: Ground position

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## License

MIT License - See LICENSE file for details 