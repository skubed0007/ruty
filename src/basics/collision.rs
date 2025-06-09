//! Collision System
//! 
//! This module provides a comprehensive collision detection and response system
//! for the Ruty game engine. It handles both point-to-point and quad-to-quad collisions
//! with support for slope physics and bounce effects.
//! 
//! # Features
//! - Point-to-point collision detection and response
//! - Quad-to-quad collision detection and response
//! - Slope physics for realistic movement
//! - Configurable bounce and friction effects
//! 
//! # Examples
//! ```rust
//! use ruty::basics::collision::Collision;
//! 
//! // Create a collision component with bounce and slope friction
//! let collision = Collision::new(0.5, 0.8);
//! ```

use crate::basics::Component;
use crate::objects::point::Point;
use crate::objects::quad::Quad;
use macroquad::prelude::*;

/// Component that handles collision detection and response
/// 
/// This component provides collision detection and response functionality
/// for both Point and Quad objects. It includes support for bounce effects
/// and slope physics.
/// 
/// # Parameters
/// * `bounce` - How bouncy the collision response should be (0.0 to 1.0)
/// * `slope_friction` - How much friction to apply on slopes (0.0 to 1.0)
#[derive(Debug, Clone)]
pub struct Collision {
    /// How bouncy the collision response should be (0.0 to 1.0)
    bounce: f32,
    /// How much friction to apply on slopes (0.0 to 1.0)
    slope_friction: f32,
}

impl Collision {
    /// Creates a new Collision component instance
    /// 
    /// # Arguments
    /// * `bounce` - The bounce coefficient (0.0 to 1.0)
    /// * `slope_friction` - The slope friction coefficient (0.0 to 1.0)
    /// 
    /// # Returns
    /// A new Collision component instance
    pub fn new(bounce: f32, slope_friction: f32) -> Self {
        Self {
            bounce: bounce.clamp(0.0, 1.0),
            slope_friction: slope_friction.clamp(0.0, 1.0),
        }
    }

    /// Calculate the normal vector of a slope between two points
    /// 
    /// # Arguments
    /// * `p1` - First point of the slope
    /// * `p2` - Second point of the slope
    /// 
    /// # Returns
    /// The normalized normal vector of the slope
    fn calculate_slope_normal(&self, p1: &Point, p2: &Point) -> Vec2 {
        let dx = p2.position.0 - p1.position.0;
        let dy = p2.position.1 - p1.position.1;
        let length = (dx * dx + dy * dy).sqrt();
        Vec2::new(-dy / length, dx / length)
    }

    /// Check if a point is on a slope
    /// 
    /// # Arguments
    /// * `point` - The point to check
    /// * `slope_p1` - First point of the slope
    /// * `slope_p2` - Second point of the slope
    /// 
    /// # Returns
    /// True if the point is on the slope, false otherwise
    fn is_on_slope(&self, point: &Point, slope_p1: &Point, slope_p2: &Point) -> bool {
        let dx = slope_p2.position.0 - slope_p1.position.0;
        let dy = slope_p2.position.1 - slope_p1.position.1;
        let length = (dx * dx + dy * dy).sqrt();
        
        // Project point onto slope line
        let t = ((point.position.0 - slope_p1.position.0) * dx + 
                (point.position.1 - slope_p1.position.1) * dy) / (length * length);
        
        // Check if projection is within slope segment
        if t < 0.0 || t > 1.0 {
            return false;
        }

        // Calculate distance from point to slope line
        let proj_x = slope_p1.position.0 + t * dx;
        let proj_y = slope_p1.position.1 + t * dy;
        let dist = ((point.position.0 - proj_x).powi(2) + 
                   (point.position.1 - proj_y).powi(2)).sqrt();

        // Point is on slope if it's close enough to the line
        dist < point.radius + 5.0
    }

    /// Apply slope physics to a point
    /// 
    /// # Arguments
    /// * `point` - The point to apply slope physics to
    /// * `slope_p1` - First point of the slope
    /// * `slope_p2` - Second point of the slope
    fn apply_slope_physics(&self, point: &mut Point, slope_p1: &Point, slope_p2: &Point) {
        let normal = self.calculate_slope_normal(slope_p1, slope_p2);
        
        // Project velocity onto slope direction
        let slope_dir = Vec2::new(slope_p2.position.0 - slope_p1.position.0,
                                slope_p2.position.1 - slope_p1.position.1).normalize();
        let vel_proj = point.velocity.0 * slope_dir.x + point.velocity.1 * slope_dir.y;
        
        // Apply slope friction
        let new_vel = slope_dir * vel_proj * self.slope_friction;
        point.velocity.0 = new_vel.x;
        point.velocity.1 = new_vel.y;
        
        // Add a small downward force to keep the point on the slope
        point.velocity.1 += 0.1;
    }
}

impl Component<Point> for Collision {
    /// Called when a collision occurs between this Point (`me`) and another Point (`other`)
    /// 
    /// # Arguments
    /// * `me` - The Point that owns this Collision component
    /// * `other` - The Point with which `me` has collided
    fn on_collide(&mut self, me: &mut Point, other: &mut Point) {
        // Calculate distance between points
        let dx = other.position.0 - me.position.0;
        let dy = other.position.1 - me.position.1;
        let distance = (dx * dx + dy * dy).sqrt();
        let min_distance = me.radius + other.radius;

        // Check for collision
        if distance < min_distance {
            // Calculate collision normal
            let nx = dx / distance;
            let ny = dy / distance;

            // Calculate relative velocity
            let relative_vel_x = other.velocity.0 - me.velocity.0;
            let relative_vel_y = other.velocity.1 - me.velocity.1;
            let relative_vel_dot_normal = relative_vel_x * nx + relative_vel_y * ny;

            // Only resolve if objects are moving toward each other
            if relative_vel_dot_normal < 0.0 {
                // Calculate impulse
                let impulse = -(1.0 + self.bounce) * relative_vel_dot_normal;
                let impulse_x = impulse * nx;
                let impulse_y = impulse * ny;

                // Apply impulse
                if !me.fixed {
                    me.velocity.0 -= impulse_x;
                    me.velocity.1 -= impulse_y;
                }
                if !other.fixed {
                    other.velocity.0 += impulse_x;
                    other.velocity.1 += impulse_y;
                }

                // Separate points
                let overlap = min_distance - distance;
                if !me.fixed {
                    me.position.0 -= nx * overlap * 0.5;
                    me.position.1 -= ny * overlap * 0.5;
                }
                if !other.fixed {
                    other.position.0 += nx * overlap * 0.5;
                    other.position.1 += ny * overlap * 0.5;
                }

                // Check for slope collision
                if self.is_on_slope(me, other, other) {
                    self.apply_slope_physics(me, other, other);
                }
                if self.is_on_slope(other, me, me) {
                    self.apply_slope_physics(other, me, me);
                }
            }
        }
    }

    /// Update is unused for this component but required by the trait
    fn update(&mut self, _point: &mut Point) {
        // No update needed for collision component
    }
}

impl Component<Quad> for Collision {
    /// Called when a collision occurs between this Quad (`me`) and another Quad (`other`)
    /// 
    /// # Arguments
    /// * `me` - The Quad that owns this Collision component
    /// * `other` - The Quad with which `me` has collided
    fn on_collide(&mut self, me: &mut Quad, other: &mut Quad) {
        // Calculate distance between quads
        let dx = other.position.0 - me.position.0;
        let dy = other.position.1 - me.position.1;
        let distance = (dx * dx + dy * dy).sqrt();
        let min_distance = (me.size.0 + other.size.0) * 0.5;

        // Check for collision
        if distance < min_distance {
            // Calculate collision normal
            let nx = dx / distance;
            let ny = dy / distance;

            // Calculate relative velocity
            let relative_vel_x = other.velocity_x - me.velocity_x;
            let relative_vel_y = other.velocity_y - me.velocity_y;
            let relative_vel_dot_normal = relative_vel_x * nx + relative_vel_y * ny;

            // Only resolve if objects are moving toward each other
            if relative_vel_dot_normal < 0.0 {
                // Calculate impulse
                let impulse = -(1.0 + self.bounce) * relative_vel_dot_normal;
                let impulse_x = impulse * nx;
                let impulse_y = impulse * ny;

                // Apply impulse
                me.velocity_x -= impulse_x;
                me.velocity_y -= impulse_y;
                other.velocity_x += impulse_x;
                other.velocity_y += impulse_y;

                // Separate quads
                let overlap = min_distance - distance;
                me.position.0 -= nx * overlap * 0.5;
                me.position.1 -= ny * overlap * 0.5;
                other.position.0 += nx * overlap * 0.5;
                other.position.1 += ny * overlap * 0.5;
            }
        }
    }

    /// Update is unused for this component but required by the trait
    fn update(&mut self, _quad: &mut Quad) {
        // No update needed for collision component
    }
}
