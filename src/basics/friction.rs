use std::any::Any;

use crate::{basics::Component, objects::quad::Quad};

/// A component that applies friction to reduce the horizontal velocity of a Quad.
///
/// This simulates a frictional force slowing down horizontal movement over time.
/// The friction coefficient should be a value between 0.0 and 1.0, where values
/// closer to 0 cause more rapid slowing.
///
/// # Fields
///
/// - `coefficient`: The multiplier applied to horizontal velocity every update.
///
/// # Example
///
/// ```rust
/// let friction = Friction::new(0.85);
/// quad.add_component(Box::new(friction));
/// ```
pub struct Friction {
    pub coefficient: f32,
}

impl Friction {
    /// Creates a new Friction component with the given coefficient.
    ///
    /// # Parameters
    /// - `coefficient`: A value (usually < 1) that slows horizontal velocity.
    ///
    /// # Returns
    /// A new `Friction` instance.
    pub fn new(coefficient: f32) -> Self {
        Self { coefficient }
    }
}

impl Component for Friction {
    /// Updates the Quad’s horizontal velocity by applying friction.
    ///
    /// Multiplies `velocity_x` by the friction coefficient each frame,
    /// gradually reducing speed until it stops when below a small threshold.
    fn update(&mut self, quad: &mut Quad) {
        quad.velocity_x *= self.coefficient;
        if quad.velocity_x.abs() < 0.01 {
            quad.velocity_x = 0.0;
        }
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
