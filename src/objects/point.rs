use macroquad::color::Color;
use macroquad::shapes::draw_circle;
use crate::basics::Component;

/// Represents a physics point that can be connected to other points via constraints
pub struct Point {
    pub position: (f32, f32),
    pub velocity: (f32, f32),
    pub force: (f32, f32),
    pub mass: f32,
    pub radius: f32,
    pub color: Color,
    pub components: Vec<Box<dyn Component<Point>>>,
    pub fixed: bool, // Whether the point is fixed in space
}

impl Point {
    pub fn new(x: f32, y: f32, mass: f32, radius: f32, color: Color) -> Self {
        Self {
            position: (x, y),
            velocity: (0.0, 0.0),
            force: (0.0, 0.0),
            mass,
            radius,
            color,
            components: Vec::new(),
            fixed: false,
        }
    }

    pub fn draw(&self) {
        draw_circle(
            self.position.0,
            self.position.1,
            self.radius,
            self.color,
        );
    }

    pub fn add_component(&mut self, comp: Box<dyn Component<Point>>) {
        self.components.push(comp);
    }

    pub fn update_components(&mut self) {
        let mut comps = std::mem::take(&mut self.components);
        for comp in comps.iter_mut() {
            comp.update(self);
        }
        self.components = comps;
    }

    pub fn apply_force(&mut self, fx: f32, fy: f32) {
        self.force.0 += fx;
        self.force.1 += fy;
    }

    pub fn update(&mut self, dt: f32) {
        if self.fixed {
            return;
        }

        // Update velocity using forces (F = ma)
        self.velocity.0 += (self.force.0 / self.mass) * dt;
        self.velocity.1 += (self.force.1 / self.mass) * dt;

        // Update position
        self.position.0 += self.velocity.0 * dt;
        self.position.1 += self.velocity.1 * dt;

        // Reset forces
        self.force = (0.0, 0.0);
    }

    pub fn is_colliding_with(&self, other: &Point) -> bool {
        let dx = self.position.0 - other.position.0;
        let dy = self.position.1 - other.position.1;
        let distance = (dx * dx + dy * dy).sqrt();
        distance < (self.radius + other.radius)
    }

    pub fn resolve_collision(&mut self, other: &mut Point) {
        if self.fixed && other.fixed {
            return;
        }

        let dx = other.position.0 - self.position.0;
        let dy = other.position.1 - self.position.1;
        let distance = (dx * dx + dy * dy).sqrt();
        
        if distance == 0.0 {
            return;
        }

        let overlap = (self.radius + other.radius) - distance;
        if overlap <= 0.0 {
            return;
        }

        // Normalize the collision vector
        let nx = dx / distance;
        let ny = dy / distance;

        // Calculate relative velocity
        let relative_velocity_x = self.velocity.0 - other.velocity.0;
        let relative_velocity_y = self.velocity.1 - other.velocity.1;

        // Calculate relative velocity in terms of the normal direction
        let velocity_along_normal = relative_velocity_x * nx + relative_velocity_y * ny;

        // Do not resolve if objects are moving apart
        if velocity_along_normal > 0.0 {
            return;
        }

        // Calculate restitution (bounciness)
        let restitution = 0.8;

        // Calculate impulse scalar
        let impulse_scalar = -(1.0 + restitution) * velocity_along_normal;
        let impulse_scalar = impulse_scalar / (1.0/self.mass + 1.0/other.mass);

        // Apply impulse
        let impulse_x = impulse_scalar * nx;
        let impulse_y = impulse_scalar * ny;

        if !self.fixed {
            self.velocity.0 -= impulse_x / self.mass;
            self.velocity.1 -= impulse_y / self.mass;
        }
        if !other.fixed {
            other.velocity.0 += impulse_x / other.mass;
            other.velocity.1 += impulse_y / other.mass;
        }

        // Positional correction to prevent sinking
        let percent = 0.2;
        let correction = (overlap / distance) * percent;
        let correction_x = nx * correction;
        let correction_y = ny * correction;

        if !self.fixed {
            self.position.0 -= correction_x;
            self.position.1 -= correction_y;
        }
        if !other.fixed {
            other.position.0 += correction_x;
            other.position.1 += correction_y;
        }
    }
} 