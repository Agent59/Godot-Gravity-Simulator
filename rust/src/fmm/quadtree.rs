use crate::fmm::{Scalar, Cell, Object};

/// The coordinates describe the center of mass for that cell.
#[derive(Debug, Clone)]
pub struct Quadtree {
    x: Scalar,
    y: Scalar,
    m: Scalar,
    children: Vec<Option<Self>>,
}

impl Quadtree {
    /// Creates a quadtree from a set of objects.
    /// The cell configures the size of the quadtree.
    pub fn create_from_objects(objects: Vec<Object>, cell: Cell) -> Self {
        let mut root = Self::leaf(cell.x, cell.y, 0.);

        for object in objects {
            root.insert(object.x, object.y, object.m, cell);
        }
        root
    }

    /// Returns a cell that has the size and the origin a quadtree,
    /// which is build with the same objects, should be based on.
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
    pub fn leaf(x: Scalar, y: Scalar, m: Scalar) -> Self {
        Self {
            x,
            y,
            m,
            children: vec![None, None, None, None],
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
                current.new_child(quadrant, current.x, current.y, current.m);

                current = current.children[quadrant].as_mut().unwrap();
                current_cell = current_cell.child(quadrant);

                quadrant = current_cell.quadrant(x, y);
                quadrant2 = current_cell.quadrant(x2, y2);
            }
            // Once the quadrants are different, a node is created for each object.
            // (The node for the other object is created further below)
            current.new_child(quadrant2, x2, y2, m2);
        }
        // If the node is not a leaf
        // and the node does not contain a child in which the object would fit,
        // a child, which represents the object, is created.
        current.new_child(quadrant, x, y, m);
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
    pub fn new_child(&mut self, quadrant: usize, x: Scalar, y: Scalar, m: Scalar) {
        self.children[quadrant] = Some(Self::leaf(x, y, m))
    }

    /// Returns a list of all the leaves.
    /// TODO Probably very memory intese at the moment.
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

    /// TODO Probably very memory intense.
    pub fn fmt_helper(&self, formatter: &mut std::fmt::Formatter<'_>, level: usize) -> std::fmt::Result {
        for _ in 0..level {
            write!(formatter, "  |  ")?;
        }
        write!(formatter, "({:.2}|{:.2}|{:.2})\n", self.x, self.y, self.m)?;
            
        for child_opt in &self.children {
            if let Some(child) = child_opt {
                child.fmt_helper(formatter, level + 1)?;
            }
        }
        write!(formatter, "")
    }
}

impl std::fmt::Display for Quadtree {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_helper(formatter, 0)
    }
}

#[cfg(test)]
mod quadtree_tests {
    use crate::fmm::{Quadtree, Object};
    
    #[test]
    fn test_tree_creation() {
        let mut objects = Vec::<Object>::new();
        for i in (1..10).map(|i| i as f64) {
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
        println!("\nQuadtree ({:?}):\n{}", tree_box, qtree);
    }
}
