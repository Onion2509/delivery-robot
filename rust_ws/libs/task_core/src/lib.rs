use grid_map::GridPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Pending,
    Assigned,
    Navigating,
    Completed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeliveryTask {
    id: u64,
    destination: GridPosition,
    state: TaskState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskError {
    NotPending,
    NotAssigned,
    NotNavigating,
    AlreadyCompleted,
    AlreadyFailed,
}

impl DeliveryTask {
    pub fn new(id: u64, destination: GridPosition) -> Self {
        Self {
            id,
            destination,
            state: TaskState::Pending,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn destination(&self) -> GridPosition {
        self.destination
    }

    pub fn state(&self) -> TaskState {
        self.state
    }

    pub fn assign(&mut self) -> Result<(), TaskError> {
        if self.state == TaskState::Pending {
            self.state = TaskState::Assigned;
            Ok(())
        } else {
            Err(TaskError::NotPending)
        }
    }

    pub fn start_navigation(&mut self) -> Result<(), TaskError> {
        if self.state == TaskState::Assigned {
            self.state = TaskState::Navigating;
            Ok(())
        } else {
            Err(TaskError::NotAssigned)
        }
    }

    pub fn complete(&mut self) -> Result<(), TaskError> {
        if self.state == TaskState::Navigating {
            self.state = TaskState::Completed;
            Ok(())
        } else {
            Err(TaskError::NotNavigating)
        }
    }

    pub fn fail(&mut self) -> Result<(), TaskError> {
        match self.state {
            TaskState::Pending | TaskState::Assigned | TaskState::Navigating => {
                self.state = TaskState::Failed;
                Ok(())
            }
            TaskState::Completed => Err(TaskError::AlreadyCompleted),
            TaskState::Failed => Err(TaskError::AlreadyFailed),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_task_starts_pending() {
        let destination = GridPosition { x:2, y:3 };
        let task = DeliveryTask::new(7, destination);

        assert_eq!(task.id(), 7);
        assert_eq!(task.destination, destination);
        assert_eq!(task.state(), TaskState::Pending);
    }

    #[test]
    fn assign_moves_pending_to_assigned() {
        let destination = GridPosition { x: 1, y: 1 };
        let mut task = DeliveryTask::new(1, destination);

        assert_eq!(task.assign(), Ok(()));
        assert_eq!(task.state(), TaskState::Assigned);
    }

    #[test]
    fn start_navigation_requires_assigned() {
        let destination = GridPosition { x: 1, y: 1 };
        let mut task = DeliveryTask::new(1, destination);

        assert_eq!(task.start_navigation(), Err(TaskError::NotAssigned));
    }

    #[test]
    fn complete_moves_navigating_to_completed() {
        let destination = GridPosition { x: 4, y: 2 };
        let mut task = DeliveryTask::new(2, destination);

        task.assign().unwrap();
        task.start_navigation().unwrap();

        assert_eq!(task.complete(), Ok(())); 
        assert_eq!(task.state(), TaskState::Completed);  
    }

    #[test]
    fn fail_moves_active_task_to_failed() {
        let destination = GridPosition { x: 5, y: 5 };
        let mut task = DeliveryTask::new(3, destination);

        task.assign().unwrap();

        assert_eq!(task.fail(), Ok(()));
        assert_eq!(task.state(), TaskState::Failed);
    }

    #[test]
    fn fail_cannot_run_twice() {
        let destination = GridPosition { x: 5, y: 5 };
        let mut task = DeliveryTask::new(3, destination);

        task.fail().unwrap();

        assert_eq!(task.fail(), Err(TaskError::AlreadyFailed));
    }
}