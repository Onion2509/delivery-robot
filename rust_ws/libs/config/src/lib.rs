use grid_map::GridPosition;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppConfig {
    pub map_width: usize,
    pub map_height: usize,
    pub start: GridPosition,
    pub goal: GridPosition,
    pub obstacles: Vec<GridPosition>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigError {
    ZeroWidth,
    ZeroHeight,
    StartOutOfBounds,
    GoalOutOfBounds,
    ObstacleOutOfBounds,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            map_width: 3,
            map_height: 3,
            start: GridPosition { x: 0, y: 0 },
            goal: GridPosition { x: 2, y: 2 },
            obstacles: vec![
                GridPosition { x: 1, y: 0 },
                GridPosition { x: 1, y: 1 },
            ],
        }
    }
}

impl AppConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.map_width == 0 {
            return Err(ConfigError::ZeroWidth);
        }

        if self.map_height == 0 {
            return Err(ConfigError::ZeroHeight);
        }

        if self.start.x >= self.map_width || self.start.y >= self.map_height {
            return Err(ConfigError::StartOutOfBounds);
        }

        if self.goal.x >= self.map_width || self.goal.y >= self.map_height {
            return Err(ConfigError::GoalOutOfBounds);
        }

        for obstacle in &self.obstacles {
            if obstacle.x >= self.map_width || obstacle.y >= self.map_height {
                return Err(ConfigError::ObstacleOutOfBounds);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_valid() {
        let config = AppConfig::default();

        assert_eq!(config.validate(), Ok(()));
    }

    #[test]
    fn zero_width_is_invalid() {
        let mut config = AppConfig::default();
        config.map_width = 0;

        assert_eq!(config.validate(), Err(ConfigError::ZeroWidth));
    }

    #[test]
    fn zero_height_is_invalid() {
        let mut config = AppConfig::default();
        config.map_height = 0;

        assert_eq!(config.validate(), Err(ConfigError::ZeroHeight));
    }

    #[test]
    fn start_must_be_in_bounds() {
        let mut config = AppConfig::default();
        config.start = GridPosition { x: 99, y: 0 };

        assert_eq!(config.validate(), Err(ConfigError::StartOutOfBounds));
    }

    #[test]
    fn goal_must_be_in_bounds() {
        let mut config = AppConfig::default();
        config.goal = GridPosition { x: 99, y: 0 };

        assert_eq!(config.validate(), Err(ConfigError::GoalOutOfBounds));
    }

    #[test]
    fn obstacles_must_be_in_bounds() {
        let mut config = AppConfig::default();
        config.obstacles.push(GridPosition { x: 99, y: 0 });

        assert_eq!(config.validate(), Err(ConfigError::ObstacleOutOfBounds));
    }
}