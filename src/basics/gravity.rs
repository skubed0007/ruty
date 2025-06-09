use crate::basics::Component;
use crate::objects::point::Point;
use crate::objects::quad::Quad;

/// Component that applies gravity to an object
pub struct Gravity {
    /// The strength of gravity (positive values pull downward)
    pub strength: f32,
}

impl Gravity {
    /// Creates a new Gravity component with the specified strength.
    ///
    /// # Parameters
    /// - `strength`: The gravity acceleration to apply per update frame.
    ///
    /// # Returns
    /// A new `Gravity` instance.
    pub fn new(strength: f32) -> Self {
        Self { strength }
    }
}

impl Component<Point> for Gravity {
    /// Updates the Point's velocity by adding the gravity force to its vertical velocity.
    ///
    /// This simulates gravity pulling the Point downward every frame.
    fn update(&mut self, point: &mut Point) {
        if !point.fixed {
            point.velocity.1 += self.strength;
        }
    }

    /// No collision handling needed for gravity
    fn on_collide(&mut self, _me: &mut Point, _other: &mut Point) {
        // No collision handling needed for gravity
    }
}

impl Component<Quad> for Gravity {
    /// Updates the Quad's velocity by adding the gravity force to its vertical velocity.
    ///
    /// This simulates gravity pulling the Quad downward every frame.
    fn update(&mut self, quad: &mut Quad) {
        quad.velocity_y += self.strength;
    }

    /// No collision handling needed for gravity
    fn on_collide(&mut self, _me: &mut Quad, _other: &mut Quad) {
        // No collision handling needed for gravity
    }
}
