# Diseño: Executor Mínimo

**Curso:** Rust Async · **Capítulo:** 05 · **Estado:** draft

El executor es quien entrega oportunidades de progreso: toma una tarea de una
cola, la sondea y retira solo las tareas que terminan. El future conserva su
estado; el executor no interpreta su resultado.

## Invariantes

1. Una tarea pendiente vuelve a la cola únicamente si el modelo declara que
   habrá otra oportunidad de progreso.
2. Una tarea `Ready` se retira y no se sondea de nuevo.
3. La cola es explícita; el orden será FIFO para este modelo didáctico.
4. Un paso del executor es finito y no bloquea por E/S.

## Alternativas y Límites

Un loop manual es suficiente para una sola operación; un runtime de producción
añade wakers, I/O, temporizadores, concurrencia y políticas de equidad. Este
executor solo demuestra la frontera task/scheduler y no pretende competir con
Tokio.
