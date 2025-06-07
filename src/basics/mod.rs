use std::any::Any;

use crate::objects::quad::Quad;

pub trait Component: Any {
    fn update(&mut self, quad: &mut Quad);
    fn on_collide(&mut self, _me: &mut Quad, _other: &Quad) {}
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
pub mod collision;
pub mod force;
pub mod friction;
pub mod gravity;
