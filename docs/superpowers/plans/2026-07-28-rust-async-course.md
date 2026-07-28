# Plan de implementación de Rust Async

**Objetivo:** construir `rust-async` como curso complementario de Jeresoft
Academy: un libro de ingeniería y crate educativo que explique la asincronía en
Rust desde sus protocolos hasta Tokio y actores.

**Arquitectura:** un crate Rust sin dependencias durante los capítulos
fundacionales; un capítulo por concepto, con implementación o modelo educativo,
documentación compatible con mdBook, ejemplos, pruebas, ejercicios, diagramas
Mermaid y benchmarks cuando puedan sostener una afirmación útil.

**Decisiones de origen:** RFC-0001 §1, §2, §10, §12, §13, §14, §15, §16, §17 y
§20. Tokio se estudia después de un executor propio; `unsafe` queda fuera de
alcance hasta que exista una decisión humana explícita.

## Fundación

- [x] Establecer identidad, estructura, licencias, crate mínimo y CI.
- [x] Crear el [GitHub Project #18](https://github.com/users/jeresoftx/projects/18), milestones e issues antes de tocar capítulos.
- [x] Confirmar que la vista principal del Project esté agrupada por `Milestone`.

## Regla De Producción Por Capítulo

Para cada capítulo, antes de pasar al siguiente:

- [ ] Explicar concepto, problema, alternativas y justificación.
- [ ] Completar las secciones obligatorias de RFC-0001 §14 y metadatos de §16.
- [ ] Incluir diagrama Mermaid y referencias de calidad.
- [ ] Implementar un modelo Rust claro, con documentación pública y tests.
- [ ] Añadir ejemplos progresivos, ejercicios de niveles 1 a 4 y soluciones 1 a 3.
- [ ] Añadir benchmark o declarar con honestidad por qué no aplica.
- [ ] Ejecutar formato, Clippy, tests, doctests y la verificación de diffs.
- [ ] Actualizar README, ROADMAP y este checklist sin marcar `reviewed` o
  `published`.

## Secuencia Del Curso

### Milestone 1: Fundamentos del protocolo asíncrono

- [ ] Capítulo 01: de espera bloqueante a trabajo cooperativo.
  - [x] #5 Especificar concepto, problema, invariantes y alternativas.
  - [x] #6 Implementar y probar un modelo mínimo de progreso cooperativo.
  - [x] #7 Escribir capítulo, diagrama, ejemplos y ejercicios.
- [ ] Capítulo 02: `Future` y `Poll`.
  - [x] #8 Especificar contrato, estados y alternativas.
  - [ ] #9 Implementar y probar futuros educativos mínimos.
  - [ ] #10 Escribir capítulo, diagrama, ejemplos y ejercicios.
- [ ] Capítulo 03: `Waker` y `Context`.
- [ ] Capítulo 04: `Pin` y datos auto-referenciales.

### Milestone 2: Runtime educativo

- [ ] Capítulo 05: executor mínimo de una tarea.
- [ ] Capítulo 06: tasks, cola de ejecución y concurrencia cooperativa.

### Milestone 3: Runtime de producción y coordinación

- [ ] Capítulo 07: Tokio y un runtime de producción.
- [ ] Capítulo 08: `select!`, cancelación y timeouts.
- [ ] Capítulo 09: canales y sincronización asíncrona.

### Milestone 4: Composición avanzada

- [ ] Capítulo 10: modelo de actores.
- [ ] Completar ruta de lectura, glosario, referencias cruzadas y verificación
  final de coherencia del curso.

## Límites

- No agregar dependencias externas no triviales sin aprobación humana.
- No usar `unsafe`.
- No reimplementar el canon de concurrencia con hilos de `rust-concurrency`.
- No publicar ni marcar material como revisado sin revisión humana.
