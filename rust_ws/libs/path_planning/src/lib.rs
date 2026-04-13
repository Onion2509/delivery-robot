use grid_map::{GridMap, GridPosition};
use std::cmp::Ordering;
use std::collections::{HashMap, BinaryHeap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathPlanningError {
    StartOutOfBounds,
    GoalOutOfBounds,
    StartBlocked,
    GoalBlocked,
    NoPathFound,
}

pub trait PathPlanner {
    fn plan(
        &self,
        map: &GridMap,
        start: GridPosition,
        goal: GridPosition,
    ) -> Result<Vec<GridPosition>, PathPlanningError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FrontierEntry {
    position: GridPosition,
    priority: usize,
}

//待探索节点
impl Ord for FrontierEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        let priority_order = other.priority.cmp(&self.priority);

        if priority_order != Ordering::Equal {
            return priority_order;
        }

        let y_order = other.position.y.cmp(&self.position.y);

        if y_order != Ordering::Equal {
            return y_order;
        }

        other.position.x.cmp(&self.position.x)
    }
}

impl PartialOrd for FrontierEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn should_update_cost(
    cost_so_far: &HashMap<GridPosition, usize>,
    position: GridPosition,
    new_cost: usize,
) -> bool {
    match cost_so_far.get(&position).copied() {
        Some(old_cost) => new_cost < old_cost,
        None => true, 
    }
}

fn reconstruct_path(
    came_from: &HashMap<GridPosition, GridPosition>,
    start: GridPosition,
    goal: GridPosition,
) -> Result<Vec<GridPosition>, PathPlanningError> {
    if start == goal {
        return Ok(vec![start]);
    }

    let mut current = goal;
    let mut path = vec![goal];

    while let Some(previous) = came_from.get(&current).copied() {
        current = previous;
        path.push(current);

        if current == start {
            path.reverse();
            return Ok(path);
        }
    }

    Err(PathPlanningError::NoPathFound)
}

fn heuristic(from: GridPosition, to: GridPosition) -> usize {
    from.x.abs_diff(to.x) + from.y.abs_diff(to.y)
}

pub struct AStartPlanner;

impl PathPlanner for AStartPlanner {
    fn plan(
        &self,
        map: &GridMap,
        start: GridPosition,
        goal: GridPosition,
    ) -> Result<Vec<GridPosition>, PathPlanningError> {
        if !map.in_bounds(start) {
            return Err(PathPlanningError::StartOutOfBounds);
        }

        if !map.in_bounds(goal) {
            return Err(PathPlanningError::GoalOutOfBounds);
        }

        if map.is_walkable(start) == Ok(false) {
            return Err(PathPlanningError::StartBlocked);
        }

        if map.is_walkable(goal) == Ok(false) {
            return Err(PathPlanningError::GoalBlocked);
        }

        if start == goal {
            return Ok(vec![start]);
        }

        let mut frontier = BinaryHeap::new();
        frontier.push(FrontierEntry {
            position: start,
            priority: 0,
        });

        let mut came_from: HashMap<GridPosition, GridPosition> = HashMap::new();
        let mut cost_so_far: HashMap<GridPosition, usize> = HashMap::new();

        cost_so_far.insert(start, 0);

        while let Some(current_entry) = frontier.pop() {
            let current = current_entry.position;

            if current == goal {
                return reconstruct_path(&came_from, start, goal)
            }

            let current_cost = match cost_so_far.get(&current).copied() {
                Some(cost) => cost,
                None => continue,
            };

            let neighbors = match map.neighbors(current) {
                Ok(neighbors) => neighbors,
                Err(_) => return Err(PathPlanningError::NoPathFound),
            };

            for next in neighbors {
                let new_cost = current_cost + 1;

                if should_update_cost(&cost_so_far, next, new_cost) {
                    cost_so_far.insert(next, new_cost);
                    came_from.insert(next, current);

                    let priority = new_cost + heuristic(next, goal);

                    frontier.push(FrontierEntry {
                        position: next,
                        priority,
                    });
                }
            }
        }

        Err(PathPlanningError::NoPathFound)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::BinaryHeap;


    #[test]
    fn returns_start_out_of_bounds_when_start_is_invalid() {
        let planner = AStartPlanner;
        let map = GridMap::new(3,3);

        assert_eq!(
            planner.plan(
                &map,
                GridPosition{ x: 9, y: 0 },
                GridPosition { x: 1, y: 1 },
            ),
            Err(PathPlanningError::StartOutOfBounds)
        );
    }

    #[test]
    fn returns_goal_out_of_bounds_when_goal_is_invalid() {
        let planner = AStartPlanner;
        let map = GridMap::new(3,3);

        assert_eq!(
            planner.plan(
                &map,
                GridPosition { x: 0, y: 0 },
                GridPosition { x: 9, y: 9 },
            ),
            Err(PathPlanningError::GoalOutOfBounds)
        );
    }

    #[test]
    fn returns_start_blocked_when_start_cannot_be_walked() {
        let planner = AStartPlanner;
        let mut map = GridMap::new(3, 3);
        
        map.set_walkable(GridPosition { x: 0, y: 0 }, false).unwrap();

        assert_eq!(
            planner.plan(
                &map,
                GridPosition { x: 0, y: 0 },
                GridPosition { x: 2, y: 2 },
            ),
            Err(PathPlanningError::StartBlocked)
        );
    }

    #[test]
    fn returns_goal_blocked_when_goal_cannot_be_walked() {
        let planner = AStartPlanner;
        let mut map = GridMap::new(3, 3);

        map.set_walkable(GridPosition { x: 2, y: 2 }, false).unwrap();

        assert_eq!(
            planner.plan(
                &map,
                GridPosition { x: 0, y: 0 },
                GridPosition { x: 2, y: 2 },
            ),
            Err(PathPlanningError::GoalBlocked)
        );
    }

    #[test]
    fn returns_single_point_path_when_start_equals_goal() {
        let planner = AStartPlanner;
        let map = GridMap::new(3, 3);
        let point = GridPosition { x:1, y:1 };

        assert_eq!(
            planner.plan(&map, point, point),
            Ok(vec![point])
        );
    }

    #[test]
    fn finds_path_on_open_grid() {
        let planner = AStartPlanner;
        let map = GridMap::new(3, 3);

        let path = planner
            .plan(
                &map,
                GridPosition { x: 0, y: 0 },
                GridPosition { x: 2, y: 2 },
            ).unwrap();
        
        assert_eq!(path.first().copied(), Some(GridPosition { x: 0, y: 0 }));
        assert_eq!(path.last().copied(), Some(GridPosition { x: 2, y: 2 }));
        assert_eq!(path.len(), 5);
    }

    #[test]
    fn returns_no_path_found_when_goal_is_unreachable() {
        let planner = AStartPlanner;
        let mut map = GridMap::new(3, 3);

        map.set_walkable(GridPosition { x: 1, y: 0 }, false).unwrap();
        map.set_walkable(GridPosition { x: 0, y: 1 }, false).unwrap();

        assert_eq!(
            planner.plan(
                &map,
                GridPosition { x: 0, y: 0 },
                GridPosition { x: 2, y: 2 },
            ),
            Err(PathPlanningError::NoPathFound),
        )
    }

    #[test]
    fn lower_priority_entry_is_popped_first() {
        let mut frontier = BinaryHeap::new();

        frontier.push(FrontierEntry {
            position: GridPosition { x: 0, y: 0 },
            priority: 5,
        });

        frontier.push(FrontierEntry {
            position: GridPosition { x: 1, y: 0 },
            priority: 1,
        });

        let first = frontier.pop().unwrap();

        assert_eq!(first.position, GridPosition { x: 1, y:0 });
        assert_eq!(first.priority, 1);
    }

    #[test]
    fn hash_map_can_store_and_read_cost() {
        let start = GridPosition { x: 0, y: 0 };
        let mut cost_so_far: HashMap<GridPosition, usize> = HashMap::new();

        cost_so_far.insert(start, 0);

        assert_eq!(cost_so_far.get(&start).copied(), Some(0));
    }

    #[test]
    fn hash_map_can_store_and_read_parent() {
        let start = GridPosition { x: 0, y: 0 };
        let goal = GridPosition { x: 1, y: 0 };
        let mut came_from: HashMap<GridPosition, GridPosition> = HashMap::new();

        came_from.insert(goal, start);

        assert_eq!(came_from.get(&goal).copied(), Some(start));
        assert_eq!(came_from.get(&start).copied(), None);
    }

    #[test]
    fn should_update_cost_for_new_position() {
        let cost_so_far: HashMap<GridPosition, usize> = HashMap::new();

        assert_eq!(
            should_update_cost(&cost_so_far, GridPosition { x: 1, y: 1 }, 3),
            true
        );
    }

    #[test]
    fn should_update_cost_when_new_cost_is_smaller() {
        let position = GridPosition { x: 1, y: 1 };
        let mut cost_so_far: HashMap<GridPosition, usize> = HashMap::new();

        cost_so_far.insert(position, 5);

        assert_eq!(
            should_update_cost(&cost_so_far, position, 3),
            true
        );
    }

    #[test]
    fn should_not_update_cost_when_new_cost_is_not_smaller() {
        let position = GridPosition { x: 1, y: 1 };
        let mut cost_so_far: HashMap<GridPosition, usize> = HashMap::new();

        cost_so_far.insert(position, 3);

        assert_eq!(
            should_update_cost(&cost_so_far, position, 5),
            false
        );
    }

    #[test]
    fn reconstruct_path_returns_full_path_when_chain_exists() {
        let start = GridPosition { x: 0, y: 0 };
        let middle = GridPosition { x: 1, y: 0 };
        let goal = GridPosition { x: 2, y: 0 };
        let mut came_from: HashMap<GridPosition, GridPosition> = HashMap::new();

        came_from.insert(middle, start);
        came_from.insert(goal, middle);

        assert_eq!(
            reconstruct_path(&came_from, start, goal),
            Ok(vec![start, middle, goal])
        );
    }

    #[test]
    fn reconstruct_path_returns_single_point_when_start_equals_goal() {
        let point = GridPosition { x: 1, y: 1 };
        let came_from: HashMap<GridPosition, GridPosition> = HashMap::new();

        assert_eq!(
            reconstruct_path(&came_from, point, point),
            Ok(vec![point])
        );
    }

    #[test]
    fn reconstruct_path_returns_error_when_chain_is_missing() {
        let start = GridPosition { x: 0, y: 0 };
        let goal = GridPosition { x: 2, y: 2 };
        let came_from: HashMap<GridPosition, GridPosition> = HashMap::new();

        assert_eq!(
            reconstruct_path(&came_from, start, goal),
            Err(PathPlanningError::NoPathFound)
        );
    }

    #[test]
    fn heuristic_is_zero_from_same_point() {
        let point = GridPosition { x: 1, y: 1 };

        assert_eq!(
            heuristic(point, point),
            0
        );
    }

    #[test]
    fn heuristic_returns_manhattan_distance() {
        let from = GridPosition { x: 1, y: 2 };
        let to = GridPosition { x: 4, y: 6 };

        assert_eq!(
            heuristic(from, to),
            7
        );
    }
}