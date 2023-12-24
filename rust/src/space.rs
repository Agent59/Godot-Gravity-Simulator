use crate::base::{Object, Vec2};
use crate::barnes_hut::{Quadtree, Cell, THETA};

use godot::prelude::*;
use godot::engine::{Node2D, INode2D, RigidBody2D};

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

    fn physics_process(&mut self, _delta: f64) {
        // The godot representation of masses
        let mut bodies = Vec::<Gd<RigidBody2D>>::new();
        // The custom representation of masses
        let mut objects = Vec::<Object>::new();

        // uses the same steps as calc_tree_box in Quadtree,
        // but by using the for loop that is already needed,
        // we can save a little time.
        let mut smallest_x = 0.;
        let mut smallest_y = 0.;
        let mut largest_x = 0.;
        let mut largest_y = 0.;
        let mut first_body = true;

        for child in self.node2d.get_children().iter_shared() {
            if child.get_scene_file_path().to_string() == "res://mass.tscn" {
                // "." is the current node
                let rigid_body2d = child.try_get_node_as::<RigidBody2D>(".").unwrap();                
                let pos: Vec2 = rigid_body2d.get_position().into();
                
                objects.push(Object::copy_from_rigidbody(&rigid_body2d));
                bodies.push(rigid_body2d);

                if first_body {
                    smallest_x = pos.x; smallest_y = pos.y;
                    largest_x = pos.x; largest_y = pos.y;
                    first_body = false;
                    continue;
                }

                if pos.x < smallest_x { smallest_x = pos.x }
                else if pos.x > largest_x { largest_x = pos.x }

                if pos.y < smallest_y { smallest_y = pos.y }
                else if pos.y > largest_y { largest_y = pos.y }
            }
        }

        let size = largest_x.max(largest_y) - smallest_x.min(smallest_y);
        let cell = Cell::new(smallest_x, smallest_y, size);

        let qtree = Quadtree::create_from_objects(objects, cell);

        for mut body in bodies {
            let force = qtree.calc_force(Object::copy_from_rigidbody(&body), THETA);
            body.apply_force(force.into());
        }
    }
}
