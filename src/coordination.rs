//! Modelos deterministas para select, cancelación y timeouts con Tokio.

use std::time::Duration;

/// Devuelve la primera rama disponible sin depender de una espera real.
pub async fn first_ready() -> &'static str {
    tokio::select! {
        value = async { "immediate" } => value,
        _ = std::future::pending::<()>() => "pending",
    }
}

/// Aplica un timeout a una operación que ya puede terminar.
pub async fn immediate_with_timeout() -> Result<u8, tokio::time::error::Elapsed> {
    tokio::time::timeout(Duration::from_millis(10), async { 1_u8 }).await
}
