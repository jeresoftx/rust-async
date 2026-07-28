# Rust Async

Repositorio complementario de Jeresoft Academy para estudiar programación
asíncrona en Rust. Parte desde el protocolo que permite pausar y reanudar
trabajo (`Future`, `Poll`, `Waker` y `Pin`), construye un executor mínimo y
después contrasta ese modelo con las herramientas de producción de Tokio.

El objetivo no es memorizar una API: es entender qué problema resuelve cada
pieza, qué invariante conserva y cuándo conviene elegir asincronía frente a
hilos, paralelismo de datos o código secuencial (RFC-0001 §2 y §10).

## Lugar En El Camino

Este curso complementario aprovecha fundamentos de Rust, concurrencia, redes y
sistemas operativos. Alimenta sistemas distribuidos, cloud, mensajería, video,
travel tech y cualquier servicio que necesite coordinar muchas operaciones de
E/S sin asignar un hilo a cada espera.

La concurrencia con hilos es canónica en `rust-concurrency`; este repositorio
estudia concurrencia cooperativa y runtimes asíncronos. Tokio se presenta
después de implementar un executor pequeño y explícito, como establece
RFC-0001 §10.

## Capítulos

| # | Capítulo | Estado |
|---|----------|--------|
| 01 | De espera bloqueante a trabajo cooperativo | planned |
| 02 | Future y Poll | planned |
| 03 | Waker y Context | planned |
| 04 | Pin y datos auto-referenciales | planned |
| 05 | Executor mínimo | planned |
| 06 | Tasks y concurrencia cooperativa | planned |
| 07 | Tokio y runtime de producción | planned |
| 08 | select!, cancelación y timeouts | planned |
| 09 | Canales y sincronización asíncrona | planned |
| 10 | Modelo de actores | planned |

Estados posibles: `planned`, `draft`, `implemented`, `tested`, `benchmarked`,
`reviewed`, `published`.

## Estructura

```text
docs/       Capítulos compatibles con mdBook.
src/        Modelos e implementaciones educativas en Rust.
examples/   Ejemplos progresivos y soluciones de ejercicios.
tests/      Pruebas de integración.
benches/    Mediciones que justifican afirmaciones de rendimiento.
diagrams/   Diagramas Mermaid.
```

## Verificación

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
```

## Gobernanza

- El checklist de implementación vive en
  [`docs/superpowers/plans/2026-07-28-rust-async-course.md`](docs/superpowers/plans/2026-07-28-rust-async-course.md).
- El avance se coordina en el [GitHub Project](https://github.com/users/jeresoftx/projects/17).
- El código usa `MIT OR Apache-2.0`; el contenido educativo usa `CC BY-SA 4.0`.
- Ningún capítulo se marca como `reviewed` o `published` sin revisión humana.

## Filosofía

Este repositorio debe poder leerse como un libro de ingeniería: claridad antes
que ingenio, calidad antes que velocidad y el porqué antes del cómo.
Programación asíncrona en Rust: del protocolo Future a executors y Tokio.
