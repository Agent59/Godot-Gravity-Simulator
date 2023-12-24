pub mod vec2;
pub use vec2::Vec2;

pub type Scalar = f32;

pub const G: Scalar = 6.67430e-11;

/// An object with a mass and a position.
/// Could be for example a planet, an asteroid or an apple.
/// Objects are sorted by their position, their mass is ignored in the process.
#[derive(Debug, Clone, Copy)]
pub struct Object {
    pub x: Scalar,
    pub y: Scalar,
    pub m: Scalar,
}

impl Object {
    pub fn new(x: Scalar, y: Scalar, m: Scalar) -> Self {
        Self { x, y, m }
    }

    pub fn pos(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}
