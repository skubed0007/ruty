# Ruty Game Engine

Ruty is a simple, educational 2D game engine written in Rust, built on top of [macroquad](https://github.com/not-fl3/macroquad). It demonstrates a component-based architecture inspired by engines like Unity, making it easy to add, remove, and combine behaviors for game objects.

## Features

- **Component System**: Attach modular behaviors (gravity, friction, collision, force, etc.) to game objects (Quads).
- **Physics Simulation**: Basic gravity, friction, and force application for 2D platformer-like movement.
- **Collision Handling**: Simple collision detection and resolution for ground/platforms.
- **Input Handling**: Move and jump using keyboard input (A/D/Space/Shift).
- **Rendering**: Draws colored rectangles (Quads) for player and ground.
- **Screen Utilities**: Helpers for screen size, ground placement, and centering.
- **Custom font and text rendering** using Macroquad's TTF support

## Project Structure

```
src/
  main.rs            # Game loop and engine entry point
  basics/            # Core components (gravity, force, friction, collision)
  objects/           # Game object definitions (Quad)
  utils/             # Utility functions (screen size, ground position)
rsrcs/               # Icons and resources
```

## Example Usage

The main game loop creates a player Quad, attaches gravity, collision, and friction components, and processes input for movement and jumping. Components are updated each frame, and collisions with the ground are resolved.

```rust
let mut cube = Quad::new(200.0, 0.0, 50.0, 50.0, WHITE);
cube.add_component(Box::new(Gravity::new(0.5)));
cube.add_component(Box::new(Collision::new()));
cube.add_component(Box::new(Friction::new(0.85)));
```

## Controls

- **A/D**: Move left/right
- **Space**: Jump (when on ground)
- **Shift**: Sprint

## Building & Running

Make sure you have Rust and Cargo installed. Then run:

```fish
cargo run
```

## Dependencies
- [macroquad](https://crates.io/crates/macroquad) (graphics, input, windowing)
- [image](https://crates.io/crates/image) (optional, for image handling)

## License

MIT License. See [LICENSE](LICENSE) for details.

---

*Ruty Game Engine – for learning, prototyping, and fun!*
