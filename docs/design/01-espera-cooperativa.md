# Diseño: De Espera Bloqueante a Trabajo Cooperativo

**Curso:** Rust Async · **Capítulo:** 01 · **Estado:** draft

Esta nota fija el razonamiento que guiará el primer capítulo. No es todavía el
capítulo publicable: faltan el modelo ejecutable, sus pruebas, los ejemplos,
los ejercicios y el diagrama que completarán las entregas posteriores.

## Concepto

La espera cooperativa permite que una tarea que no puede progresar ahora ceda
el control sin retener el recurso que ejecuta otras tareas. Más tarde, cuando
el evento esperado ocurra, la tarea puede volver a intentarlo.

No significa "hacer dos cosas a la vez". La concurrencia asíncrona coordina
progreso intercalado; el paralelismo requiere recursos de ejecución realmente
simultáneos. Un runtime puede ofrecer ambos, pero son decisiones separadas.

## Problema

Una operación de E/S suele pasar más tiempo esperando una respuesta que usando
CPU. Si cada espera bloquea un hilo, el número de conexiones simultáneas queda
atado al número de hilos disponibles. Aumentar hilos puede aliviar el síntoma,
pero agrega memoria por pila, cambios de contexto, contención y límites
operativos.

El problema no es que esperar sea incorrecto. El problema es ocupar un hilo
mientras no hay trabajo útil que hacer. La asincronía crea una frontera:
registrar el interés por el evento, devolver el control y reanudar solo cuando
exista una razón para intentarlo de nuevo.

## Invariantes

El modelo educativo del capítulo debe preservar estas reglas:

1. Una tarea nunca se declara terminada mientras aún pueda requerir progreso.
2. Una tarea pendiente no realiza espera activa ni monopoliza el executor.
3. La reanudación solo se solicita ante un evento observable o una decisión
   explícita del scheduler.
4. Volver a sondear una tarea debe ser seguro para su contrato documentado.
5. La cancelación o abandono de una tarea libera sus recursos de forma
   comprensible; no deja trabajo oculto sin dueño.
6. El orden de reanudación no se promete salvo que el modelo lo declare.

Estas reglas distinguen un sistema cooperativo de un bucle que consulta
repetidamente un estado sin ceder ejecución.

## Alternativas

| Alternativa | Ventaja | Costo o límite | Cuándo elegirla |
|---|---|---|---|
| Código secuencial bloqueante | Modelo mental simple | Un hilo queda ocupado durante la espera | Herramientas pequeñas, baja concurrencia o APIs inevitablamente bloqueantes |
| Un hilo por conexión | Aislamiento directo y depuración familiar | Memoria, scheduler del SO y escalamiento limitado | Concurrencia moderada y trabajo CPU-bound por conexión |
| Pool de hilos | Limita la creación de hilos | Puede atascarse si todas las tareas bloquean | Trabajo bloqueante controlado o adaptación de APIs heredadas |
| Eventos + callbacks | Uso eficiente de recursos | Flujo de control fragmentado y manejo de errores difícil | APIs de bajo nivel o interoperabilidad existente |
| Espera cooperativa | Muchas operaciones de E/S con pocos hilos | Requiere contratos explícitos de progreso y cancelación | Servicios con muchas esperas independientes |

La espera cooperativa no reemplaza un pool de hilos para trabajo intensivo de
CPU. Si una tarea no cede durante un cálculo largo, perjudica a las demás igual
que una llamada bloqueante. El capítulo posterior mostrará cómo desplazar ese
trabajo sin presentar la asincronía como solución universal.

## Justificación Del Orden Del Curso

Antes de presentar `Future`, `Poll`, `Waker` o `Pin`, el lector necesita poder
nombrar la necesidad que resuelven: una tarea debe expresar "todavía no", ceder
el executor y recibir una oportunidad de continuar. Los capítulos 02 a 04
formalizan ese contrato; el capítulo 05 construye el executor que lo consume.

Este orden evita aprender tipos y macros como si fueran magia. Primero aparece
el costo de bloquear; después el protocolo de progreso; al final, las
herramientas de producción.

## Límites Del Capítulo

- No implementa un runtime de producción.
- No agrega Tokio ni otra dependencia externa.
- No usa `unsafe` ni modela datos auto-referenciales; esa frontera pertenece al
  capítulo sobre `Pin`.
- No promete equidad, paralelismo ni cancelación automática sin un contrato
  posterior que lo sustente.

## Referencias De Continuidad

- [Documentación de `Future`](https://doc.rust-lang.org/std/future/trait.Future.html)
- [Módulo `std::task`](https://doc.rust-lang.org/std/task/)
- [Plan del curso](../superpowers/plans/2026-07-28-rust-async-course.md)
