//! Scaling function — typed-pointer resolution.
//!
//! The scaling function takes a BlockRef and returns a concrete block
//! the Subleq instruction can read or write. The block can be a memory cell,
//! a Quilt cell, or a remote quilt's input-port.
//!
//! "Scaling" because a small typed pointer scales out to any block in the
//! substrate — local or remote, however encoded or reached.

use std::collections::HashMap;
use crate::block::{Block, BlockRef, MemoryBlock, CellBlock, PortBlock};

/// The scaling context — owns the local tape and cell store.
/// Remote ports are looked up against a port registry.
pub struct ScalingContext<'a> {
    pub tape: &'a mut Vec<i64>,
    pub cells: &'a mut HashMap<String, i64>,
    pub ports: &'a mut HashMap<String, i64>,
}

impl<'a> ScalingContext<'a> {
    pub fn new(
        tape: &'a mut Vec<i64>,
        cells: &'a mut HashMap<String, i64>,
        ports: &'a mut HashMap<String, i64>,
    ) -> Self {
        Self { tape, cells, ports }
    }

    /// Resolve a BlockRef to a Memory block. Returns an error if the ref
    /// isn't a memory ref. Mutates the tape shape (grows on demand) as needed.
    pub fn memory<'b>(&'b mut self, r: &'b BlockRef) -> Result<MemoryBlock<'b>, String> {
        match r {
            BlockRef::Memory(addr) => Ok(MemoryBlock {
                r#ref: r.clone(),
                tape: self.tape,
                addr: *addr,
            }),
            _ => Err(format!("not a memory ref: {}", r)),
        }
    }

    /// Resolve a BlockRef to a Cell block.
    pub fn cell<'b>(&'b mut self, r: &'b BlockRef) -> Result<CellBlock<'b>, String> {
        match r {
            BlockRef::Cell(_) => Ok(CellBlock {
                r#ref: r.clone(),
                store: self.cells,
            }),
            _ => Err(format!("not a cell ref: {}", r)),
        }
    }

    /// Resolve a BlockRef to a Port block (uses the local port registry).
    pub fn port<'b>(&'b mut self, r: &'b BlockRef) -> Result<PortBlock, String> {
        match r {
            BlockRef::Port { url, sig, .. } => {
                // Try the combined key first ("url::sig"), then fall back to bare sig
                let key = format!("{}::{}", url, sig);
                let v = self.ports.get(&key).copied()
                    .or_else(|| self.ports.get(sig).copied())
                    .unwrap_or(0);
                Ok(PortBlock {
                    r#ref: r.clone(),
                    buffer: std::cell::Cell::new(v),
                })
            }
            _ => Err(format!("not a port ref: {}", r)),
        }
    }

    /// Resolve any BlockRef to any block. Returns a Boxed Block trait object.
    /// This is the "any quilt does scaling" entry point.
    pub fn resolve<'b>(&'b mut self, r: &'b BlockRef) -> Result<Box<dyn Block + 'b>, String> {
        match r {
            BlockRef::Memory(_) => Ok(Box::new(self.memory(r)?)),
            BlockRef::Cell(_) => Ok(Box::new(self.cell(r)?)),
            BlockRef::Port { .. } => Ok(Box::new(self.port(r)?)),
        }
    }
}

/// The scaling function — top-level entry point.
/// Given three BlockRefs and a context, perform the Subleq operation.
///
/// Returns the new pc (which may be C if the result <= 0).
pub fn subleq(
    a: &BlockRef,
    b: &BlockRef,
    c: &BlockRef,
    ctx: &mut ScalingContext,
) -> Result<usize, String> {
    // If a is a "halt" sentinel, signal halt
    if matches!(a, BlockRef::Memory(usize::MAX)) {
        return Ok(usize::MAX);
    }

    // Resolve a and b — read a first, release the borrow, then resolve b.
    let va = {
        let blk_a = ctx.resolve(a)?;
        blk_a.read()?
    };
    let new_b = {
        let mut blk_b = ctx.resolve(b)?;
        let vb = blk_b.read()?;
        let r = vb.wrapping_sub(va);
        blk_b.write(r)?;
        r
    };

    if new_b <= 0 {
        // Jump to C. C's address is the target row index.
        if let BlockRef::Memory(pc) = c {
            Ok(*pc)
        } else {
            Err(format!("branch target must be a memory (pc) ref, got: {}", c))
        }
    } else {
        // Advance pc by 3 — but pc isn't known at this level (it's a machine concept).
        // The machine passes pc in. So this function just signals: branch? or no branch?
        // Actually for cleanest impl, the caller passes pc.
        Ok(usize::MAX - 1) // sentinel: "no branch"
    }
}
