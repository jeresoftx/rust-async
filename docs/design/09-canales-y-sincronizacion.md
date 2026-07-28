# Diseño: Canales y Sincronización Asíncrona

**Curso:** Rust Async · **Capítulo:** 09 · **Estado:** draft

Un canal transfiere la propiedad de mensajes entre tareas sin obligarlas a
esperar en el mismo instante. El productor y el consumidor conservan ritmos
independientes; la capacidad del canal decide cuántos mensajes puede absorber
la cola antes de ejercer presión hacia el productor.

## Problema

Compartir una colección mutable entre tareas acopla la coordinación a su
bloqueo y a la vida de la estructura compartida. Un canal expresa una frontera
más clara: un emisor entrega mensajes y un receptor decide cuándo procesarlos.
La elección no elimina el costo de coordinación; lo hace visible mediante
capacidad, espera, cierre y pérdida potencial de mensajes.

## Alternativas

- Un canal acotado aplica *backpressure*: cuando la cola se llena, `send`
  espera hasta que exista capacidad o el receptor cierre el canal.
- Un canal no acotado evita esa espera del emisor, pero puede acumular memoria
  sin límite útil; no es la opción predeterminada para un sistema con carga
  desconocida.
- Una estructura compartida con `Mutex` puede ser apropiada para estado
  compartido, pero no comunica por sí sola propiedad, cierre ni orden de
  consumo.
- Un `watch` o una señal de cancelación modelan el último estado, no una cola
  de todos los mensajes; no deben sustituir un canal cuando cada evento importa.

## Invariantes

1. En el modelo acotado, la cola nunca excede su capacidad declarada.
2. Cada mensaje aceptado se entrega como máximo una vez a un receptor.
3. El orden FIFO se garantiza dentro de un único canal y un único receptor;
   varios receptores o prioridades requieren un contrato diferente.
4. Al cerrarse todos los emisores, el receptor puede drenar lo pendiente y
   después observa el cierre de forma explícita.
5. Al cerrar el receptor, un emisor pendiente recibe un error; no se finge que
   el mensaje fue entregado.
6. La sincronización no debe depender de `sleep`: las pruebas usarán señales y
   resultados observables del canal.

## Decisión Educativa

El modelo siguiente usará `tokio::sync::mpsc::channel` con una capacidad
pequeña y mensajes enteros. Esto permite demostrar presión de cola, cierre del
emisor y cierre del receptor con resultados deterministas, sin introducir
`unsafe` ni abstraer el protocolo real detrás de una API inventada.

Un capítulo posterior comparará este límite con el modelo de actores: el canal
es la tubería; el actor agrega propiedad de estado, protocolo de mensajes y
ciclo de vida.
