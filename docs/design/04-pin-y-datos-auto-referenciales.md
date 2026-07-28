# Diseño: Pin y Datos Auto-Referenciales

**Curso:** Rust Async · **Capítulo:** 04 · **Estado:** draft

`Pin` expresa que, una vez fijado, un valor no debe moverse mediante una ruta
que permita relocalizarlo. Esto importa cuando una estructura guarda referencias
a partes de sí misma: moverla invalidaría esas referencias.

## Problema e Invariantes

Los futures compilados desde `async` pueden guardar estado entre awaits y, en
algunos casos, referencias internas. El contrato relevante es: si un valor no
implementa `Unpin`, el código seguro no obtiene una referencia mutable que lo
pueda mover después de fijarlo.

`Pin` no vuelve inmóvil a toda la memoria ni hace seguro un diseño incorrecto.
Es una restricción sobre las operaciones disponibles. El capítulo usa un modelo
seguro de estado fijado; construir datos auto-referenciales reales queda fuera
de alcance porque requeriría `unsafe` y revisión humana explícita.

## Alternativas y Justificación

Evitar auto-referencias, usar índices en lugar de referencias o separar la
propiedad suelen ser diseños más simples. `Pin` existe para los casos donde el
contrato de una abstracción asíncrona necesita estabilidad de ubicación.
