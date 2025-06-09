use crate::objects::point::Point;
use macroquad::shapes::draw_line;

/// Represents a distance constraint between two points
pub struct Constraint {
    pub point1: usize,  // Index of first point
    pub point2: usize,  // Index of second point
    pub rest_length: f32,  // The desired distance between points
    pub stiffness: f32,    // How rigid the constraint is (0-1)
    pub color: macroquad::color::Color,
}

impl Constraint {
    pub fn new(point1: usize, point2: usize, rest_length: f32, stiffness: f32, color: macroquad::color::Color) -> Self {
        Self {
            point1,
            point2,
            rest_length,
            stiffness,
            color,
        }
    }

    pub fn draw(&self, points: &[Point]) {
        if let (Some(p1), Some(p2)) = (points.get(self.point1), points.get(self.point2)) {
            draw_line(
                p1.position.0,
                p1.position.1,
                p2.position.0,
                p2.position.1,
                2.0,
                self.color,
            );
        }
    }

    pub fn solve(&self, points: &mut [Point]) {
        // Get mutable references to both points
        let (p1, p2) = if self.point1 < self.point2 {
            let (left, right) = points.split_at_mut(self.point2);
            (&mut left[self.point1], &mut right[0])
        } else {
            let (left, right) = points.split_at_mut(self.point1);
            (&mut right[0], &mut left[self.point2])
        };

        let dx = p2.position.0 - p1.position.0;
        let dy = p2.position.1 - p1.position.1;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance == 0.0 {
            return;
        }

        // Calculate the difference from the rest length
        let diff = (distance - self.rest_length) / distance;

        // Calculate the correction vector
        let correction_x = dx * diff * self.stiffness;
        let correction_y = dy * diff * self.stiffness;

        // Apply the correction based on mass ratios
        let total_mass = p1.mass + p2.mass;
        let ratio1 = p2.mass / total_mass;
        let ratio2 = p1.mass / total_mass;

        if !p1.fixed {
            p1.position.0 += correction_x * ratio1;
            p1.position.1 += correction_y * ratio1;
        }
        if !p2.fixed {
            p2.position.0 -= correction_x * ratio2;
            p2.position.1 -= correction_y * ratio2;
        }
    }
} 