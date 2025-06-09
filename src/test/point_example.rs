use macroquad::prelude::*;
use crate::basics::collision::Collision;
use crate::objects::point::Point;
use crate::objects::constraint::Constraint;
use crate::basics::gravity::Gravity;
use crate::basics::force::Force;
use crate::basics::friction::Friction;

// Helper function to create a regular polygon
fn create_polygon(center_x: f32, center_y: f32, radius: f32, sides: usize, color: Color, gravity: f32, is_pentagon: bool) -> (Vec<Point>, Vec<Constraint>) {
    let mut points = Vec::new();
    let mut constraints = Vec::new();
    
    println!("Creating polygon at ({}, {}) with {} sides", center_x, center_y, sides);
    
    // Create points with larger radius for better collision detection
    for i in 0..sides {
        let angle = (i as f32 * 2.0 * std::f32::consts::PI) / (sides as f32);
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        points.push(Point::new(x, y, 1.0, 15.0, color));
        println!("Created point {} at ({}, {})", i, x, y);
    }
    
    // Create constraints between adjacent points with higher stiffness
    for i in 0..sides {
        let next = (i + 1) % sides;
        let distance = ((points[i].position.0 - points[next].position.0).powi(2) + 
                       (points[i].position.1 - points[next].position.1).powi(2)).sqrt();
        // Use very high stiffness for all shapes to prevent deformation
        let stiffness = if is_pentagon { 0.98 } else { 0.97 };
        constraints.push(Constraint::new(i, next, distance, stiffness, color));
        println!("Created edge constraint between points {} and {} with distance {}", i, next, distance);
    }
    
    // Add cross-bracing for stability with higher stiffness
    if sides > 3 {
        for i in 0..sides {
            let opposite = (i + sides/2) % sides;
            let distance = ((points[i].position.0 - points[opposite].position.0).powi(2) + 
                          (points[i].position.1 - points[opposite].position.1).powi(2)).sqrt();
            // Use very high stiffness for cross-braces
            let stiffness = if is_pentagon { 0.96 } else { 0.95 };
            constraints.push(Constraint::new(i, opposite, distance, stiffness, color));
            println!("Created cross-brace constraint between points {} and {} with distance {}", i, opposite, distance);
        }
    }
    
    // Add gravity component to all points
    for point in points.iter_mut() {
        point.add_component(Box::new(Gravity::new(gravity)));
        // Use very high friction for all shapes
        let friction = if is_pentagon { 0.95 } else { 0.97 };
        point.add_component(Box::new(Friction::new(friction)));
        println!("Added gravity and friction to point at ({}, {})", point.position.0, point.position.1);
    }
    
    (points, constraints)
}

// Helper function to create a slope
fn create_slope(start_x: f32, start_y: f32, end_x: f32, end_y: f32, spacing: f32, color: Color) -> (Vec<Point>, Vec<Constraint>) {
    let mut points = Vec::new();
    let mut constraints = Vec::new();
    
    let dx = end_x - start_x;
    let dy = end_y - start_y;
    let length = (dx * dx + dy * dy).sqrt();
    let steps = (length / spacing).ceil() as i32;
    
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let x = start_x + dx * t;
        let y = start_y + dy * t;
        points.push(Point::new(x, y, 0.0, 12.0, color));
    }
    
    // Connect points with constraints
    for i in 0..points.len() - 1 {
        let distance = spacing;
        constraints.push(Constraint::new(i, i + 1, distance, 0.99, color));
    }
    
    // Make all points fixed
    for point in points.iter_mut() {
        point.fixed = true;
        point.add_component(Box::new(Friction::new(0.85))); // Lower friction for sliding
    }
    
    (points, constraints)
}

// Helper function to create a rounded object (circle)
fn create_circle(center_x: f32, center_y: f32, radius: f32, num_points: usize, color: Color, gravity: f32) -> (Vec<Point>, Vec<Constraint>) {
    let mut points = Vec::new();
    let mut constraints = Vec::new();
    
    // Create points around the circle
    for i in 0..num_points {
        let angle = (i as f32 * 2.0 * std::f32::consts::PI) / (num_points as f32);
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        points.push(Point::new(x, y, 1.0, 12.0, color));
    }
    
    // Connect adjacent points
    for i in 0..num_points {
        let next = (i + 1) % num_points;
        let distance = ((points[i].position.0 - points[next].position.0).powi(2) + 
                       (points[i].position.1 - points[next].position.1).powi(2)).sqrt();
        constraints.push(Constraint::new(i, next, distance, 0.98, color));
    }
    
    // Add cross-bracing for stability
    for i in 0..num_points {
        let opposite = (i + num_points/2) % num_points;
        let distance = ((points[i].position.0 - points[opposite].position.0).powi(2) + 
                       (points[i].position.1 - points[opposite].position.1).powi(2)).sqrt();
        constraints.push(Constraint::new(i, opposite, distance, 0.96, color));
    }
    
    // Add gravity and friction
    for point in points.iter_mut() {
        point.add_component(Box::new(Gravity::new(gravity)));
        point.add_component(Box::new(Friction::new(0.92))); // Lower friction for rolling
    }
    
    (points, constraints)
}

pub async fn run_point_example() {
    let mut all_points = Vec::new();
    let mut all_constraints = Vec::new();
    
    println!("\n=== Creating Shapes ===");
    
    // Create a circle of shapes
    let center_x = 400.0;
    let center_y = 200.0;
    let circle_radius = 150.0;
    let num_shapes = 8;
    
    // Define different shapes to create
    let shape_types = vec![
        (30.0, 4, RED, 50.0),     // Square - reduced gravity
        (25.0, 3, BLUE, 30.0),    // Triangle - reduced gravity
        (35.0, 5, GREEN, 10.0),   // Pentagon - reduced gravity
        (20.0, 6, YELLOW, 40.0),  // Hexagon - reduced gravity
    ];
    
    for i in 0..num_shapes {
        let angle = (i as f32 * 2.0 * std::f32::consts::PI) / (num_shapes as f32);
        let x = center_x + circle_radius * angle.cos();
        let y = center_y + circle_radius * angle.sin();
        
        // Cycle through shape types
        let (radius, sides, color, gravity) = shape_types[i % shape_types.len()];
        let is_pentagon = sides == 5;
        
        println!("\nCreating shape {} at ({}, {})", i, x, y);
        let (mut points, mut constraints) = create_polygon(x, y, radius, sides, color, gravity, is_pentagon);
        
        for p in points.iter_mut() {
            p.add_component(Box::new(Collision::new(0.5, 0.8)));
            println!("Added collision component to point at ({}, {})", p.position.0, p.position.1);
        }
        all_points.append(&mut points);
        all_constraints.append(&mut constraints);
    }
    
    // Add some rounded objects
    let circles = vec![
        (200.0, 300.0, 40.0, 16, PURPLE, 20.0),  // Large circle
        (600.0, 300.0, 30.0, 12, ORANGE, 15.0),  // Medium circle
    ];
    
    for (x, y, radius, points, color, gravity) in circles {
        let (mut circle_points, mut circle_constraints) = create_circle(x, y, radius, points, color, gravity);
        for p in circle_points.iter_mut() {
            p.add_component(Box::new(Collision::new(0.5, 0.8)));
        }
        all_points.append(&mut circle_points);
        all_constraints.append(&mut circle_constraints);
    }
    
    println!("\n=== Creating Ground and Slopes ===");
    
    // Create a single point that will slide
    let mut sliding_point = Point::new(100.0, 100.0, 1.0, 20.0, RED); // Much larger radius
    sliding_point.add_component(Box::new(Gravity::new(5.0))); // Much lower gravity
    sliding_point.add_component(Box::new(Friction::new(0.99))); // Very high friction
    sliding_point.add_component(Box::new(Collision::new(0.0, 0.0))); // Reduced slope friction
    all_points.push(sliding_point);
    
    // Create a series of connected slopes
    let ground_y = 500.0;
    let slopes = vec![
        // Start with a gentle slope
        (100.0, ground_y, 300.0, ground_y - 50.0, 8.0, BLUE), // Much tighter spacing
        // Steeper slope
        (300.0, ground_y - 50.0, 500.0, ground_y - 150.0, 8.0, GREEN),
        // Gentle curve
        (500.0, ground_y - 150.0, 700.0, ground_y - 100.0, 8.0, YELLOW),
        // Final gentle slope
        (700.0, ground_y - 100.0, 900.0, ground_y, 8.0, PURPLE),
    ];
    
    for (start_x, start_y, end_x, end_y, spacing, color) in slopes {
        let (mut slope_points, mut slope_constraints) = create_slope(start_x, start_y, end_x, end_y, spacing, color);
        for p in slope_points.iter_mut() {
            p.fixed = true;
            p.add_component(Box::new(Collision::new(0.0, 0.85))); // Reduced slope friction
        }
        all_points.extend(slope_points);
        all_constraints.extend(slope_constraints);
    }

    println!("\n=== Starting Simulation ===");
    let mut frame_count = 0;
    let mut last_collision_print = 0;

    loop {
        clear_background(BLACK);
        frame_count += 1;

        // Update all points
        for point in all_points.iter_mut() {
            point.update_components();
            point.update(1.0/60.0);
        }

        // Solve constraints multiple times for stability
        for _ in 0..8 {
            for constraint in all_constraints.iter() {
                constraint.solve(&mut all_points);
            }
        }

        // Check collisions between all points with debug prints
        let mut collision_count = 0;
        for i in 0..all_points.len() {
            let (left, right) = all_points.split_at_mut(i + 1);
            let point_i = &mut left[i];
            for point_j in right.iter_mut() {
                let dx = point_i.position.0 - point_j.position.0;
                let dy = point_i.position.1 - point_j.position.1;
                let distance = (dx * dx + dy * dy).sqrt();
                let min_distance = point_i.radius + point_j.radius;
                
                // More conservative collision detection
                if distance < min_distance * 1.05 {
                    collision_count += 1;
                    
                    // Print collision info every 30 frames
                    if frame_count - last_collision_print >= 30 {
                        println!("\n=== Collision Detected ===");
                        println!("Frame: {}", frame_count);
                        println!("Point 1: pos=({:.1}, {:.1}), vel=({:.1}, {:.1}), radius={:.1}, fixed={}", 
                            point_i.position.0, point_i.position.1,
                            point_i.velocity.0, point_i.velocity.1,
                            point_i.radius, point_i.fixed);
                        println!("Point 2: pos=({:.1}, {:.1}), vel=({:.1}, {:.1}), radius={:.1}, fixed={}", 
                            point_j.position.0, point_j.position.1,
                            point_j.velocity.0, point_j.velocity.1,
                            point_j.radius, point_j.fixed);
                        println!("Distance: {:.1}, Min distance: {:.1}", distance, min_distance);
                        println!("=== End Collision Info ===\n");
                        last_collision_print = frame_count;
                    }

                    // Force separation if points are too close
                    if distance < min_distance * 0.95 {
                        let nx = dx / distance;
                        let ny = dy / distance;
                        let push = (min_distance - distance) * 0.2;
                        
                        if !point_i.fixed {
                            point_i.position.0 += nx * push;
                            point_i.position.1 += ny * push;
                            // Much more aggressive damping
                            let damping = if point_i.velocity.0.abs() > 3.0 || point_i.velocity.1.abs() > 3.0 { 0.3 } else { 0.5 };
                            point_i.velocity.0 *= damping;
                            point_i.velocity.1 *= damping;
                        }
                        if !point_j.fixed {
                            point_j.position.0 -= nx * push;
                            point_j.position.1 -= ny * push;
                            // Much more aggressive damping
                            let damping = if point_j.velocity.0.abs() > 3.0 || point_j.velocity.1.abs() > 3.0 { 0.3 } else { 0.5 };
                            point_j.velocity.0 *= damping;
                            point_j.velocity.1 *= damping;
                        }
                    }

                    point_i.resolve_collision(point_j);
                }
            }
        }

        // Draw all constraints
        for constraint in all_constraints.iter() {
            constraint.draw(&all_points);
        }

        // Draw all points
        for point in all_points.iter() {
            point.draw();
        }

        // Add interactivity
        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            // Apply a force to points near the mouse
            for point in all_points.iter_mut() {
                let dx = mouse_x - point.position.0;
                let dy = mouse_y - point.position.1;
                let distance = (dx * dx + dy * dy).sqrt();
                if distance < 100.0 {
                    let force = Force::new(Vec2::new(dx * 0.1, dy * 0.1));
                    point.add_component(Box::new(force));
                }
            }
        }

        // Draw instructions and collision count
        draw_text(
            &format!("Click and drag to interact with the shapes\nCollisions this frame: {}", collision_count),
            10.0,
            20.0,
            20.0,
            WHITE,
        );

        next_frame().await;
    }
} 