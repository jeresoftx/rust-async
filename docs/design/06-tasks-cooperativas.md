# Diseño: Tasks Cooperativas

**Curso:** Rust Async · **Capítulo:** 06 · **Estado:** draft

Una task es un future que el executor administra como unidad de scheduling. La
cooperación exige que cada paso devuelva control; una tarea CPU-bound que no
cede puede retrasar a todas las demás.

## Invariantes

1. La cola FIFO atiende tareas en el orden de inserción del modelo.
2. Una tarea pendiente se reencola al final, no monopoliza el turno actual.
3. Una tarea lista se retira.
4. La equidad es local al modelo: no promete prioridad, deadlines ni reparto
   de CPU entre hilos.

El capítulo implementará una cola determinista de tareas de cuenta regresiva.
Un runtime de producción necesita wakeups, I/O y políticas adicionales.
