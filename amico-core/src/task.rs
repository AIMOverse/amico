//! Task management module for scheduling and executing recurring or one-off tasks.
//!
//! This module provides functionality to create and run tasks with optional intervals
//! and delays. Tasks can be configured to run once or repeatedly with either constant
//! or random intervals between executions.
//!
//! # Examples
//!
//! ```
//! use std::time::Duration;
//! use amico_core::task::{Task, TaskInterval};
//!
//! # async fn example() {
//! // Create a task that runs every 5 seconds
//! let task = Task::new(
//!     "periodic_task".to_string(),
//!     Some(TaskInterval::Constant(Duration::from_secs(5))),
//!     None,
//!     "context"
//! );
//!
//! // Run the task with a closure
//! task.run(|ctx| async move {
//!     println!("Running task with context: {}", ctx);
//!     Ok(())
//! }).await;
//! # }
//! ```

use std::time::Duration;
use tokio::time;

use crate::error::AmicoError;

/// A task that can be executed once or periodically.
///
/// # Type Parameters
///
/// * `T` - The type of the context data that will be passed to the task execution function.
///         Must implement `Clone + Send + 'static`.
///
/// # Fields
///
/// * `name` - A unique identifier for the task
/// * `interval` - Optional scheduling interval for recurring tasks
/// * `delay` - Optional initial delay before first execution
/// * `ctx` - Context data passed to the task execution function
#[derive(Clone)]
pub struct Task<T>
where
    T: Clone + Send + 'static,
{
    pub name: String,
    pub interval: Option<TaskInterval>,
    pub delay: Option<Duration>,
    pub ctx: T,
}

/// Defines how frequently a task should be executed.
///
/// # Variants
///
/// * `Constant` - Task runs at fixed time intervals
/// * `Random` - Task runs at random intervals between min and max duration
#[derive(Clone)]
pub enum TaskInterval {
    /// Fixed duration between task executions
    Constant(Duration),
    /// Random duration between min and max values
    Random(Duration, Duration),
}

impl<T> Task<T>
where
    T: Clone + Send + 'static,
{
    /// Creates a new task with the given parameters.
    ///
    /// # Parameters
    ///
    /// * `name` - Unique identifier for the task
    /// * `interval` - Optional scheduling interval for recurring tasks
    /// * `delay` - Optional initial delay before first execution
    /// * `ctx` - Context data passed to the task execution function
    pub fn new(
        name: String,
        interval: Option<TaskInterval>,
        delay: Option<Duration>,
        ctx: T,
    ) -> Self {
        Self {
            name,
            interval,
            delay,
            ctx,
        }
    }

    /// Runs the task with the given execution function.
    ///
    /// This method first checks if a delay is specified for the task. If a delay is present,
    /// the task will sleep for the specified duration before executing. After the delay,
    /// the task will check its interval. If no interval is specified, the task will run once
    /// and then complete. If an interval is specified, the task will run repeatedly at the
    /// specified interval.
    ///
    /// # Parameters
    ///
    /// * `f` - Closure that will be executed with the task context
    ///
    pub async fn run<F, Fut>(self, f: F)
    where
        F: Fn(T) -> Fut + Send + Clone + 'static,
        Fut: std::future::Future<Output = Result<(), AmicoError>> + Send + 'static,
    {
        match self.delay {
            None => {}
            Some(delay) => tokio::time::sleep(delay).await,
        }

        match self.interval {
            None => {
                // Run once
                if let Err(e) = f(self.ctx).await {
                    tracing::error!("Task '{}' failed: {:?}", self.name, e)
                } else {
                    tracing::info!("Task '{}' completed successfully", self.name)
                }
            }
            Some(interval) => {
                // Spawn a new task that runs repeatedly
                let name = self.name.clone();

                tokio::spawn(async move {
                    loop {
                        if let Err(e) = f(self.ctx.clone()).await {
                            match e {
                                AmicoError::TaskError(name, msg) => {
                                    tracing::error!("Task '{}' failed: {}", name, msg);
                                }
                                _ => {
                                    tracing::error!("Task '{}' failed: {:?}", name, e);
                                }
                            }
                        } else {
                            tracing::info!("Task '{}' completed successfully", name);
                        }

                        let sleep_duration = match interval {
                            TaskInterval::Constant(duration) => duration,
                            TaskInterval::Random(min_duration, max_duration) => {
                                use rand::Rng;
                                rand::thread_rng().gen_range(min_duration..max_duration)
                            }
                        };

                        tracing::info!(
                            "Task '{}' next execution scheduled in {:.1} minutes...",
                            name,
                            sleep_duration.as_secs_f64() / 60.0
                        );

                        time::sleep(sleep_duration).await;
                    }
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use ::time::OffsetDateTime;

    use super::*;

    #[tokio::test]
    async fn test_delay() {
        let task = Task::<()>::new(
            "test".to_string(),
            None, // only run once
            Some(Duration::from_secs(1)),
            (),
        );

        println!(
            "Calling task to run.\nCurrent timestamp: {}",
            OffsetDateTime::now_utc()
        );

        task.run(|_| async {
            println!(
                "Task runs.\nCurrent timestamp: {}",
                OffsetDateTime::now_utc()
            );
            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn test_task_error() {
        let task = Task::<()>::new(
            "test".to_string(),
            None, // only run once
            None,
            (),
        );

        task.run(|_| async {
            Err(AmicoError::TaskError(
                "test task".to_string(),
                "This error is intentional".to_string(),
            ))
        })
        .await;
    }
}
