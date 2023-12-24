use crate::base::{Scalar, Object, Vec2, G};
use crate::barnes_hut::Cell;

/// The coordinates describe the center of mass for that cell.
#[derive(Debug, Clone)]
pub struct Quadtree {
    x: Scalar,
    y: Scalar,
    m: Scalar,
    children: Vec<Option<Self>>,
    cell: Cell,
}

impl Quadtree {
    /// Creates a quadtree from a set of objects.
    /// The cell configures the size of the quadtree.
    pub fn create_from_objects(objects: Vec<Object>, cell: Cell) -> Self {
        let mut root = Self::leaf(cell.x, cell.y, 0., cell);

        for object in objects {
            root.insert(object.x, object.y, object.m, cell);
        }
        root
    }

    /// Returns a cell that has the size and the origin a quadtree,
    /// which is build with the same objects, should be based on.
    /// TODO WARNING this is not the minimal bounding box.
    pub fn calc_tree_box(objects: &Vec<Object>) -> Cell {
        let mut smallest_x = objects[0].x;
        let mut largest_x = objects[0].x;

        let mut smallest_y = objects[0].y;
        let mut largest_y = objects[0].y;

        for object in objects {
            if object.x < smallest_x { smallest_x = object.x }
            else if object.x > largest_x { largest_x = object.x }

            if object.y < smallest_y { smallest_y = object.y }
            else if object.y > largest_y { largest_y = object.y }
        }
        let size = largest_x.max(largest_y) - smallest_x.min(smallest_y);

        return Cell::new(smallest_x, smallest_y, size)
    }


    /// Creates an empty quadtree node.
    /// Because it is a leaf, it represents an object
    /// or if m == 0 an empty quadtree.
    pub fn leaf(x: Scalar, y: Scalar, m: Scalar, cell: Cell) -> Self {
        Self {
            x,
            y,
            m,
            children: vec![None; 4],
            cell,
        }
    }

    /// Inserts an object into the quadtree.
    /// An object is represented by a leaf.
    pub fn insert(&mut self, x: Scalar, y: Scalar, m: Scalar, cell: Cell) {
        // An object with no mass is not influenced and does not influene gravity.
        if m == 0. { return }

        // If no object has been inserted before, the root node can just represent it.
        if self.m == 0. { self.x = x; self.y = y; self.m = m; return }
        
        // Find the parent node to insert the node under.
        let mut current: &mut Self = self;
        let mut current_cell: Cell = cell;
        let mut quadrant: usize = cell.quadrant(x, y);
        
        while let Some(_) = &mut current.children[quadrant] {
            current.update_com(x, y, m);

            current = current.children[quadrant].as_mut().unwrap();
            current_cell = current_cell.child(quadrant);

            quadrant = current_cell.quadrant(x, y);
        }

        // This is only needed if current.is_leaf() is true,
        // because this must be done before current.update_com(x, y, m) is called, this is not inside the if statement.
        // The object that was represented by the node (now referred to as object2).
        let (x2, y2, m2) = (current.x, current.y, current.m);

        current.update_com(x, y, m);

        // Because a leaf represents an object,
        // the two objects must be split up into seperate cells.
        if current.is_leaf() {

            // The quadrant of object2.
            let mut quadrant2: usize = current_cell.quadrant(x2, y2);

            // Splits the cell until the objects are not in the same quadrant anymore.
            while quadrant == quadrant2 {

                // Creates a cell that contains both objects.
                // The center of mass stays the same and does not need to be updated.
                current.new_child(quadrant, current.x, current.y, current.m, current_cell.child(quadrant));

                current = current.children[quadrant].as_mut().unwrap();
                current_cell = current_cell.child(quadrant);

                quadrant = current_cell.quadrant(x, y);
                quadrant2 = current_cell.quadrant(x2, y2);
            }
            // Once the quadrants are different, a node is created for each object.
            // (The node for the other object is created further below)
            current.new_child(quadrant2, x2, y2, m2, current_cell.child(quadrant2));
        }
        // If the node is not a leaf
        // and the node does not contain a child in which the object would fit,
        // a child, which represents the object, is created.
        current.new_child(quadrant, x, y, m, current_cell.child(quadrant));
    }

    /// Updates the center of mass.
    pub fn update_com(&mut self, x: Scalar, y: Scalar, m: Scalar) {
        let total_m = self.m + m;
        self.x = (self.m * self.x + m * x) / total_m;
        self.y = (self.m * self.y + m * y) / total_m;
        self.m = total_m;
    }

    /// Checks if the node is a leaf.
    pub fn is_leaf(&self) -> bool {
        for child in &self.children {
            if child.is_some() {
                return false
            }
        }
        true
    }

    /// Adds a child under the node.
    pub fn new_child(&mut self, quadrant: usize, x: Scalar, y: Scalar, m: Scalar, cell: Cell) {
        self.children[quadrant] = Some(Self::leaf(x, y, m, cell))
    }

    /// Calculates the total force that acts on the provided body
    /// for the accuracy θ.
    pub fn calc_force(&self, obj: Object, theta: Scalar) -> Vec2 {
        let mut total_force = Vec2::new(0., 0.);

        type QTFCI<'a> = QuadtreeForceCalculationIterator<'a>;

        for obj2 in QTFCI::new(obj.pos(), theta, self) {
            let r_vec = obj2.pos() - obj.pos();

            if r_vec.length() == 0. { continue; }

            let force = G * ((obj.m * obj2.m) / r_vec.length().powf(3.)) * r_vec;
            total_force += force;
        }
        total_force
    }

    /// Calls the provided function for every node with the nodes properties.
    /// The functions parameters should be fn(node_x, node_y, node_m, cell, level).
    pub fn do_on_nodes<T>(&self, f: &mut impl FnMut(Scalar, Scalar, Scalar, Cell, usize) -> T) -> T {
        self.do_on_nodes_helper_func(0, f)
    }

    /// Is used by do_on_nodes.
    /// Needs to be given a starting level to correctly calculate the current level.
    /// The provided level must always be zero.
    /// Calls the provided function for every node with the nodes properties.
    /// The functions parameters should be fn(node_x, node_y, node_m, cell, level).
    fn do_on_nodes_helper_func<T>(&self, level: usize, f: &mut impl FnMut(Scalar, Scalar, Scalar, Cell, usize) -> T) -> T {
        let result = f(self.x, self.y, self.m, self.cell, level);
        for opt_child in &self.children {
            if let Some(child) = opt_child {
                child.do_on_nodes_helper_func(level + 1, f);
            }
        }
        result
    }

    /// Returns a list of all the leaves.
    /// TODO Probably very memory intese and slow at the moment.
    pub fn get_leaves(&self) -> Vec<Self> {
        let mut leaves = Vec::<Quadtree>::new();
        let mut no_child: u8 = 1;

        for child in &self.children {
            if child.is_some() {
                leaves.append(&mut child.clone().unwrap().get_leaves());
            } else {
                no_child += 1;
            }
        }
        if no_child == 4 {
            leaves.push(self.clone());
        }
        leaves
    }
}

impl std::fmt::Display for Quadtree {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let mut fmt_helper = |x: Scalar, y: Scalar, m: Scalar, cell: Cell, level: usize| {
            for _ in 0..level {
                write!(formatter, "  |  ")?;
            }
            write!(formatter, "({:.2}|{:.2}|{:.2} | {cell})\n", x, y, m)
        };

        self.do_on_nodes(&mut fmt_helper)
    }
}

struct QuadtreeForceCalculationIterator<'a> {
    pos: Vec2,
    theta: Scalar,
    sub_trees: Vec<&'a Quadtree>,
}

impl<'a> QuadtreeForceCalculationIterator<'a> {
    pub fn new(pos: Vec2, theta: Scalar, qtree: &'a Quadtree) -> Self {
        Self { pos, theta, sub_trees: vec![qtree], }
    }
}

/// Returns the next Object that is needed to calculate the total force.
/// The returned Object represents a center of masse or a single mass.
impl<'a> Iterator for QuadtreeForceCalculationIterator<'a> {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.sub_trees.is_empty() {
            let sub_tree = self.sub_trees.pop()?;

            let distance = (Vec2::new(sub_tree.x, sub_tree.y) - self.pos).length();
            
            if sub_tree.cell.size / distance < self.theta || sub_tree.is_leaf() {
                return Some(Object::new(sub_tree.x, sub_tree.y, sub_tree.m))
            }

            for opt_child in &sub_tree.children {
                if let Some(child_tree) = opt_child {
                    self.sub_trees.push(&child_tree);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod quadtree_tests {
    use crate::base::*;
    use crate::barnes_hut::Quadtree;
    
    #[test]
    fn test_tree_creation() {
        let mut objects = Vec::<Object>::new();
        for i in (1..10).map(|i| i as Scalar) {
            let object = Object::new(i, i, 1.);
            objects.push(object);
        }
        objects.remove(4);

        let tree_box = Quadtree::calc_tree_box(&objects);
        let qtree = Quadtree::create_from_objects(objects.clone(), tree_box);


        println!("objects:\n");
        for object in objects {
            println!("{:?}\n", object);
        }
        println!("\nQuadtree ({}):\n{}", tree_box, qtree);
    }
}
