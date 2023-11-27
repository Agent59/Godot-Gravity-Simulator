use std::collections::HashMap;

use crate::godot_aliases::Vec2;
use crate::fmm::Cell;

/// Holds the particles in the form of an octree,
/// not by nesting structs but rather using hashes
///
/// The range is the width or length of the rectangular Grid.
/// The maximum depth should be updated if a new cell with a deeper level is created.
///
/// TODO improve comments
pub struct Grid {
    // relative to the grid
    cell_x: HashMap<usize, f64>,
    cell_y: HashMap<usize, f64>,

    range: f64,
    max_depth: usize,
}

impl Grid {

    /// Creates a new empty 'Grid'.
    pub fn new(max_depth: usize) -> Self {

        //let cell_x = vec![0.0; 0];
        //let cell_y = vec![0.0; 0];
        let cell_x = HashMap::new();
        let cell_y = HashMap::new();

        Grid {
            cell_x,
            cell_y,

            range: 0.0,
            max_depth: 1,
        }
    }

    pub fn add_body(&self, point: Vec2, mass: f64) {

        // the cells or nodes the point is inside
        //let fitting_cells = Vec::<Cell>::new();

        //for level in (1..self.max_depth).map(|x| x as u64) {
        //

        let cell = self.create_own_cell(point);
    }

    pub fn move_body() {}
    
    /// TODO add description and improve comments
    fn create_own_cell(&self, point: Vec2) -> Cell {

        // checks at which level a new cell has to be created
        let mut cell_has_child = true;
        let mut level = 1;        
        let mut hash: usize;
        let mut cell;

        while cell_has_child {
            // creates a temporary cell
            cell = Cell::from_point(point, level);

            // checks if the cell is already in the grid,
            // if not the temporary cell should be added to the grid
            hash = cell.hash();
            let x_key: bool = self.cell_x.contains_key(&hash);
            let y_key: bool = self.cell_y.contains_key(&hash);

            if !(x_key && y_key) {
                cell_has_child = false;
                level += 1;
            }
        }
        // inserts the cell into the grid
        self.cell_x.insert(hash, 0.0);
        self.cell_y.insert(hash, 0.0);

        cell
    }
}
