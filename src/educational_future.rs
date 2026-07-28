//! Future educativo que muestra el contrato `Pending` / `Ready` sin runtime.

use std::task::Poll;

/// Operación que requiere un número fijo de oportunidades de progreso.
#[derive(Debug, Clone, Copy)]
pub struct CountdownFuture {
    remaining_pending_polls: u8,
    completed: bool,
}

impl CountdownFuture {
    /// Crea una operación que devolverá `Pending` exactamente `pending_polls` veces.
    #[must_use]
    pub const fn new(pending_polls: u8) -> Self {
        Self {
            remaining_pending_polls: pending_polls,
            completed: false,
        }
    }

    /// Intenta progresar sin bloquear.
    pub fn poll(&mut self) -> Poll<u8> {
        if self.completed {
            return Poll::Ready(0);
        }
        if self.remaining_pending_polls > 0 {
            self.remaining_pending_polls -= 1;
            return Poll::Pending;
        }
        self.completed = true;
        Poll::Ready(0)
    }
}
