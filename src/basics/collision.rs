use std::any::Any;

use crate::basics::Component;
use crate::objects::quad::Quad;

/// A component that handles simple collision response between Quads.
///
/// This component is responsible for resolving vertical collisions by
/// adjusting the position of the Quad to sit on top of the "other" Quad,
/// and stopping downward velocity (simulating a solid surface).
///
/// # Behavior
///
/// When a collision is detected, it:
/// - Moves the Quad vertically so its bottom aligns with the top of the collided Quad.
/// - Resets vertical velocity to zero to stop falling.
///
/// # Example
///
/// Used for simple ground/platform collision to prevent objects from falling through.
pub struct Collision;

impl Collision {
    /// Creates a new Collision component instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Collision {
    /// Called when a collision occurs between this Quad (`me`) and another Quad (`other`).
    ///
    /// Adjusts the vertical position of `me` so it rests exactly on top of `other`
    /// and nullifies any downward velocity to simulate a solid collision response.
    ///
    /// # Parameters
    ///
    /// - `me`: The Quad that owns this Collision component (usually the moving object).
    /// - `other`: The Quad with which `me` has collided (usually a static platform or ground).
    fn on_collide(&mut self, me: &mut Quad, other: &Quad) {
        // Position `me` on top of `other`
        me.position.1 = other.position.1 - me.size.1;
        // Stop downward movement
        me.velocity_y = 0.0;
    }

    /// Provides a way to downcast to this concrete type.
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Provides mutable downcasting capability.
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    /// Update is unused for this component but required by the trait.
    fn update(&mut self, _quad: &mut Quad) {}
}
