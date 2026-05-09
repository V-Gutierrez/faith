//! `faith` — agent-first Bible CLI library.
//!
//! Public modules constitute the v0.1 API. The contract is `faith.v1`
//! (see `docs/SPEC.md` and `docs/SCHEMA.md`).

pub mod books;
pub mod citation;
pub mod cli;
pub mod error;
pub mod installer;
pub mod reference;
pub mod schema;
pub mod store;
pub mod translations;

pub use schema::SCHEMA_VERSION;
