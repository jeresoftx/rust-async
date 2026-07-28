# Executor Mínimo

> **Curso:** Rust Async · **Capítulo:** 05 · **Código:** `src/single_task_executor.rs` · **Estado:** draft

## Introducción

Un executor ofrece oportunidades de progreso. No hace trabajo de red ni conoce
la semántica de un future: solo sondea y retira lo que termina.

## Fundamentos

El modelo usa una tarea `CountdownFuture`. Cada `step` realiza un sondeo
finito; `Pending` conserva la tarea y `Ready` marca su retiro terminal.

```mermaid
flowchart LR
    Q[Cola con una tarea] --> P[poll]
    P -->|Pending| Q
    P -->|Ready| R[Retirar tarea]
```

## Implementación y Límites

`SingleTaskExecutor` enseña la frontera scheduler/future. No tiene cola real,
wakers, I/O, temporizadores, concurrencia ni garantías de equidad; Tokio se
estudiará después de construir estos contratos.

```rust
use rust_async::educational_future::CountdownFuture;
use rust_async::single_task_executor::SingleTaskExecutor;

let mut executor = SingleTaskExecutor::new(CountdownFuture::new(1));
assert!(!executor.step());
assert!(executor.step());
```

## Ejercicios y Referencias

1. Ejecuta una tarea ya lista.
2. Cuenta los pasos hasta terminar.
3. Diseña una cola FIFO de dos tareas.
4. Discute qué datos requiere un waker real.

La referencia de continuidad es [Future](https://doc.rust-lang.org/std/future/trait.Future.html).
