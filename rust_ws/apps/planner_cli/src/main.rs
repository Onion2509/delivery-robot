use config::AppConfig;
use grid_map::{GridMap, GridPosition};
use path_planning::{AStartPlanner, PathPlanner, PathPlanningError};
use std::env;
use task_core::{DeliveryTask, TaskError};

const START_SYMBOL: char = 'S';
const GOAL_SYMBOL: char = 'G';
const PATH_SYMBOL: char = '*';
const FREE_SYMBOL: char = '.';
const OBSTACLE_SYMBOL: char = '#';

struct DemoScene {
    name: &'static str,
    config: AppConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RunTaskError {
    Task(TaskError),
    Planning(PathPlanningError),
}

fn print_map_with_path(
    map: &GridMap,
    path: &[GridPosition],
    start: GridPosition,
    goal: GridPosition,
) {
    let max_x = map.width() - 1;
    let max_y = map.height() - 1;
    let coord_width = max_x.max(max_y).to_string().len();

    println!("== Map View ==");
    println!("map (x ->, y down):");

    print!("{:>width$} ", "", width = coord_width);
    
    for x in 0..map.width() {
        print!("{:>width$} ", x, width = coord_width);
    }

    println!();

    for y in 0..map.height() {
        print!("{:>width$} ", y, width = coord_width);

        for x in 0..map.width() {
            let position = GridPosition { x, y };

            let symbol = if position == start {
                START_SYMBOL
            } else if position == goal {
                GOAL_SYMBOL
            } else if path.contains(&position) {
                PATH_SYMBOL
            } else if map.is_walkable(position).unwrap() {
                FREE_SYMBOL
            } else {
                OBSTACLE_SYMBOL
            };

            print!("{:>width$} ", symbol, width = coord_width);
        }

        println!();
    }
}

fn build_demo_scene(scene_id: u8) -> DemoScene {
    match scene_id {
        1 => DemoScene {
            name: "basic",
            config: AppConfig::default(),
        },
        2 => DemoScene {
            name: "blocked_start",
            config: AppConfig {
                map_width: 3,
                map_height: 3,
                start: GridPosition { x: 0, y: 0 },
                goal: GridPosition { x: 2, y: 2 },
                obstacles: vec![
                    GridPosition { x: 1, y: 0 },
                    GridPosition { x: 0, y: 1 },
                ],
            },
        },
        other => {
            println!("unknow scene id {}, using empty scene", other);
            
            DemoScene {
                name: "empty",
                config: AppConfig {
                    map_width: 3,
                    map_height: 3,
                    start:GridPosition { x: 0, y: 0 },
                    goal: GridPosition { x: 2, y: 2 },
                    obstacles: vec![],
                },
            }
        }
    }
}

fn build_map(config: &AppConfig) -> GridMap {
    let mut map = GridMap::new(config.map_width, config.map_height);

    for obstacle in &config.obstacles {
        map.set_walkable(*obstacle, false).unwrap();
    }

    map
}

fn build_task_from_scene(scene: &DemoScene) -> DeliveryTask {
    DeliveryTask::new(1, scene.config.goal)
}

fn run_task(
    planner: &AStartPlanner,
    map: &GridMap,
    start: GridPosition,
    task: &mut DeliveryTask,
) -> Result<Vec<GridPosition>, RunTaskError> {
    if let Err(error) = task.assign() {
        return Err(RunTaskError::Task(error));
    }

    if let Err(error) = task.start_navigation() {
        return Err(RunTaskError::Task(error));
    }

    match planner.plan(map, start, task.destination()) {
        Ok(path) => {
            if let Err(error) = task.complete() {
                return Err(RunTaskError::Task(error))
            }

            Ok(path)
        }
        Err(error) => {
            if let Err(task_error) = task.fail() {
                return Err(RunTaskError::Task(task_error))
            }

            Err(RunTaskError::Planning(error))
        }
    }
}

fn parse_scene_id_arg(arg: Option<&String>) -> Result<u8, &'static str> {
    match arg {
        Some(value) => match value.parse::<u8>() {
            Ok(scene_id) => Ok(scene_id),
            Err(_) => Err("invalid"),
        },
        None => Err("missing"),
    }
}

fn read_scene_id() -> u8 {
    let args: Vec<String> = env::args().collect();

    match parse_scene_id_arg(args.get(1)) {
        Ok(scene_id) => scene_id,
        Err("missing") => {
            println!("no scene id provided, defaulting to scene 1");
            1
        }
        Err(_) => {
            println!("invalid scene id '{}', defaulting to scene 1", args[1]);
            1
        }
    }
}

fn print_scene_summary(scene: &DemoScene) {
    println!("== Scene Summary ==");
    println!("running scene: {}", scene.name);
    println!("size: {}x{}", scene.config.map_width,scene.config.map_height);
    println!("start: ({}, {})", scene.config.start.x, scene.config.start.y);
    println!("goal: ({}, {})", scene.config.goal.x, scene.config.goal.y);
    println!("obstacle count: {}", scene.config.obstacles.len());

    for obstacle in &scene.config.obstacles {
        println!("obstacle: ({}, {})", obstacle.x, obstacle.y);
    }
}

fn print_task_summary(task: &DeliveryTask) {
    println!("== Task Summary ==");
    println!("task id: {}", task.id());
    println!(
        "task destination: ({}, {})",
        task.destination().x,
        task.destination().y
    );
    println!("task state: {:?}", task.state());
}

fn describe_step(from: GridPosition, to: GridPosition) -> &'static str {
    if to.x > from.x {
        "→"
    } else if to.x < from.x {
        "←"
    } else if to.y > from.y {
        "↓"
    } else if to.y < from.y {
        "↑"
    } else {
        "."
    }
}

fn print_path_details(path: &[GridPosition]) {
    println!("== Path Details ==");
    println!("path point count: {}", path.len());

    let move_count = if path.len() > 0 {
        path.len() - 1 
    } else {
        0
    };

    println!("path move count: {}", move_count);

    for (step_index, position) in path.iter().enumerate() {
        println!("step {}: ({}, {})", step_index, position.x, position.y);
    }

    if path.len() >= 2 {
        println!();
        println!("== Step Directions ==");

        for step_index in 1..path.len() {
            let from = path[step_index - 1];
            let to = path[step_index];
            let direction = describe_step(from, to);

            println!(
                "move {}: ({}, {}) -> ({}, {}) [{}]",
                step_index - 1,
                from.x,
                from.y,
                to.x,
                to.y,
                direction,
            );
        }
    }
}

fn main() {
    let planner = AStartPlanner;
    let scene_id = read_scene_id();
    let scene = build_demo_scene(scene_id);

    if let Err(error) = scene.config.validate() {
        println!("== Config Error ==");
        println!("invalid config: {:?}", error);
        return;
    }

    let map = build_map(&scene.config);
    let mut task = build_task_from_scene(&scene);

    print_scene_summary(&scene);
    println!();
    print_task_summary(&task);
    println!();

    let run_result = run_task(&planner, &map, scene.config.start, &mut task);

    println!("== Task Progress ==");
    println!("task state after run: {:?}", task.state());
    println!();

    match run_result {
        Ok(path) => {
            print_path_details(&path);
            println!();
            print_map_with_path(&map, &path, scene.config.start, task.destination());
            println!();
            print_task_summary(&task);
        }
        Err(RunTaskError::Planning(error)) => {
            println!("== Planning Result ==");
            println!("planning failed: {:?}", error);
            println!();
            print_map_with_path(&map, &[], scene.config.start, task.destination());
            println!();
            print_task_summary(&task);
        }
        Err(RunTaskError::Task(error)) => {
            println!("== Task Error ==");
            println!("task state transition failed: {:?}", error);
            println!();
            print_task_summary(&task);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_scene_id_arg_accepts_valid_number() {
        let arg = String::from("2");

        assert_eq!(parse_scene_id_arg(Some(&arg)), Ok(2));
    }

    #[test]
    fn parse_scene_id_arg_rejects_non_number() {
        let arg = String::from("abc");

        assert_eq!(parse_scene_id_arg(Some(&arg)), Err("invalid"));
    }

    #[test]
    fn parse_scene_id_arg_reports_missing_argument() {
        assert_eq!(parse_scene_id_arg(None), Err("missing"));
    }

    #[test]
    fn build_task_from_scene_uses_scene_goal() {
        let scene = build_demo_scene(1);
        let task = build_task_from_scene(&scene);

        assert_eq!(task.destination(), scene.config.goal);
        assert_eq!(task.state(), task_core::TaskState::Pending);
    }

    #[test]
    fn run_task_completes_task_when_path_exists() {
        let planner = AStartPlanner;
        let scene = build_demo_scene(1);
        let map = build_map(&scene.config);
        let mut task = build_task_from_scene(&scene);

        let result = run_task(&planner, &map, scene.config.start, &mut task);

        assert!(result.is_ok());
        assert_eq!(task.state(), task_core::TaskState::Completed);
    }

    #[test]
    fn run_task_marks_task_failed_when_no_path_exists() {
        let planner = AStartPlanner;
        let scene = build_demo_scene(2);
        let map = build_map(&scene.config);
        let mut task = build_task_from_scene(&scene);

        let result = run_task(&planner, &map, scene.config.start, &mut task);

        assert_eq!(
            result,
            Err(RunTaskError::Planning(PathPlanningError::NoPathFound))
        );
        assert_eq!(task.state(), task_core::TaskState::Failed);
    }
}