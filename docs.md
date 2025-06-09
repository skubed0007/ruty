# Ruty Game Engine Documentation

## Table of Contents
1. [Overview](#overview)
2. [Core Concepts](#core-concepts)
3. [Physics System](#physics-system)
4. [UI System](#ui-system)
5. [Game Objects](#game-objects)
6. [Best Practices](#best-practices)

## Overview

Ruty is a modern 2D game engine built in Rust that combines powerful physics simulation with a beautiful UI system. It's designed to be both educational and practical, making it perfect for both learning game development and creating actual games.

### Key Features
- Component-based architecture for flexible game object behavior
- Advanced physics simulation with realistic interactions
- Modern UI system with smooth animations
- Input handling for keyboard and mouse
- Resource management for assets and fonts

## Core Concepts

### Component System
The component system is the heart of Ruty. It allows you to create game objects by combining different behaviors:

1. **Adding Components**
   - Components can be added to any game object
   - Each component adds specific behavior
   - Components can be added or removed at runtime

2. **Component Types**
   - Physics components (gravity, collision, friction)
   - UI components (buttons, panels, inputs)
   - Custom components for game-specific behavior

3. **Component Communication**
   - Components can interact with each other
   - Events and callbacks for component interaction
   - State management through component properties

## Physics System

### Gravity
The gravity component simulates gravitational force:

- **Properties**
  - Strength: Controls how strong the gravity is
  - Direction: Can be customized for different effects
  - Fixed Points: Some objects can be unaffected by gravity

- **Usage**
  - Add to objects that should fall
  - Adjust strength for different game mechanics
  - Combine with other physics components

### Collision
The collision component handles object interactions:

- **Features**
  - AABB (Axis-Aligned Bounding Box) collision detection
  - Slope support for realistic movement
  - Bounce and friction properties
  - Collision response and resolution

- **Usage**
  - Add to objects that need to collide
  - Configure bounce and friction values
  - Handle collision events for game logic

### Friction
The friction component simulates surface resistance:

- **Properties**
  - Coefficient: How much friction to apply
  - Surface Type: Different friction for different surfaces
  - Air Resistance: Optional air friction

- **Usage**
  - Add to moving objects
  - Adjust coefficient for different materials
  - Combine with other physics components

### Force
The force component applies directional forces:

- **Types**
  - Permanent Forces: Constant force application
  - Temporary Forces: One-time force application
  - Impulse Forces: Instant force application

- **Usage**
  - Add for movement and jumping
  - Create wind or magnetic effects
  - Simulate explosions or impacts

## UI System

### Theme System
The theme system provides consistent styling:

- **Properties**
  - Colors: Primary, secondary, accent, and more
  - Typography: Font sizes and styles
  - Spacing: Padding and margins
  - Animations: Transition speeds and effects

- **Usage**
  - Create consistent UI appearance
  - Switch between light and dark themes
  - Customize for different game sections

### UI Components

#### Panel
The panel is a container for other UI elements:

- **Features**
  - Title bar with optional text
  - Scrollable content area
  - Background and border styling
  - Child element management

- **Usage**
  - Create menus and dialogs
  - Group related UI elements
  - Create nested layouts

#### Button
The button component provides clickable elements:

- **Features**
  - Text or icon display
  - Hover and press animations
  - Click callbacks
  - Disabled state

- **Usage**
  - Create menu buttons
  - Add interactive elements
  - Handle user input

#### Input Field
The input field allows text entry:

- **Features**
  - Text input and editing
  - Placeholder text
  - Cursor animation
  - Change callbacks

- **Usage**
  - Create forms
  - Get user input
  - Display editable text

#### Slider
The slider provides range input:

- **Features**
  - Draggable handle
  - Min/max values
  - Value change callbacks
  - Visual feedback

- **Usage**
  - Volume controls
  - Settings adjustments
  - Progress indication

#### Checkbox
The checkbox provides boolean input:

- **Features**
  - Toggle state
  - Checkmark animation
  - State change callbacks
  - Disabled state

- **Usage**
  - Settings toggles
  - Feature enable/disable
  - Form inputs

#### Progress Bar
The progress bar shows completion:

- **Features**
  - Progress visualization
  - Smooth animations
  - Customizable appearance
  - Value updates

- **Usage**
  - Loading indicators
  - Health bars
  - Progress tracking

#### Dropdown
The dropdown provides selection from options:

- **Features**
  - Multiple options
  - Selection callback
  - Hover effects
  - Z-ordering support

- **Usage**
  - Menu navigation
  - Option selection
  - Settings choices

## Game Objects

### Quad
The quad is the basic rectangular game object:

- **Properties**
  - Position and size
  - Color and appearance
  - Velocity and movement
  - Component support

- **Usage**
  - Create platforms
  - Build game levels
  - Make player characters

### Point
The point is used for physics simulation:

- **Properties**
  - Position and velocity
  - Mass and radius
  - Fixed state
  - Component support

- **Usage**
  - Physics simulations
  - Particle systems
  - Constraint systems

## Best Practices

### Performance
1. **Component Management**
   - Remove unused components
   - Use appropriate component types
   - Optimize update cycles

2. **Physics Optimization**
   - Use appropriate collision detection
   - Optimize physics calculations
   - Balance accuracy and performance

3. **UI Optimization**
   - Minimize UI updates
   - Use efficient animations
   - Optimize layout calculations

### Design Patterns
1. **Component Composition**
   - Combine components for complex behavior
   - Keep components focused and simple
   - Use events for communication

2. **State Management**
   - Use appropriate state containers
   - Handle state transitions
   - Maintain clean state flow

3. **Resource Management**
   - Load resources efficiently
   - Cache frequently used resources
   - Clean up unused resources

### UI Design
1. **Layout**
   - Use consistent spacing
   - Create clear visual hierarchy
   - Design for different screen sizes

2. **Interaction**
   - Provide clear feedback
   - Use appropriate animations
   - Handle edge cases

3. **Accessibility**
   - Support keyboard navigation
   - Provide clear visual feedback
   - Consider color contrast

## Getting Started

1. **Setup**
   - Install Rust and Cargo
   - Add Ruty to your project
   - Set up your development environment

2. **First Steps**
   - Create a basic game object
   - Add physics components
   - Create a simple UI

3. **Next Steps**
   - Explore more components
   - Build complex interactions
   - Create your game

## Contributing

1. **Development**
   - Fork the repository
   - Create a feature branch
   - Make your changes
   - Submit a pull request

2. **Documentation**
   - Update documentation
   - Add examples
   - Improve guides

3. **Testing**
   - Write tests
   - Verify changes
   - Check performance

## License

MIT License - See LICENSE file for details 