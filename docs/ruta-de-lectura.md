# Ruta de Lectura

Esta ruta conserva el orden de dependencias del curso. No es una lista de APIs
para memorizar: cada paso responde una pregunta que el siguiente necesita.

| Tramo | Pregunta | Capítulos | Resultado esperado |
|---|---|---|---|
| Protocolo | ¿Cómo puede una operación pausar sin bloquear un hilo? | [01](./01-espera-bloqueante-y-cooperacion.md), [02](./02-future-y-poll.md), [03](./03-waker-y-context.md), [04](./04-pin-y-datos-auto-referenciales.md) | Distinguir progreso, sondeo, notificación y estabilidad de memoria. |
| Runtime educativo | ¿Quién vuelve a sondear y con qué política? | [05](./05-executor-minimo.md), [06](./06-tasks-y-concurrencia-cooperativa.md) | Explicar executor, task, cola y límites de la cooperación. |
| Producción y coordinación | ¿Cómo se aplican estos contratos con un runtime real? | [07](./07-tokio-y-runtime-de-produccion.md), [08](./08-select-cancelacion-y-timeouts.md), [09](./09-canales-y-sincronizacion-asincrona.md) | Elegir runtime, combinar operaciones y acotar flujo mediante canales. |
| Composición | ¿Quién posee el estado y el protocolo de mensajes? | [10](./10-modelo-de-actores.md) | Delimitar un actor, su buzón, sus respuestas y sus fallas. |

## Lecturas Según La Necesidad

- Si vienes de código secuencial, inicia en el capítulo 01 y no pases a Tokio
  hasta poder explicar `Pending`, `Ready` y una notificación.
- Si ya usas Tokio pero los timeouts o `select!` son confusos, repasa los
  capítulos 02 y 03 antes de los capítulos 07 y 08.
- Si el problema es acumulación de trabajo, estudia primero el capítulo 09:
  la capacidad del canal es una decisión de producto y operación, no un detalle
  sintáctico.
- Si varias tareas quieren modificar el mismo agregado, conecta el capítulo 09
  con el 10 y compara actor, canal simple y estado con `Mutex`.

## Fronteras Del Curso

`rust-concurrency` conserva el canon de hilos y paralelismo. Este curso estudia
la cooperación alrededor de esperas y la coordinación de tareas. Ninguna ruta
autoriza asumir que asincronía acelera cálculo intensivo de CPU.

Consulta el [glosario](./glosario.md) cuando una palabra del protocolo aparezca
antes de sentirse natural.
