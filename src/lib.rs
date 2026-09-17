//! quilt-subleq
//!
//! Quilt on Subleq, and Subleq on Quilt.
//!
//! Two directions of one insight:
//! 1. Quilt's 11 opcodes compile to Subleq programs (substrate-free proof).
//! 2. Subleq gets a scaling function that resolves typed pointers into blocks.
//! 3. Blocks are memory, cells, or remote quilts' input-ports — uniform.
//!
//! Casey: "Quilt becomes more than a registry. It's a distribution of reality."

pub mod subleq;
pub mod block;
pub mod scaling;
pub mod cell;
pub mod port;
pub mod quilt_as_subleq;

#[cfg(test)]
mod tests;
