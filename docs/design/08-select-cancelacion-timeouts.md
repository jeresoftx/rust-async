# Diseño: select!, Cancelación y Timeouts

**Curso:** Rust Async · **Capítulo:** 08 · **Estado:** draft

`select!` coordina varios futures y continúa con la primera rama preparada. Un
timeout convierte una espera potencialmente ilimitada en un resultado explícito.
La cancelación ocurre, normalmente, al soltar un future pendiente: por eso los
futures deben mantener invariantes correctas en cada punto de suspensión.

## Invariantes

1. Una rama ganadora no prueba que las otras hayan terminado.
2. Un timeout distingue expiración de error interno y éxito.
3. Cancelar no debe dejar recursos compartidos en estado inválido.
4. La equidad de `select!` no se presupone sin consultar el contrato del macro.

El modelo del siguiente issue usará `tokio::time::timeout` y una tarea inmediata
para mostrar resultados deterministas, sin depender de sleeps frágiles.
