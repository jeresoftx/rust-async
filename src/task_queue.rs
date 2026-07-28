//! Cola FIFO educativa para tareas cooperativas.

use crate::educational_future::CountdownFuture;
use std::collections::VecDeque;
use std::task::Poll;

/// Ejecuta un sondeo FIFO por paso y reencola tareas pendientes.
#[derive(Debug, Default)]
pub struct TaskQueue {
    tasks: VecDeque<CountdownFuture>,
}

impl TaskQueue {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
        }
    }

    pub fn push(&mut self, task: CountdownFuture) {
        self.tasks.push_back(task);
    }

    /// Ejecuta una tarea; devuelve si el paso retiró una tarea terminada.
    pub fn step(&mut self) -> bool {
        let Some(mut task) = self.tasks.pop_front() else {
            return false;
        };
        if matches!(task.poll(), Poll::Ready(_)) {
            true
        } else {
            self.tasks.push_back(task);
            false
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}
