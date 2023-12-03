use crate::fmm::Vec2;
use godot::builtin;

impl From<builtin::Vector2> for Vec2 {
    fn from(vector2: builtin::Vector2) -> Self {
        Self {
            x: vector2.x as f64,
            y: vector2.y as f64,
        }
    }
}
