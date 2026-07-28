# Diseño: Waker y Context

**Curso:** Rust Async · **Capítulo:** 03 · **Estado:** draft

Un `Waker` separa el estado pendiente de la decisión de volver a sondearlo: la
operación avisa que puede progresar, y el executor decide cuándo ofrecer otra
oportunidad. `Context` entrega esa capacidad durante `poll` sin acoplar el
future a un runtime concreto.

## Invariantes

1. Despertar no completa una operación por sí mismo; solo solicita sondeo.
2. Varias notificaciones pueden coalescerse en una sola oportunidad.
3. Un future pendiente debe registrar interés antes de devolver `Pending`.
4. El executor conserva la responsabilidad por la cola y el orden.

Sin waker, reintentar por temporizador puede funcionar como demostración, pero
desperdicia trabajo. Callbacks mezclan operación y scheduler; el par
`Context`/`Waker` mantiene esa frontera explícita.
