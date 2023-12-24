use crate::base::{Scalar, Vec2, Object};
use godot::{builtin, engine, obj};

impl From<builtin::Vector2> for Vec2 {
    fn from(vector2: builtin::Vector2) -> Self {
        Self {
            x: vector2.x as Scalar,
            y: vector2.y as Scalar,
        }
    }
}

impl From<Vec2> for builtin::Vector2 {
    fn from(vec2: Vec2) -> Self {
        builtin::Vector2 {
            x: vec2.x as f32,
            y: vec2.y as f32,
        }
    }
}

impl From<obj::Gd<engine::RigidBody2D>> for Object {
    fn from(rigid_body2d: obj::Gd<engine::RigidBody2D>) -> Self {
        let pos: Vec2 = rigid_body2d.get_position().into();
        Self::new(pos.x, pos.y, rigid_body2d.get_mass() as Scalar)
    }
}

impl Object {
    pub fn copy_from_rigidbody(rigid_body2d: &obj::Gd<engine::RigidBody2D>) -> Self {
        let pos: Vec2 = rigid_body2d.get_position().into();
        Self::new(pos.x, pos.y, rigid_body2d.get_mass() as Scalar)
    }
}
