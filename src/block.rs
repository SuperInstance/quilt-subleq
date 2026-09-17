//! Block — the uniform resolution target.
//!
//! A block is anything a Quilt-Subleq instruction can read or write.
//! Three kinds: Memory, Cell, Port. The scaling function returns a Block.

use std::fmt;

/// A typed pointer into the substrate. Three flavors:
/// - `Memory(addr)` — local integer tape address
/// - `Cell(id)`     — Quilt cell by 14-tuple id
/// - `Port(url, sig, encoding)` — remote quilt's input-port
/// - `Memory(addr)` — local integer tape address
/// - `Cell(id)`     — Quilt cell by 14-tuple id
/// - `Port(url, sig, encoding)` — remote quilt's input-port
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BlockRef {
    /// A local memory cell. `addr` is the integer tape index.
    Memory(usize),
    /// A Quilt cell, addressed by its 14-tuple id (or hash thereof).
    Cell(String),
    /// A remote quilt's input-port.
    /// `url` is the network address; `sig` is the cell id or signature;
    /// `encoding` is how the port's bytes are framed (JSON, bincode, msgpack, raw).
    Port { url: String, sig: String, encoding: Encoding },
}

impl fmt::Display for BlockRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockRef::Memory(a) => write!(f, "mem://{}", a),
            BlockRef::Cell(id) => write!(f, "cell://{}", id),
            BlockRef::Port { url, sig, encoding } =>
                write!(f, "port://{}?sig={}&enc={}", url, sig, encoding),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Encoding {
    Json,
    Bincode,
    Msgpack,
    Raw,
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Encoding::Json => "json",
            Encoding::Bincode => "bincode",
            Encoding::Msgpack => "msgpack",
            Encoding::Raw => "raw",
        };
        f.write_str(s)
    }
}

/// The block trait — a uniform interface for any read/write target.
///
/// Read returns a value (an i64 for now; can generalize later).
/// Write stores a value. The block knows how to resolve its own address.
pub trait Block {
    fn read(&self) -> Result<i64, String>;
    fn write(&mut self, v: i64) -> Result<(), String>;
    fn r#ref(&self) -> &BlockRef;
}

/// A memory block — backed by a slot in the local integer tape.
pub struct MemoryBlock<'a> {
    pub r#ref: BlockRef,
    pub tape: &'a mut Vec<i64>,
    pub addr: usize,
}

impl<'a> Block for MemoryBlock<'a> {
    fn read(&self) -> Result<i64, String> {
        self.tape.get(self.addr).copied().ok_or_else(|| format!("mem out of bounds: {}", self.addr))
    }
    fn write(&mut self, v: i64) -> Result<(), String> {
        if self.addr >= self.tape.len() {
            // Grow on write
            self.tape.resize(self.addr + 1, 0);
        }
        self.tape[self.addr] = v;
        Ok(())
    }
    fn r#ref(&self) -> &BlockRef { &self.r#ref }
}

/// A cell block — backed by a Quilt cell's value slot.
/// The full cell has 14 tuple fields; we expose just the integer projection here.
pub struct CellBlock<'a> {
    pub r#ref: BlockRef,
    pub store: &'a mut std::collections::HashMap<String, i64>,
}

impl<'a> Block for CellBlock<'a> {
    fn read(&self) -> Result<i64, String> {
        if let Some(id) = self.r#ref.as_cell_id() {
            Ok(self.store.get(&id).copied().unwrap_or(0))
        } else {
            Err("not a cell ref".into())
        }
    }
    fn write(&mut self, v: i64) -> Result<(), String> {
        if let Some(id) = self.r#ref.as_cell_id() {
            self.store.insert(id, v);
            Ok(())
        } else {
            Err("not a cell ref".into())
        }
    }
    fn r#ref(&self) -> &BlockRef { &self.r#ref }
}

/// A port block — backed by a remote quilt's input-port.
/// For now we model it as a local buffer (the remote fetch is mocked).
pub struct PortBlock {
    pub r#ref: BlockRef,
    pub buffer: std::cell::Cell<i64>,
}

impl Block for PortBlock {
    fn read(&self) -> Result<i64, String> { Ok(self.buffer.get()) }
    fn write(&mut self, v: i64) -> Result<(), String> { self.buffer.set(v); Ok(()) }
    fn r#ref(&self) -> &BlockRef { &self.r#ref }
}

impl BlockRef {
    pub fn as_cell_id(&self) -> Option<String> {
        match self {
            BlockRef::Cell(id) => Some(id.clone()),
            _ => None,
        }
    }
}
