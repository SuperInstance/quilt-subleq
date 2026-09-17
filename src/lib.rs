//! quilt-subleq — Quilt as Subleq, Subleq as Quilt.
//!
//! The smallest computer that can run a Quilt cell. One instruction
//! (`mem[B] -= mem[A]; branch if <= 0`). Every Quilt opcode compiles
//! to a small Subleq program on the local tape.
//!
//! ## Architecture
//!
//! - [`subleq`] — pure Subleq machine (interpreter)
//! - [`block`] — typed pointers (Memory/Cell/Port)
//! - [`scaling`] — scaling context that resolves any BlockRef to a unified block
//! - [`cell`] — Quilt cell backed by a Subleq program
//! - [`port`] — port registry for cross-quilt calls
//! - [`quilt_as_subleq`] — 11 Quilt opcodes compiled to Subleq
//!
//! ## Canonical encoding
//!
//! The caller pre-places values at known addresses (the "value well"):
//!
//! | Address | Name | Purpose |
//! |---------|------|---------|
//! | 90 | ZERO_LOC | Holds 0 (for clearing) |
//! | 91 | VALUE_LOC | Holds -value (for BIND/LINK) |
//! | 92 | NEG_ONE_LOC | Holds -1 (for EFFECT/increment) |
//!
//! Then BIND/LINK/EFFECT are 1 instruction. VIEW is 2. PROOF/ROUTE/CRDT/
//! WORLD/TIME are stubbed (substrate-level semantics are witness-only).

pub mod block;
pub mod cell;
pub mod port;
pub mod quilt_as_subleq;
pub mod scaling;
pub mod subleq;

#[cfg(test)]
mod tests;
