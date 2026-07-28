//! Modelo educativo de un actor que posee un contador.

use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;

/// Mensajes que entiende el actor contador.
pub enum CounterMessage {
    /// Suma una cantidad al estado que posee el actor.
    Increment(u64),
    /// Devuelve el valor observado por el actor al procesar la consulta.
    Get(oneshot::Sender<u64>),
}

/// Inicia un actor contador y devuelve su buzón y la tarea que lo ejecuta.
///
/// El actor termina al cerrar todos los emisores. `capacity` conserva el
/// backpressure de un canal acotado.
#[must_use]
pub fn spawn_counter(capacity: usize) -> (mpsc::Sender<CounterMessage>, JoinHandle<()>) {
    let (sender, mut receiver) = mpsc::channel(capacity);
    let task = tokio::spawn(async move {
        let mut count = 0_u64;

        while let Some(message) = receiver.recv().await {
            match message {
                CounterMessage::Increment(amount) => count += amount,
                CounterMessage::Get(reply) => {
                    let _ = reply.send(count);
                }
            }
        }
    });

    (sender, task)
}
