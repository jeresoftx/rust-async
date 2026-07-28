# ROADMAP

`rust-async` es el curso complementario de programación asíncrona de Jeresoft
Academy. No tiene fechas límite: registra dirección y calidad, no una carrera
por terminar (RFC-0001 §1).

## Estado Actual

La fundación está completa: identidad, crate mínimo, controles de calidad,
plan versionado, milestones, issues y Project ya existen. El siguiente paso
natural es el issue #5, que especifica la transición de espera bloqueante a
trabajo cooperativo antes de implementar su modelo educativo.

El checklist detallado vive en
[`docs/superpowers/plans/2026-07-28-rust-async-course.md`](docs/superpowers/plans/2026-07-28-rust-async-course.md).

El [GitHub Project](https://github.com/users/jeresoftx/projects/18) contiene
los issues accionables y debe mantener su vista principal agrupada por
`Milestone`.

## Capítulos Planeados

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

## Alineación RFC-0001

- La estructura sigue RFC-0001 §15.
- Cada capítulo debe cumplir RFC-0001 §14 y la plantilla de §16.
- Los ejercicios siguen los niveles de RFC-0001 §17.
- La IA acelera y el criterio humano decide, conforme a RFC-0001 §20.

## Fuera De Alcance Por Ahora

- Repetir primitivas de hilos que pertenecen a `rust-concurrency`.
- Implementar runtimes de producción alternativos a Tokio.
- `unsafe`, FFI, runtime multihilo de bajo nivel o dependencias no triviales.
- Marcar cualquier capítulo como revisado o publicado.
