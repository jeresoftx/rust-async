//! Modelo mínimo de progreso cooperativo.
//!
//! Una tarea pendiente no se completa por ser sondeada repetidamente. Debe
//! recibir una notificación explícita antes de poder devolver [`Progress::Ready`].

/// Resultado observable de sondear una tarea cooperativa.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Progress {
    /// La tarea cedió el control y espera una notificación.
    Pending,
    /// La tarea terminó su trabajo y no volverá a estar pendiente.
    Ready,
}

/// Tarea educativa que separa el sondeo de la notificación del evento.
#[derive(Debug, Default)]
pub struct CooperativeTask {
    notified: bool,
    completed: bool,
}

impl CooperativeTask {
    /// Crea una tarea que todavía no tiene un evento para procesar.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            notified: false,
            completed: false,
        }
    }

    /// Registra que el evento esperado ocurrió.
    ///
    /// Notificar varias veces es idempotente: la tarea necesita una oportunidad
    /// de progresar, no una cola de despertares artificiales.
    pub fn notify(&mut self) {
        self.notified = true;
    }

    /// Intenta progresar sin bloquear al llamador.
    ///
    /// Devuelve [`Progress::Pending`] hasta que exista una notificación. Tras
    /// completarse, la tarea conserva [`Progress::Ready`] en sondeos posteriores.
    pub fn poll(&mut self) -> Progress {
        if self.completed {
            return Progress::Ready;
        }

        if !self.notified {
            return Progress::Pending;
        }

        self.completed = true;
        Progress::Ready
    }
}

#[cfg(test)]
mod tests {
    use super::{CooperativeTask, Progress};

    #[test]
    fn repeated_notifications_do_not_change_completion() {
        let mut task = CooperativeTask::new();
        task.notify();
        task.notify();

        assert_eq!(task.poll(), Progress::Ready);
        assert_eq!(task.poll(), Progress::Ready);
    }
}
