use crate::basics::Component;
use crate::objects::point::Point;
use crate::objects::quad::Quad;

/// Component that applies friction to an object's movement
pub struct Friction {
    /// The friction coefficient (0.0 to 1.0)
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

impl Component<Point> for Friction {
    /// Updates the Point's velocity by applying friction.
    ///
    /// Multiplies `velocity.0` and `velocity.1` by the friction coefficient each frame,
    /// gradually reducing speed until it stops when below a small threshold.
    fn update(&mut self, point: &mut Point) {
        if !point.fixed {
            point.velocity.0 *= self.coefficient;
            point.velocity.1 *= self.coefficient;
        }
    }

    fn on_collide(&mut self, _me: &mut Point, _other: &mut Point) {
        // No collision handling needed for friction
    }
}

impl Component<Quad> for Friction {
    /// Updates the Quad's horizontal and vertical velocities by applying friction.
    ///
    /// Multiplies `velocity_x` and `velocity_y` by the friction coefficient each frame,
    /// gradually reducing speed until it stops when below a small threshold.
    fn update(&mut self, quad: &mut Quad) {
        quad.velocity_x *= self.coefficient;
        quad.velocity_y *= self.coefficient;
    }

    fn on_collide(&mut self, _me: &mut Quad, _other: &mut Quad) {
        // No collision handling needed for friction
    }
}
