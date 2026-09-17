//! A Quilt cell that runs Subleq internally.
//!
//! The cell is the 14-tuple. Its executable substrate is a Subleq machine.
//! Each opcode is a Subleq program.

use crate::subleq::Machine;
use std::collections::HashMap;

/// A Quilt cell, projected down to its Subleq substrate.
pub struct SubleqCell {
    /// The cell's id (or signature).
    pub id: String,
    /// The cell's executable: a Subleq program.
    pub program: Vec<i64>,
    /// Local tape the cell reads/writes.
    pub tape: Vec<i64>,
    /// Inbound links (other cells feeding into this one).
    pub inbound: Vec<String>,
    /// Outbound links (cells this one feeds).
    pub outbound: Vec<String>,
}

impl SubleqCell {
    pub fn new(id: impl Into<String>, program: Vec<i64>, tape_size: usize) -> Self {
        Self {
            id: id.into(),
            program,
            tape: vec![0; tape_size],
            inbound: vec![],
            outbound: vec![],
        }
    }

    /// Execute the cell's Subleq program. Returns ticks used.
    pub fn run(&mut self) -> Result<u64, String> {
        self.run_with_max_ticks(1_000_000)
    }
    
    /// Run with explicit max_ticks to prevent infinite loops.
    pub fn run_with_max_ticks(&mut self, max_ticks: u64) -> Result<u64, String> {
        // The tape IS the memory. Program lives at offset 0..prog_len.
        // User pre-writes (e.g., cell.write(42, 42)) land in the data area.
        let mut tape = std::mem::take(&mut self.tape);
        let prog_len = self.program.len();
        if tape.len() < prog_len {
            tape.resize(prog_len, 0);
        }
        // Place program at offset 0
        for (i, &v) in self.program.iter().enumerate() {
            tape[i] = v;
        }
        let mut m = Machine::new(tape);
        let mut ticks = 0;
        while !m.halted {
            m.step()?;
            ticks += 1;
            if ticks >= max_ticks {
                self.tape = m.mem;
                return Err(format!("subleq exceeded max_ticks={} (likely infinite loop)", max_ticks));
            }
        }
        self.tape = m.mem;
        Ok(ticks)
    }

    /// Read a tape cell.
    pub fn read(&self, addr: usize) -> i64 {
        self.tape.get(addr).copied().unwrap_or(0)
    }

    /// Write a tape cell.
    pub fn write(&mut self, addr: usize, v: i64) {
        if addr >= self.tape.len() {
            self.tape.resize(addr + 1, 0);
        }
        self.tape[addr] = v;
    }
}

/// A small sheet of SubleqCells, wired together by id.
pub struct Sheet {
    pub cells: HashMap<String, SubleqCell>,
    pub pc_order: Vec<String>,
}

impl Sheet {
    pub fn new() -> Self {
        Self { cells: HashMap::new(), pc_order: vec![] }
    }

    pub fn add(&mut self, cell: SubleqCell) {
        self.pc_order.push(cell.id.clone());
        self.cells.insert(cell.id.clone(), cell);
    }

    /// Run every cell in pc order. Cells are independent; links are not
    /// auto-resolved here (the caller is responsible for setting up
    /// initial tape values from upstream cells).
    pub fn run_all(&mut self) -> Result<HashMap<String, u64>, String> {
        let mut ticks = HashMap::new();
        let order = self.pc_order.clone();
        for id in order {
            if let Some(cell) = self.cells.get_mut(&id) {
                let t = cell.run()?;
                ticks.insert(id, t);
            }
        }
        Ok(ticks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quilt_as_subleq::*;

    #[test]
    fn test_subleq_cell_run() {
        // BIND requires a data cell. The cell's tape must be large enough
        // to hold both the program and the data cell.
        // Program is 24 cells, plus the value reference (mem[42] = 42 means literal 42 at addr 42).
        let mut cell = SubleqCell::new("a", bind_program(50, 42), 100);
        // BIND requires pre-placement: mem[VALUE_LOC] = -value
        cell.write(VALUE_LOC, -42);
        let _ = cell.run().unwrap();
        assert_eq!(cell.read(50), 42, "mem[50] should be 42 after BIND");
    }
}
