use grid_map::{GridMap, GridPosition};
use path_planning::{AStartPlanner, PathPlanner};

fn print_map_with_path(
    map: &GridMap,
    path: &[GridPosition],
    start: GridPosition,
    goal: GridPosition,
) {
    for y in 0..map.height() {
        for x in 0..map.width() {
            let position = GridPosition { x, y };

            let symbol = if position == start {
                'S'
            } else if position == goal {
                'G'
            } else if path.contains(&position) {
                '*'
            } else if map.is_walkable(position).unwrap() {
                '.'
            } else {
                '#'
            };

            print!("{}", symbol);
        }

        println!();
    }
}

fn main() {
    let planner = AStartPlanner;
    let mut map = GridMap::new(3, 3);

    map.set_walkable(GridPosition { x: 1, y: 0 }, false).unwrap();
    map.set_walkable(GridPosition { x: 1, y: 1 }, false).unwrap();

    let start = GridPosition { x: 0, y: 0 };
    let goal = GridPosition { x: 2, y: 2 };

    match planner.plan(&map, start, goal) {
        Ok(path) => {
            println!("planned path: {:?}", path);
            print_map_with_path(&map, &path, start, goal);
        }
        Err(error) => {
            println!("planning failed: {:?}", error);
        }
    }
}