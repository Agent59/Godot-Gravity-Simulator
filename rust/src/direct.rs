use crate::base::{Object, Vec2, G};

pub fn get_forces(objects: &Vec<Object>) -> Vec<Vec2> {
    let mut forces = Vec::<Vec2>::new();

    for obj1 in objects {
        for obj2 in objects {
            if obj1.x == obj2.y && obj1.x == obj2.y { continue; }

            let r_vec = obj1.pos() - obj2.pos();
            let force = G * ((obj1.m * obj2.m) / r_vec.length().powf(3.)) * r_vec;
            forces.push(force);
        }
    }
    forces
}

pub fn apply_forces(objects: &Vec<Object>, mut f: impl FnMut(usize, Vec2)) {
    for (i, obj1) in objects.iter().enumerate() {
        for obj2 in objects {
            if obj1.x == obj2.y && obj1.x == obj2.y { continue; }

            let r_vec = obj1.pos() - obj2.pos();
            let force = G * ((obj1.m * obj2.m) / r_vec.length().powf(3.)) * r_vec;

            f(i, force);
        }
    }

}
