# Glosario

| Término | Definición operativa | Ver también |
|---|---|---|
| Actor | Task que posee estado y procesa mensajes secuencialmente. | [Capítulo 10](./10-modelo-de-actores.md) |
| Backpressure | Señal que limita al productor cuando una cola acotada no tiene capacidad. | [Capítulo 09](./09-canales-y-sincronizacion-asincrona.md) |
| Buzón | Canal por el que un actor recibe mensajes. | [Capítulo 10](./10-modelo-de-actores.md) |
| Cancelación | Abandono de un future pendiente, normalmente al soltarlo; exige invariantes válidas en cada suspensión. | [Capítulo 08](./08-select-cancelacion-y-timeouts.md) |
| Context | Valor que acompaña a `poll` y entrega el `Waker` actual al future. | [Capítulo 03](./03-waker-y-context.md) |
| Executor | Componente que sondea futures y decide cuándo volver a ejecutarlos. | [Capítulo 05](./05-executor-minimo.md) |
| Future | Valor que representa un resultado que puede no estar listo todavía. | [Capítulo 02](./02-future-y-poll.md) |
| `Pin` | Restricción que protege la ubicación de un valor cuando moverlo rompería su contrato. | [Capítulo 04](./04-pin-y-datos-auto-referenciales.md) |
| `Poll` | Resultado de sondear un future: `Pending` o `Ready`. | [Capítulos 01–02](./02-future-y-poll.md) |
| Runtime | Infraestructura que ejecuta tasks, integra E/S y ofrece temporizadores y sincronización. | [Capítulo 07](./07-tokio-y-runtime-de-produccion.md) |
| Task | Future administrado como unidad de ejecución por un runtime o executor. | [Capítulo 06](./06-tasks-y-concurrencia-cooperativa.md) |
| Timeout | Límite explícito de espera que distingue una expiración de un éxito. | [Capítulo 08](./08-select-cancelacion-y-timeouts.md) |
| Waker | Mecanismo con el que una fuente de eventos avisa que un future podría progresar. | [Capítulo 03](./03-waker-y-context.md) |

Las definiciones describen el contrato usado por el curso. No sustituyen la
documentación de Rust o Tokio cuando un programa de producción depende de los
detalles de una API concreta.
