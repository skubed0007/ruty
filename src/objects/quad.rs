use macroquad::color::Color;
use macroquad::shapes::draw_rectangle;

use crate::basics::Component;

/// Represents a basic rectangular game object in 2D space.
///
/// This struct holds position, size, color, velocity, and a list of
/// dynamic components that can modify the object's behavior, similar
/// to Unity's component-based architecture.
///
/// Components can include physics (gravity, friction), input handling,
/// rendering, collision detection, etc.
///
/// # Fields
///
/// - `position`: The (x, y) coordinates of the top-left corner of the Quad.
/// - `size`: Width and height of the Quad.
/// - `color`: Color used to draw the Quad.
/// - `velocity_x`, `velocity_y`: Current velocity components along the X and Y axes.
/// - `components`: A vector of boxed components attached to this Quad.
///
/// # Example
///
/// ```rust
/// let mut quad = Quad::new(100.0, 200.0, 50.0, 50.0, WHITE);
/// quad.add_component(Box::new(Gravity::new(0.5)));
/// ```
pub struct Quad {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub color: Color,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub components: Vec<Box<dyn Component<Quad>>>,
}

impl Quad {
    /// Creates a new Quad with specified position, size, and color.
    ///
    /// Velocity starts at zero and no components are attached by default.
    ///
    /// # Parameters
    /// - `x`, `y`: Starting position of the Quad's top-left corner.
    /// - `w`, `h`: Width and height of the Quad.
    /// - `color`: The color used when drawing the Quad.
    ///
    /// # Returns
    /// A new `Quad` instance.
    pub fn new(x: f32, y: f32, w: f32, h: f32, color: Color) -> Self {
        Self {
            position: (x, y),
            size: (w, h),
            color,
            velocity_x: 0.0,
            velocity_y: 0.0,
            components: Vec::new(),
        }
    }

    /// Draws the Quad on the screen using macroquad's rectangle drawing.
    ///
    /// This is a basic render method — you can extend it later to support sprites,
    /// animations, or GUI elements.
    pub fn draw(&self) {
        draw_rectangle(
            self.position.0,
            self.position.1,
            self.size.0,
            self.size.1,
            self.color,
        );
    }

    /// Adds a component to the Quad's component list.
    ///
    /// Components implement custom behavior, such as physics or input handling.
    /// They are stored as trait objects (`Box<dyn Component>`) for dynamic dispatch.
    ///
    /// # Parameters
    /// - `comp`: A boxed component to add.
    pub fn add_component(&mut self, comp: Box<dyn Component<Quad>>) {
        self.components.push(comp);
    }

    /// Updates all attached components by calling their `update` method.
    ///
    /// This allows components to modify the Quad's state (e.g., velocity, position)
    /// each frame.
    ///
    /// Usually called once per game loop iteration.
    pub fn update_components(&mut self) {
        let mut comps = std::mem::take(&mut self.components);
        for comp in comps.iter_mut() {
            comp.update(self);
        }
        self.components = comps;
    }

    /// Checks if this Quad is colliding with another Quad.
    ///
    /// Collision detection is based on Axis-Aligned Bounding Box (AABB) overlap.
    ///
    /// # Parameters
    /// - `other`: The other Quad to check collision against.
    ///
    /// # Returns
    /// `true` if the bounding boxes overlap, else `false`.
    pub fn is_colliding_with(&self, other: &Quad) -> bool {
        let (x1, y1, w1, h1) = (self.position.0, self.position.1, self.size.0, self.size.1);
        let (x2, y2, w2, h2) = (
            other.position.0,
            other.position.1,
            other.size.0,
            other.size.1,
        );

        x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2
    }

    /// Removes a component of a specific type from the Quad.
    ///
    /// Uses Rust's type system and `TypeId` to identify the component to remove.
    ///
    /// # Type Parameters
    /// - `T`: The component type to remove.
    pub fn remove_component<T: 'static>(&mut self) {
        self.components.retain(|comp| {
            std::any::TypeId::of::<T>() != std::any::TypeId::of::<Box<dyn Component<Quad>>>()
        });
    }
}
