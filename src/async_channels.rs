//! Modelos mínimos de canales acotados con Tokio.

use tokio::sync::mpsc;

/// Comprueba que un canal acotado comunica la falta de capacidad.
///
/// La función usa `try_send` para observar presión de cola sin introducir una
/// espera temporal: el segundo mensaje no cabe mientras nadie reciba el primero.
pub fn bounded_channel_applies_backpressure() -> bool {
    let (sender, _receiver) = mpsc::channel(1);

    sender.try_send(1).is_ok() && sender.try_send(2).is_err()
}

/// Envía mensajes, cierra todos los emisores y drena la cola en orden FIFO.
pub async fn drain_after_senders_close() -> Vec<u8> {
    let (sender, mut receiver) = mpsc::channel(2);
    sender.send(1).await.expect("el receptor sigue abierto");
    sender.send(2).await.expect("el receptor sigue abierto");
    drop(sender);

    let mut received = Vec::new();
    while let Some(message) = receiver.recv().await {
        received.push(message);
    }

    received
}

/// Indica que un emisor no puede confirmar entrega tras el cierre del receptor.
pub async fn receiver_closure_rejects_send() -> bool {
    let (sender, receiver) = mpsc::channel(1);
    drop(receiver);

    sender.send(1).await.is_err()
}
