# Diseño: Future y Poll

**Curso:** Rust Async · **Capítulo:** 02 · **Estado:** draft

## Concepto Y Problema

Un `Future` representa una operación cuyo resultado puede no estar disponible
todavía. `poll` no espera: pregunta si la operación puede progresar ahora y
responde con `Poll::Pending` o `Poll::Ready(valor)`.

Ese protocolo evita que el executor tenga que conocer cada tipo de operación.
El executor ofrece oportunidades de progreso; el future conserva su estado y
declara si terminó. Un `Pending` no autoriza al executor a girar en un bucle:
el capítulo siguiente introduce el mecanismo de notificación.

## Invariantes

1. `Ready` contiene el resultado terminal y no vuelve a `Pending`.
2. `Pending` conserva estado suficiente para continuar más tarde.
3. Sondear no bloquea ni realiza espera activa.
4. El caller no infiere un orden de sondeo ni equidad a partir del trait.
5. Sin un waker, un modelo educativo puede reintentar de manera controlada,
   pero no representa un runtime eficiente.

## Alternativas

Callbacks trasladan el estado a cierres; hilos bloqueantes trasladan la espera
al SO; generators requieren soporte de lenguaje distinto. `Future` concentra
el estado de una operación en un valor componible y deja el scheduling afuera.

## Justificación

El modelo del issue #9 usará un future de cuenta regresiva con un `poll` manual
y determinista. No implementará wakers ni `Pin`; así enseña el contrato mínimo
sin ocultar las piezas que se estudian después.
