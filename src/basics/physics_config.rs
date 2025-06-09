//! Physics Configuration System
//! 
//! This module provides a comprehensive physics configuration system
//! for customizing physics behavior in the Ruty game engine.
//! 
//! # Features
//! - Customizable gravity
//! - Adjustable friction
//! - Configurable collision response
//! - Physics presets
//! 
//! # Examples
//! ```rust
//! use ruty::basics::physics_config::PhysicsConfig;
//! 
//! let config = PhysicsConfig::new()
//!     .gravity(9.81)
//!     .friction(0.8)
//!     .bounce(0.5)
//!     .air_resistance(0.1);
//! ```

use std::collections::HashMap;

/// Physics configuration
#[derive(Debug, Clone)]
pub struct PhysicsConfig {
    /// Global gravity strength
    pub gravity: f32,
    /// Global friction coefficient
    pub friction: f32,
    /// Global bounce coefficient
    pub bounce: f32,
    /// Air resistance coefficient
    pub air_resistance: f32,
    /// Physics presets
    pub presets: HashMap<String, PhysicsPreset>,
    /// Custom physics properties
    pub custom_properties: HashMap<String, f32>,
}

/// Physics preset
#[derive(Debug, Clone)]
pub struct PhysicsPreset {
    /// Gravity strength
    pub gravity: f32,
    /// Friction coefficient
    pub friction: f32,
    /// Bounce coefficient
    pub bounce: f32,
    /// Air resistance coefficient
    pub air_resistance: f32,
}

impl PhysicsConfig {
    /// Create a new physics configuration
    pub fn new() -> Self {
        Self {
            gravity: 9.81,
            friction: 0.8,
            bounce: 0.5,
            air_resistance: 0.1,
            presets: HashMap::new(),
            custom_properties: HashMap::new(),
        }
    }

    /// Set gravity strength
    pub fn gravity(mut self, gravity: f32) -> Self {
        self.gravity = gravity;
        self
    }

    /// Set friction coefficient
    pub fn friction(mut self, friction: f32) -> Self {
        self.friction = friction;
        self
    }

    /// Set bounce coefficient
    pub fn bounce(mut self, bounce: f32) -> Self {
        self.bounce = bounce;
        self
    }

    /// Set air resistance coefficient
    pub fn air_resistance(mut self, air_resistance: f32) -> Self {
        self.air_resistance = air_resistance;
        self
    }

    /// Add a physics preset
    pub fn add_preset(mut self, name: &str, preset: PhysicsPreset) -> Self {
        self.presets.insert(name.to_string(), preset);
        self
    }

    /// Add a custom physics property
    pub fn add_property(mut self, name: &str, value: f32) -> Self {
        self.custom_properties.insert(name.to_string(), value);
        self
    }

    /// Get a physics preset
    pub fn get_preset(&self, name: &str) -> Option<&PhysicsPreset> {
        self.presets.get(name)
    }

    /// Get a custom physics property
    pub fn get_property(&self, name: &str) -> Option<f32> {
        self.custom_properties.get(name).copied()
    }

    /// Create a low gravity preset
    pub fn low_gravity() -> PhysicsPreset {
        PhysicsPreset {
            gravity: 2.0,
            friction: 0.8,
            bounce: 0.7,
            air_resistance: 0.05,
        }
    }

    /// Create a high friction preset
    pub fn high_friction() -> PhysicsPreset {
        PhysicsPreset {
            gravity: 9.81,
            friction: 0.95,
            bounce: 0.2,
            air_resistance: 0.2,
        }
    }

    /// Create a bouncy preset
    pub fn bouncy() -> PhysicsPreset {
        PhysicsPreset {
            gravity: 9.81,
            friction: 0.5,
            bounce: 0.9,
            air_resistance: 0.05,
        }
    }

    /// Create a space-like preset
    pub fn space_like() -> PhysicsPreset {
        PhysicsPreset {
            gravity: 0.1,
            friction: 0.1,
            bounce: 0.8,
            air_resistance: 0.0,
        }
    }
}

impl PhysicsPreset {
    /// Create a new physics preset
    pub fn new(gravity: f32, friction: f32, bounce: f32, air_resistance: f32) -> Self {
        Self {
            gravity,
            friction,
            bounce,
            air_resistance,
        }
    }
} 