use crate::objects::point::Point;
use crate::objects::constraint::Constraint;
use crate::basics::gravity::Gravity;
use crate::basics::friction::Friction;
use crate::basics::collision::Collision;
use macroquad::prelude::*;

/// Configuration for creating shapes
#[derive(Clone, Copy)]
pub struct ShapeConfig {
    /// Color of the shape
    pub color: Color,
    /// Gravity force applied to the shape
    pub gravity: f32,
    /// Friction coefficient (0.0 to 1.0)
    pub friction: f32,
    /// Bounce coefficient (0.0 to 1.0)
    pub bounce: f32,
    /// Slope friction coefficient (0.0 to 1.0)
    pub slope_friction: f32,
    /// Point radius for collision detection
    pub point_radius: f32,
    /// Point mass
    pub point_mass: f32,
    /// Constraint stiffness (0.0 to 1.0)
    pub constraint_stiffness: f32,
    /// Whether the shape is fixed in place
    pub fixed: bool,
}

impl Default for ShapeConfig {
    fn default() -> Self {
        Self {
            color: WHITE,
            gravity: 10.0,
            friction: 0.95,
            bounce: 0.2,
            slope_friction: 0.85,
            point_radius: 15.0,
            point_mass: 1.0,
            constraint_stiffness: 0.95,
            fixed: false,
        }
    }
}

/// Creates a triangle with the given center and size point
/// size_point determines the distance and angle of the first vertex from center
pub fn create_triangle(center: Vec2, size_point: Vec2, config: ShapeConfig) -> (Vec<Point>, Vec<Constraint>) {
    let mut points = Vec::new();
    let mut constraints = Vec::new();
    
    // Calculate radius and angle from center to size point
    let radius = (size_point - center).length();
    let angle = (size_point - center).y.atan2((size_point - center).x);
    
    // Create three points in a triangle
    for i in 0..3 {
        let vertex_angle = angle + (i as f32 * 2.0 * std::f32::consts::PI) / 3.0;
        let x = center.x + radius * vertex_angle.cos();
        let y = center.y + radius * vertex_angle.sin();
        let mut point = Point::new(x, y, config.point_mass, config.point_radius, config.color);
        point.fixed = config.fixed;
        points.push(point);
    }
    
    // Create edge constraints
    for i in 0..3 {
        let next = (i + 1) % 3;
        let distance = ((points[i].position.0 - points[next].position.0).powi(2) + 
                       (points[i].position.1 - points[next].position.1).powi(2)).sqrt();
        constraints.push(Constraint::new(i, next, distance, config.constraint_stiffness, config.color));
    }
    
    // Add physics components
    for point in points.iter_mut() {
        point.add_component(Box::new(Gravity::new(config.gravity)));
        point.add_component(Box::new(Friction::new(config.friction)));
        point.add_component(Box::new(Collision::new(config.bounce, config.slope_friction)));
    }
    
    (points, constraints)
}

/// Creates a square with the given center and size point
/// size_point determines the distance and angle of the first vertex from center
pub fn create_square(center: Vec2, size_point: Vec2, config: ShapeConfig) -> (Vec<Point>, Vec<Constraint>) {
    let mut points = Vec::new();
    let mut constraints = Vec::new();
    
    // Calculate radius and angle from center to size point
    let radius = (size_point - center).length();
    let angle = (size_point - center).y.atan2((size_point - center).x);
    
    // Create four points in a square
    for i in 0..4 {
        let vertex_angle = angle + (i as f32 * std::f32::consts::PI) / 2.0;
        let x = center.x + radius * vertex_angle.cos();
        let y = center.y + radius * vertex_angle.sin();
        let mut point = Point::new(x, y, config.point_mass, config.point_radius, config.color);
        point.fixed = config.fixed;
        points.push(point);
    }
    
    // Create edge constraints
    for i in 0..4 {
        let next = (i + 1) % 4;
        let distance = ((points[i].position.0 - points[next].position.0).powi(2) + 
                       (points[i].position.1 - points[next].position.1).powi(2)).sqrt();
        constraints.push(Constraint::new(i, next, distance, config.constraint_stiffness, config.color));
    }
    
    // Add cross-bracing for stability
    let diagonal = radius * 2.0_f32.sqrt();
    constraints.push(Constraint::new(0, 2, diagonal, config.constraint_stiffness, config.color));
    constraints.push(Constraint::new(1, 3, diagonal, config.constraint_stiffness, config.color));
    
    // Add physics components
    for point in points.iter_mut() {
        point.add_component(Box::new(Gravity::new(config.gravity)));
        point.add_component(Box::new(Friction::new(config.friction)));
        point.add_component(Box::new(Collision::new(config.bounce, config.slope_friction)));
    }
    
    (points, constraints)
}

/// Creates a circle with the given center and size point
/// size_point determines the radius of the circle
pub fn create_circle(center: Vec2, size_point: Vec2, num_points: usize, config: ShapeConfig) -> (Vec<Point>, Vec<Constraint>) {
    let mut points = Vec::new();
    let mut constraints = Vec::new();
    
    // Calculate radius from center to size point
    let radius = (size_point - center).length();
    
    // Create points around the circle
    for i in 0..num_points {
        let angle = (i as f32 * 2.0 * std::f32::consts::PI) / (num_points as f32);
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        let mut point = Point::new(x, y, config.point_mass, config.point_radius, config.color);
        point.fixed = config.fixed;
        points.push(point);
    }
    
    // Create edge constraints
    for i in 0..num_points {
        let next = (i + 1) % num_points;
        let distance = ((points[i].position.0 - points[next].position.0).powi(2) + 
                       (points[i].position.1 - points[next].position.1).powi(2)).sqrt();
        constraints.push(Constraint::new(i, next, distance, config.constraint_stiffness, config.color));
    }
    
    // Add cross-bracing for stability
    for i in 0..num_points {
        let opposite = (i + num_points/2) % num_points;
        let distance = ((points[i].position.0 - points[opposite].position.0).powi(2) + 
                       (points[i].position.1 - points[opposite].position.1).powi(2)).sqrt();
        constraints.push(Constraint::new(i, opposite, distance, config.constraint_stiffness, config.color));
    }
    
    // Add physics components
    for point in points.iter_mut() {
        point.add_component(Box::new(Gravity::new(config.gravity)));
        point.add_component(Box::new(Friction::new(config.friction)));
        point.add_component(Box::new(Collision::new(config.bounce, config.slope_friction)));
    }
    
    (points, constraints)
}

/// Creates a line segment between two points
pub fn create_line(start: Vec2, end: Vec2, num_points: usize, config: ShapeConfig) -> (Vec<Point>, Vec<Constraint>) {
    let mut points = Vec::new();
    let mut constraints = Vec::new();
    
    // Create points along the line
    for i in 0..num_points {
        let t = i as f32 / (num_points - 1) as f32;
        let x = start.x + (end.x - start.x) * t;
        let y = start.y + (end.y - start.y) * t;
        let mut point = Point::new(x, y, config.point_mass, config.point_radius, config.color);
        point.fixed = config.fixed;
        points.push(point);
    }
    
    // Create constraints between adjacent points
    for i in 0..num_points - 1 {
        let distance = ((points[i].position.0 - points[i + 1].position.0).powi(2) + 
                       (points[i].position.1 - points[i + 1].position.1).powi(2)).sqrt();
        constraints.push(Constraint::new(i, i + 1, distance, config.constraint_stiffness, config.color));
    }
    
    // Add physics components if not fixed
    if !config.fixed {
        for point in points.iter_mut() {
            point.add_component(Box::new(Gravity::new(config.gravity)));
            point.add_component(Box::new(Friction::new(config.friction)));
            point.add_component(Box::new(Collision::new(config.bounce, config.slope_friction)));
        }
    }
    
    (points, constraints)
} 