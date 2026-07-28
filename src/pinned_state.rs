//! Modelo seguro que separa estabilidad de ubicación y avance de estado.

/// Estado educativo que solo permite avanzar su contador interno.
#[derive(Debug)]
pub struct PinnedState<T> {
    value: T,
    steps: usize,
}

impl<T> PinnedState<T> {
    /// Crea el estado con un valor estable para el modelo.
    #[must_use]
    pub const fn new(value: T) -> Self {
        Self { value, steps: 0 }
    }

    /// Obtiene el valor sin reemplazarlo.
    #[must_use]
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Registra una transición interna segura.
    pub fn advance(&mut self) {
        self.steps += 1;
    }

    /// Devuelve el número de transiciones realizadas.
    #[must_use]
    pub const fn steps(&self) -> usize {
        self.steps
    }
}
