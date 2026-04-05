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

    fn index_of(&self, position: GridPosition) -> Result<usize, GridMapError> {
        if self.in_bounds(position) {
            Ok(position.y * self.width + position.x)
        } else {
            Err(GridMapError::OutOfBounds)
        }
    }

    pub fn is_walkable(&self, position: GridPosition) -> Result<bool, GridMapError> {
        let index = self.index_of(position)?;
        Ok(self.cells[index])
    }

    pub fn set_walkable(
        &mut self,
        position: GridPosition,
        walkable: bool,
    ) -> Result<(), GridMapError> {
        let index = self.index_of(position)?;
        self.cells[index] = walkable;
        Ok(())
    }

    pub fn neighbors(&self, position: GridPosition) -> Result<Vec<GridPosition>, GridMapError> {
        self.index_of(position)?;

        let mut neighbors = Vec::new();

        if position.y > 0 {
            let up = GridPosition {
                x: position.x,
                y: position.y - 1,
            };

            if self.is_walkable(up)? {
                neighbors.push(up);
            }
        }

        if position.x + 1 < self.width {
            let right = GridPosition {
                x: position.x + 1,
                y: position.y,
            };

            if self.is_walkable(right)? {
                neighbors.push(right);
            }
        }

        if position.y + 1 < self.height {
            let down = GridPosition {
                x: position.x,
                y: position.y + 1,
            };

            if self.is_walkable(down)? {
                neighbors.push(down);
            }
        }

        if position.x > 0 {
            let left = GridPosition {
                x: position.x - 1,
                y: position.y,
            };

            if self.is_walkable(left)? {
                neighbors.push(left);
            }
        }

        Ok(neighbors)
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

        map.set_walkable(GridPosition { x: 1, y: 1 }, false)
            .unwrap();

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

    #[test]
    fn set_walkable_returns_error_for_out_of_bounds_position() {
        let mut map = GridMap::new(3, 2);

        assert_eq!(
            map.set_walkable(GridPosition { x: 9, y: 0 }, false),
            Err(GridMapError::OutOfBounds)
        );
    }

    #[test]
    fn middle_position_returns_four_neighbors() {
        let map = GridMap::new(3, 3);

        assert_eq!(
            map.neighbors(GridPosition { x: 1, y: 1 }),
            Ok(vec![
                GridPosition { x: 1, y: 0 },
                GridPosition { x: 2, y: 1 },
                GridPosition { x: 1, y: 2 },
                GridPosition { x: 0, y: 1 },
            ])
        );
    }

    #[test]
    fn obstacle_neighbor_is_filtered_out() {
        let mut map = GridMap::new(3, 3);

        map.set_walkable(GridPosition { x: 2, y: 1 }, false)
            .unwrap();

        assert_eq!(
            map.neighbors(GridPosition { x: 1, y: 1 }),
            Ok(vec![
                GridPosition { x: 1, y: 0 },
                GridPosition { x: 1, y: 2 },
                GridPosition { x: 0, y: 1 },
            ])
        );
    }

    #[test]
    fn out_of_bounds_position_has_no_neighbors() {
        let map = GridMap::new(3, 3);

        assert_eq!(
            map.neighbors(GridPosition { x: 10, y: 10 }),
            Err(GridMapError::OutOfBounds)
        );
    }
}
