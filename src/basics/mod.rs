use crate::objects::point::Point;
use crate::objects::quad::Quad;

/// Trait for components that can be attached to objects
pub trait Component<T> {
    /// Update the component's state
    fn update(&mut self, object: &mut T);
    
    /// Handle collision with another object
    fn on_collide(&mut self, me: &mut T, other: &mut T);
}

/// Trait for components that can be drawn
pub trait Drawable {
    /// Draw the component
    fn draw(&self);
}

pub mod collision;
pub mod force;
pub mod friction;
pub mod gravity;
