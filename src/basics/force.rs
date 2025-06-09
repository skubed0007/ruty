use crate::basics::Component;
use crate::objects::point::Point;
use crate::objects::quad::Quad;
use macroquad::prelude::*;

/// A component representing a force applied to a Quad, affecting its velocity.
///
/// This component adds a force vector `(x, y)` to the Quad's velocity each update.
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
    /// The force vector to apply
    pub force: Vec2,
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
    pub fn new(force: Vec2) -> Self {
        Self { force }
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
        Self { force: Vec2::new(x, y) }
    }
}

impl Component<Point> for Force {
    /// Applies the force to the Point's velocity each update.
    ///
    /// Adds `x` and `y` components to `velocity.0` and `velocity.1` respectively.
    /// If the force is set to decay, it resets the force to zero after applying.
    ///
    /// # Parameters
    /// - `point`: The Point instance to update.
    fn update(&mut self, point: &mut Point) {
        if !point.fixed {
            point.velocity.0 += self.force.x;
            point.velocity.1 += self.force.y;
        }
    }

    fn on_collide(&mut self, _me: &mut Point, _other: &mut Point) {
        // No collision handling needed for force
    }
}

impl Component<Quad> for Force {
    /// Applies the force to the Quad's velocity each update.
    ///
    /// Adds `x` and `y` components to `velocity_x` and `velocity_y` respectively.
    /// If the force is set to decay, it resets the force to zero after applying.
    ///
    /// # Parameters
    /// - `quad`: The Quad instance to update.
    fn update(&mut self, quad: &mut Quad) {
        quad.velocity_x += self.force.x;
        quad.velocity_y += self.force.y;
    }

    fn on_collide(&mut self, _me: &mut Quad, _other: &mut Quad) {
        // No collision handling needed for force
    }
}
