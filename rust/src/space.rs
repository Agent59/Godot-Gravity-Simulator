#[allow(unused_imports)]
use crate::base::{Scalar, Object, Vec2};

#[cfg(feature = "direct")]
use crate::direct::apply_forces;

#[cfg(feature = "barnes_hut")]
use crate::barnes_hut::{Quadtree, Cell, THETA};

#[allow(unused_imports)]
use godot::engine::RigidBody2D;
use godot::prelude::*;

#[cfg(feature = "barnes_hut_parallel_force_calc")]
use std::{thread, sync::{Mutex, Arc}};

// time testing
use std::time::Instant;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Space {
    #[base]
    node2d: Base<Node2D>,

    #[cfg(feature = "barnes_hut")]
    #[export]
    theta: f32,
}

// Needed, otherwise the #[export] wont work.
#[cfg(feature = "barnes_hut")]
#[godot_api]
impl Space {}

#[godot_api]
impl INode2D for Space {
    #[cfg(not(feature = "barnes_hut"))]
    fn init(node2d: Base<Node2D>) -> Self {
        Self { node2d }
    }

    #[cfg(feature = "barnes_hut")]
    fn init(node2d: Base<Node2D>) -> Self {
        Self { node2d, theta: THETA }
    }


    // If no algorithm is specified as a feature, a warning is thrown
    #[cfg(not(any(feature = "direct", feature = "barnes_hut", feature = "fmm")))]
    fn physics_process(&mut self, _delta: f64) {
        let message = "Compiled with no algorithm specified.\n\
            Specify an algorithm with --features <algorithm>.\n\
            Look at the Cargo.toml for available the algorithms.";

        godot_print!("{}", message);
        godot_warn!("{}", message);
    }

    #[cfg(feature = "direct")]
    fn physics_process(&mut self, _delta: f64) {
        let start = Instant::now();

        // The godot representation of masses
        let mut bodies = Vec::<Gd<RigidBody2D>>::new();
        // The custom representation of masses
        let mut objects = Vec::<Object>::new();

        for child in self.node2d.get_children().iter_shared() {
            if child.get("resource_name".into()).to_string() == "Mass" {
                // "." is the current node
                let rigid_body2d = child.try_get_node_as::<RigidBody2D>(".").unwrap();                
                
                objects.push(Object::copy_from_rigidbody(&rigid_body2d));
                bodies.push(rigid_body2d);
            }
        }
        apply_forces(&objects, |i, force| {
            bodies[i].apply_force(force.into());
        });

        godot_print!("gravity force time: {}ms", start.elapsed().as_millis());
    }

    #[cfg(feature = "barnes_hut")]
    fn physics_process(&mut self, _delta: f64) {
        let start = Instant::now();

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
            if child.get("resource_name".into()).to_string() == "Mass" {
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

        let qtree = Quadtree::create_from_objects(&objects, cell);

        godot_print!("gravity force build_tree: {}ms", start.elapsed().as_millis());

        let force_calc_start = Instant::now();

        barnes_hut_calc_forces(qtree, objects, bodies, self.theta);

        godot_print!("gravity force time: {}ms", force_calc_start.elapsed().as_millis());

        godot_print!("gravity total time: {}ms", start.elapsed().as_millis());
    }

    #[cfg(feature = "fmm")]
    fn physics_process(&mut self, _delta: f64) {
        godot_print!("Not yet implemented");
    }

}

#[cfg(all(feature = "barnes_hut", not(feature = "barnes_hut_parallel_force_calc")))]
fn barnes_hut_calc_forces(qtree: Quadtree, objects: Vec<Object>, mut bodies: Vec<Gd<RigidBody2D>>, theta: Scalar)  {
    for (i, body) in bodies.iter_mut().enumerate() {
        let force = qtree.calc_force(objects[i], theta);
        body.apply_force(force.into());
    }
}

/// This is probably inefficient!
#[cfg(feature = "barnes_hut_parallel_force_calc")]
fn barnes_hut_calc_forces(qtree: Quadtree, mut objects: Vec<Object>, mut bodies: Vec<Gd<RigidBody2D>>, theta: Scalar)  {
    // Splits the array of objects into two arrays
    let objects2 = objects.split_off(objects.len() / 2);

    let forces2_arc = Arc::new(Mutex::new(Vec::<Vec2>::new()));
    let forces2_arc_clone = Arc::clone(&forces2_arc);
    let qtree2 = qtree.clone();

    let thread2 = thread::spawn(move || {
        let mut forces2 = forces2_arc_clone.lock().unwrap();
        for object2 in objects2 {
            forces2.push(qtree2.calc_force(object2, theta));
        }
    });

    // thread1 is the current thread.
    for (i, object) in objects.iter().enumerate() {
        let force = qtree.calc_force(*object, theta);
        bodies[i].apply_force(force.into());
    }

    thread2.join().unwrap();
    let forces2 = forces2_arc.lock().unwrap();

    for (i, force2) in forces2.iter().enumerate() {
        let correct_i = i + bodies.len() / 2;
        bodies[correct_i].apply_force((*force2).into());
    }
}

