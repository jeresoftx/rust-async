//! Executor educativo para una sola tarea de cuenta regresiva.

use crate::educational_future::CountdownFuture;
use std::task::Poll;

/// Ejecuta una tarea mediante pasos explícitos y no bloqueantes.
#[derive(Debug)]
pub struct SingleTaskExecutor {
    task: CountdownFuture,
    complete: bool,
}

impl SingleTaskExecutor {
    /// Crea un executor con una tarea pendiente.
    #[must_use]
    pub const fn new(task: CountdownFuture) -> Self {
        Self {
            task,
            complete: false,
        }
    }

    /// Sondea una vez la tarea y devuelve si ya terminó.
    pub fn step(&mut self) -> bool {
        if self.complete {
            return true;
        }
        self.complete = matches!(self.task.poll(), Poll::Ready(_));
        self.complete
    }

    /// Indica si la tarea ya fue retirada de ejecución.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.complete
    }
}
