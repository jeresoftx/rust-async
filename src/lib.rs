//! Modelos educativos para el curso de programación asíncrona en Rust.
//!
//! El material avanza desde el protocolo [`Future`] y la cooperación mediante
//! [`Poll`] hacia un executor propio y, después, runtimes de producción.
//! Ningún módulo se considera contenido publicado sin revisión humana.

#![forbid(unsafe_code)]

pub mod cooperative;

pub use std::future::Future;
pub use std::task::Poll;

/// Devuelve la identidad del crate para comprobar la fundación de forma
/// explícita, antes de añadir modelos de los capítulos.
#[must_use]
pub const fn course_name() -> &'static str {
    "Rust Async"
}

#[cfg(test)]
mod tests {
    use super::course_name;

    #[test]
    fn exposes_the_course_identity() {
        assert_eq!(course_name(), "Rust Async");
    }
}
