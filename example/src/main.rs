use std::thread::sleep;
use std::time::Duration;
/// Represents the state of a task
#[derive(Debug, PartialEq)]
enum TaskState {
    Pending,
    InProgress,
    Done,
}

/// A simple task with an id and description
#[derive(Debug)]
struct Task {
    id: u32,
    description: String,
    state: TaskState,
}

/// Trait defining behavior of things that can be "run"
trait Runnable {
    fn run(&mut self);
}

impl Runnable for Task {
    fn run(&mut self) {
        self.state = TaskState::InProgress;
    }
}

/// Async trait for things that can be "completed"
trait Completable {
    fn complete(&mut self);
}

impl Completable for Task {
    fn complete(&mut self) {
        sleep(Duration::from_millis(100));
        self.state = TaskState::Done;
    }
}

fn main() {
    let mut task = Task {
        id: 1,
        description: "Learn Rust unit tests 🚀".to_string(),
        state: TaskState::Pending,
    };

    println!("Before run: {:?}", task);
    task.run();
    println!("After run: {:?}", task);

    task.complete();
    println!("After complete: {:?}", task);
}

use rusty_check::rusty_check;
rusty_check! {
    use super::*;
    case task_initial_state {
            given {
                task = Task {
                    id: 1,
                    description: "demo".to_string(),
                    state: TaskState::Pending,
                }
            }
            check {
                task.state equal TaskState::Pending
            }
        }

        case task_after_run {
            given {
                mut task = Task {
                    id: 2,
                    description: "run".to_string(),
                    state: TaskState::Pending,
                }
            }
            do {
                    task.run();
            }
            check {
                    task.state equal TaskState::InProgress
            }
        }

        case loop_over_vec_of_tasks {
            given {
                tasks = vec![
                    Task { id: 1, description: "A".into(), state: TaskState::Pending },
                    Task { id: 2, description: "B".into(), state: TaskState::Pending },
                ]
            }
            check {
                for each t in &tasks, t.state equal TaskState::Pending
            }
        }
}
