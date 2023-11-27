use godot::prelude::*;
use godot::engine::{Node2D, INode2D};

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Space {
    
    #[base]
    node2d: Base<Node2D>
}

#[godot_api]
impl INode2D for Space {
    fn init(node2d: Base<Node2D>) -> Self {
        godot_print!("Space!");

        Self {
            node2d
        }
    }

    fn ready(&mut self) {
        let child_count = self.node2d.get_child_count();
        godot_print!("child_count: {}", child_count);
    }
}
