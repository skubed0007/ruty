use std::any::Any;

use crate::{basics::Component, objects::quad::Quad};

/// A component that applies a constant downward force (gravity) to a Quad.
///
/// This component adds gravity by increasing the vertical velocity (`velocity_y`)
/// of the `Quad` every frame by the specified force value.
///
/// # Fields
///
/// - `force`: The acceleration applied downwards each update (e.g., 0.5).
///
/// # Example
///
/// ```rust
/// let gravity = Gravity::new(0.5);
/// quad.add_component(Box::new(gravity));
/// ```
pub struct Gravity {
    pub force: f32,
}

impl Gravity {
    /// Creates a new Gravity component with the specified force.
    ///
    /// # Parameters
    /// - `force`: The gravity acceleration to apply per update frame.
    ///
    /// # Returns
    /// A new `Gravity` instance.
    pub fn new(force: f32) -> Self {
        Self { force }
    }
}

impl Component for Gravity {
    /// Updates the Quad's velocity by adding the gravity force to its vertical velocity.
    ///
    /// This simulates gravity pulling the Quad downward every frame.
    fn update(&mut self, quad: &mut Quad) {
        quad.velocity_y += self.force;
    }

    /// Allows downcasting to retrieve a reference to the concrete type.
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Allows downcasting to retrieve a mutable reference to the concrete type.
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
