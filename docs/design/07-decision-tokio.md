# Decisión de Integración: Tokio

**Estado:** autorizado por Joel el 28 de julio de 2026

## Contexto

Los capítulos anteriores construyen el protocolo y un runtime educativo mínimo.
Un curso de asincronía también debe mostrar cómo esas piezas se usan en un
runtime de producción sin presentar una biblioteca como magia.

## Alternativas

- Mantener solo el runtime propio: enseña fundamentos, pero no prepara para el
  ecosistema Rust real.
- Implementar un runtime de producción completo: amplía el alcance sin aportar
  proporcionalmente al objetivo del curso.
- Integrar Tokio después del modelo propio: conecta los contratos aprendidos
  con una herramienta de producción auditada y ampliamente usada.

## Decisión

Se autoriza Tokio como dependencia externa no trivial, con características
explícitas y mínimas por ejemplo. Se conservará el crate educativo sin
reemplazar los capítulos previos, no se usará `unsafe` y cada ejemplo declarará
qué parte resuelve Tokio: scheduling, temporizadores, canales o sincronización.

## Límites

Tokio no convierte una task CPU-bound en cooperativa ni garantiza que un diseño
sea correcto. Los ejemplos deben comparar el runtime educativo con Tokio, no
duplicar su implementación ni afirmar equivalencia interna.
