use grid_map::{GridMap, GridPosition};
use path_planning::{AStartPlanner, PathPlanner};

fn main() {
    let planner = AStartPlanner;
    let mut map = GridMap::new(3, 3);

    map.set_walkable(GridPosition { x: 1, y: 0 }, false).unwrap();
    map.set_walkable(GridPosition { x: 1, y: 1 }, false).unwrap();

    let start = GridPosition { x: 0, y: 0 };
    let goal = GridPosition { x: 2, y: 2 };

    match planner.plan(&map, start, goal) {
        Ok(path) => println!("planned path: {:?}", path),
        Err(error) => println!("planning failed: {:?}", error),
    }
}