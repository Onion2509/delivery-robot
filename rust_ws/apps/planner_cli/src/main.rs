use grid_map::{GridMap, GridPosition};
use path_planning::{AStartPlanner, PathPlanner};
use std::env;

struct DemoScene {
    name: &'static str,
    width: usize,
    height: usize,
    start: GridPosition,
    goal: GridPosition,
    obstacles: Vec<GridPosition>,
}

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

fn build_demo_scene(scene_id: u8) -> DemoScene {
    match scene_id {
        1 => DemoScene {
            name: "basic",
            width: 3,
            height: 3,
            start: GridPosition { x: 0, y: 0 },
            goal: GridPosition { x: 2, y: 2 },
            obstacles: vec![
                GridPosition { x: 1, y: 0 },
                GridPosition { x: 1, y: 1 },
            ],
        },
        2 => DemoScene {
            name: "blocked_start",
            width: 3,
            height: 3,
            start: GridPosition { x: 0, y: 0 },
            goal: GridPosition { x: 2, y: 2 },
            obstacles: vec![
                GridPosition { x: 1, y: 0 },
                GridPosition { x: 0, y: 1 },
            ],
        },
        _ => DemoScene {
            name: "empty",
            width: 3,
            height: 3,
            start: GridPosition { x: 0, y: 0 },
            goal: GridPosition { x: 2, y: 2 },
            obstacles: vec![],
        },
    }
}

fn build_map(scene: &DemoScene) -> GridMap {
    let mut map = GridMap::new(scene.width, scene.height);

    for obstacle in &scene.obstacles {
        map.set_walkable(*obstacle, false).unwrap();
    }

    map
}

fn read_sccene_id() -> u8 {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return 1;
    }

    match args[1].parse::<u8>() {
        Ok(scene_id) => scene_id,
        Err(_) => 1,
    }
}

fn main() {
    let planner = AStartPlanner;
    let scene_id = read_sccene_id();
    let scene = build_demo_scene(scene_id);
    let map = build_map(&scene);

    println!("running scene : {}", scene.name);

    match planner.plan(&map, scene.start, scene.goal) {
        Ok(path) => {
            println!("planned path: {:?}", path);
            print_map_with_path(&map, &path, scene.start, scene.goal);
        }
        Err(error) => {
            println!("planning failed: {:?}", error);
            print_map_with_path(&map, &[], scene.start, scene.goal);
        }
    }
}