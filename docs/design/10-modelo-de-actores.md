# Diseño: Modelo de Actores

**Curso:** Rust Async · **Capítulo:** 10 · **Estado:** draft

Un actor posee su estado y recibe mensajes secuencialmente desde un canal. En
lugar de compartir una estructura mutable entre muchas tareas, cada mensaje
solicita una transición y el actor conserva el orden de sus propias decisiones.

## Problema

Un canal por sí solo mueve mensajes, pero no decide quién posee el estado que
esos mensajes afectan. Si varios consumidores mutan el mismo valor, reaparecen
los contratos de bloqueo, orden y recuperación de errores. El modelo de actor
reduce esa superficie: una sola tarea es propietaria del estado y del protocolo
que lo cambia.

## Alternativas

- Un `Mutex` protege estado compartido con acceso directo; es razonable para
  secciones pequeñas, pero cada llamador participa en el protocolo de bloqueo.
- Un canal simple comunica trabajo, sin definir el estado ni las respuestas
  asociadas al trabajo.
- Un actor combina canal, propiedad y ciclo de vida; añade latencia de cola y
  no vuelve paralelas las transiciones de un mismo actor.
- Una base de datos o cola persistente resuelve durabilidad y recuperación que
  este modelo en memoria no pretende ofrecer.

## Invariantes

1. Solo la tarea del actor modifica su estado interno.
2. Cada mensaje se procesa como una transición completa antes del siguiente.
3. La respuesta de una consulta corresponde al estado observado por el actor
   al procesar esa consulta, no al estado que el emisor suponía tener.
4. Al cerrarse todos los emisores, el actor puede terminar después de procesar
   los mensajes ya aceptados.
5. Si el actor termina antes de responder, el solicitante observa la falta de
   respuesta como un resultado explícito; no se inventa una respuesta.
6. El buzón acotado mantiene la presión de cola definida en el capítulo 09.

## Fallas y Límites

El actor educativo no supervisa reinicios, no persiste mensajes y no implementa
un árbol de supervisión. Un `JoinHandle` con error de ejecución y un canal de
respuesta cerrado son señales distintas: la primera pertenece a la task y la
segunda a una solicitud concreta. El capítulo las nombra, pero no promete
recuperación automática.

## Decisión Educativa

El siguiente modelo usará un contador y un `enum` de mensajes con operaciones
de incremento y consulta. Las consultas llevarán un `oneshot::Sender`, que
expone que una respuesta pertenece a una sola petición. Se conserva Tokio ya
autorizado, sin `unsafe` ni dependencias adicionales.
