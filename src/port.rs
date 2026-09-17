//! A remote quilt's input-port.
//!
//! A port is a typed address into another quilt's substrate.
//! When a scaling function resolves a PortRef, it can read the port's
//! current value or write to it. The "remote" is a fetch (or push) —
//! locally cached as a buffer.

use crate::block::{Block, BlockRef, PortBlock};

/// A port is identified by (url, signature, encoding).
/// The url points to the remote quilt. The signature is the cell id
/// or input-port name. The encoding is how the bytes are framed.
#[derive(Debug, Clone)]
pub struct Port {
    pub url: String,
    pub sig: String,
    pub buffer: std::cell::Cell<i64>,
}

impl Port {
    pub fn new(url: impl Into<String>, sig: impl Into<String>, initial: i64) -> Self {
        Self {
            url: url.into(),
            sig: sig.into(),
            buffer: std::cell::Cell::new(initial),
        }
    }

    pub fn as_block(&self) -> PortBlock {
        PortBlock {
            r#ref: BlockRef::Port {
                url: self.url.clone(),
                sig: self.sig.clone(),
                encoding: crate::block::Encoding::Raw,
            },
            buffer: std::cell::Cell::new(self.buffer.get()),
        }
    }
}

/// A registry of known remote ports.
pub struct PortRegistry {
    pub ports: std::collections::HashMap<String, Port>,
}

impl PortRegistry {
    pub fn new() -> Self { Self { ports: Default::default() } }

    pub fn register(&mut self, port: Port) {
        let key = format!("{}::{}", port.url, port.sig);
        self.ports.insert(key, port);
    }

    pub fn get(&self, url: &str, sig: &str) -> Option<&Port> {
        self.ports.get(&format!("{}::{}", url, sig))
    }
}
