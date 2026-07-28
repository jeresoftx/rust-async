//! Señal educativa que representa una solicitud coalescida de re-sondeo.

/// Una notificación pendiente de atención por un executor.
#[derive(Debug, Default)]
pub struct WakeSignal {
    pending: bool,
}

impl WakeSignal {
    /// Crea una señal sin notificaciones pendientes.
    #[must_use]
    pub const fn new() -> Self {
        Self { pending: false }
    }

    /// Solicita un nuevo sondeo. Las solicitudes repetidas se coalescen.
    pub fn wake(&mut self) {
        self.pending = true;
    }

    /// Consume la solicitud pendiente, si existe.
    pub fn take(&mut self) -> bool {
        let pending = self.pending;
        self.pending = false;
        pending
    }
}
