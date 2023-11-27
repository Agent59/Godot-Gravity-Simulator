use crate::godot_aliases::Vec2;

pub mod quadtree;

/// Represents a cell in the grid and a node in the quadtree.
/// The coordinates are relative to the 'Grid', where the upper left corner is the origin.
/// The level describes how "deep" the cell is in the quadtree.
pub struct Cell {
    x: u64,
    y: u64,
    level: u64
}

/// The provided point should be absolute.
/// The range is the one of the 'Grid'.
/*impl From<Vec2> for Cell {
    fn from(vec2: Vec2) -> Self {
        
    }
}*/

impl Cell {

    /// The provided point should be absolute.
    /// The range is the one of the 'Grid'.
    pub fn from_point(point: Vec2, range: u64) -> Self {

        // "1 << level" is the same as "2 to the power of level")
        let size = range / ((1 << level) as f64);
        
        Self {
            x,
            y,
            level
        }
    }

    pub fn hash(&self) -> usize {
        let x = self.x;
        let y = self.y;
        
        let tmp = y + ((x+1) / 2);
        let tmp = x + (tmp * tmp);

        return tmp as usize;
    }
}
