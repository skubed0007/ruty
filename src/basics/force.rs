use std::any::Any;

use crate::{basics::Component, objects::quad::Quad};

/// A component representing a force applied to a Quad, affecting its velocity.
///
/// This component adds a force vector `(x, y)` to the Quad’s velocity each update.
/// It supports both *decaying* (one-time) forces and *permanent* forces that persist.
///
/// # Fields
///
/// - `x`: The horizontal force to apply.
/// - `y`: The vertical force to apply.
/// - `decay`: If `true`, the force is applied once and then reset to zero (default).
///   If `false`, the force remains applied every update.
///
/// # Usage
///
/// To create a one-time force (like a jump impulse):
/// ```
/// let jump_force = Force::new(0.0, -10.0);
/// ```
///
/// To create a continuous force (like constant wind pushing right):
/// ```
/// let wind_force = Force::permanent(1.0, 0.0);
/// ```
pub struct Force {
    pub x: f32,
    pub y: f32,
    pub decay: bool, // Whether force resets each update (one-time) or stays constant
}

impl Force {
    /// Creates a new decaying (one-time) force with the given x and y components.
    ///
    /// After applying the force once, it resets to zero automatically.
    ///
    /// # Parameters
    /// - `x`: Horizontal force amount.
    /// - `y`: Vertical force amount.
    ///
    /// # Returns
    /// A `Force` instance with `decay` set to `true`.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, decay: true }
    }

    /// Creates a new permanent force with the given x and y components.
    ///
    /// This force will be applied every update without resetting.
    ///
    /// # Parameters
    /// - `x`: Horizontal force amount.
    /// - `y`: Vertical force amount.
    ///
    /// # Returns
    /// A `Force` instance with `decay` set to `false`.
    pub fn permanent(x: f32, y: f32) -> Self {
        Self { x, y, decay: false }
    }
}

impl Component for Force {
    /// Applies the force to the Quad’s velocity each update.
    ///
    /// Adds `x` and `y` components to `velocity_x` and `velocity_y` respectively.
    /// If the force is set to decay, it resets the force to zero after applying.
    ///
    /// # Parameters
    /// - `quad`: The Quad instance to update.
    fn update(&mut self, quad: &mut Quad) {
        quad.velocity_x += self.x;
        quad.velocity_y += self.y;
        if self.decay {
            self.x = 0.0;
            self.y = 0.0;
        }
    }

    /// Enables downcasting to access the concrete `Force` type.
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Enables mutable downcasting to access and modify the concrete `Force` type.
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
