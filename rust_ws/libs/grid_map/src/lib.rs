#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPosition {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridMap {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GridMapError {
    OutOfBounds,
}

impl GridMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![true; width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn in_bounds(&self, position: GridPosition) -> bool {
        position.x < self.width && position.y < self.height
    }

    pub fn is_walkable(&self, position: GridPosition) -> Result<bool, GridMapError> {
        if self.in_bounds(position) {
            let index = position.y * self.width + position.x;
            Ok(self.cells[index])
        } else {
            Err(GridMapError::OutOfBounds)
        }
    }

    pub fn set_walkable(
        &mut self,
        position: GridPosition,
        walkable: bool,
    ) -> Result<(), GridMapError> {
        if self.in_bounds(position) {
            let index = position.y * self.width + position.x;
            self.cells[index] = walkable;
            Ok(())
        } else {
            Err(GridMapError::OutOfBounds)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_map_with_given_size() {
        let map = GridMap::new(3, 2);

        assert_eq!(map.width(), 3);
        assert_eq!(map.height(), 2);
    }

    #[test]
    fn point_inside_map_is_in_bounds() {
        let map = GridMap::new(3, 2);

        assert_eq!(map.in_bounds(GridPosition { x: 2, y: 1 }), true);
    }

    #[test]
    fn point_outside_map_is_not_in_bounds() {
        let map = GridMap::new(3, 2);

        assert_eq!(map.in_bounds(GridPosition { x: 3, y: 1 }), false);
    }

    #[test]
    fn can_set_obstacle_and_read_it() {
        let mut map = GridMap::new(3, 2);

        map.set_walkable(GridPosition { x: 1, y: 1 }, false).unwrap();

        assert_eq!(map.is_walkable(GridPosition { x: 1, y: 1 }), Ok(false));
    }

    #[test]
    fn out_of_bounds_position_returns_error() {
        let map = GridMap::new(3, 2);

        assert_eq!(
            map.is_walkable(GridPosition { x: 5, y: 0 }),
            Err(GridMapError::OutOfBounds)
        );
    }
}