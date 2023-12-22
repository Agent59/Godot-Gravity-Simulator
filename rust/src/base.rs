pub mod vec2;
pub use vec2::Vec2;

pub type Scalar = f64;

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
}
