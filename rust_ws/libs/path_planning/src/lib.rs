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