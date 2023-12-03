pub mod quadtree;

pub use quadtree::Quadtree;
pub mod vec2;
pub use vec2::Vec2;

type Scalar = f64;

/// An object with a mass and a position.
/// Could be for example a planet, an asteroid or an apple.
/// Objects are sorted by their position, their mass is ignored in the process.
#[derive(Debug, Clone, Copy)]
pub struct Object {
    x: Scalar,
    y: Scalar,
    m: Scalar,
}

impl Object {
    pub fn new(x: Scalar, y: Scalar, m: Scalar) -> Self {
        Self { x, y, m }
    }
}

/// Cells are just temporary.
/// TODO They are utilised in the building process of the quadtree.
#[derive(Debug, Clone, Copy)]
pub struct Cell {
    x: Scalar,
    y: Scalar,
    size: Scalar,
}

impl Cell {
    pub fn new(x: Scalar, y: Scalar, size: Scalar) -> Self {
        Self { x, y, size }
    }

    /// Returns the x coordinate of the cells center.
    pub fn center_x(&self) -> Scalar {
        self.x + self.size / 2.
    }

    /// Returns the y coordinate of the cells center.
    pub fn center_y(&self) -> Scalar {
        self.y + self.size / 2.
    }

    /// Returns the Quadrant the point is in,
    /// even if the point is not inside the cells boundary.
    /// The quadrants are numbered as follows:
    /// ---------
    /// | 2 | 3 |
    /// ---------
    /// | 0 | 1 |
    /// --------- 
    pub fn quadrant(&self, x: Scalar, y: Scalar) -> usize {
        // True converted to a usize is 1.
        let x_bit = (x >= self.center_x()) as usize;
        let y_bit = (y >= self.center_y()) as usize;
        x_bit + (y_bit << 1)
    }

    /// Returns the subquadrant of the cell based on the provided quadrant.
    /// See the quadrant functions comment for the numbering scheme.
    /// Panics if anything not fitting the quadrant numbering scheme is provided.
    pub fn child(&self, quadrant: usize) -> Cell {
        let half_size = self.size / 2.;
        match quadrant {
            0 => Self::new(self.x, self.y, half_size),
            1 => Self::new(self.center_x(), self.y, half_size),
            2 => Self::new(self.x, self.center_y(), half_size),
            3 => Self::new(self.center_x(), self.center_y(), half_size),
            _ => panic!("Quadrant has to be either one of these:\n\
            ---------\n\
            | 2 | 3 |\n\
            ---------\n\
            | 0 | 1 |\n\
            ---------")
        }
    }
}
