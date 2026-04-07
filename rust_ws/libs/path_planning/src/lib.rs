use grid_map::{GridMap, GridPosition};

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

        Err(PathPlanningError::NoPathFound)
    }
}

#[cfg(test)]
mod test {
    use super::*;


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
    fn returns_no_path_found_for_different_points_in_stub_version() {
        let planner = AStartPlanner;
        let map = GridMap::new(3, 3);

        assert_eq!(
            planner.plan(
                &map,
                GridPosition { x: 0, y: 0 },
                GridPosition { x: 2, y: 2 },
            ),
            Err(PathPlanningError::NoPathFound)
        );
    }
}