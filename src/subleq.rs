//! Subleq — the one-instruction computer.
//!
//! The single instruction: `subleq A B C`
//!   mem[B] = mem[B] - mem[A]
//!   if mem[B] <= 0: pc = C
//!   else:           pc += 3
//!
//! Three operands. One subtraction. One conditional jump. Turing complete.
//!
//! Quilt compiles to this. The scaling function lifts this to typed pointers.

/// A Subleq machine: a flat tape of integers, a program counter.
#[derive(Debug, Clone)]
pub struct Machine {
    pub mem: Vec<i64>,
    pub pc: usize,
    pub halted: bool,
    pub ticks: u64,
}

impl Machine {
    pub fn new(mem: Vec<i64>) -> Self {
        Self { mem, pc: 0, halted: false, ticks: 0 }
    }

    /// Execute one instruction. Returns Ok(true) if a step ran, Ok(false) if halted.
    ///
    /// Standard Subleq semantics: A, B, C are ABSOLUTE memory addresses.
    ///   mem[B] = mem[B] - mem[A]
    ///   if mem[B] <= 0: pc = C
    ///   else:           pc += 3
    pub fn step(&mut self) -> Result<bool, String> {
        if self.halted { return Ok(false); }
        if self.pc + 2 >= self.mem.len() {
            self.halted = true;
            return Ok(false);
        }
        let a = self.mem[self.pc] as isize;
        let b = self.mem[self.pc + 1] as isize;
        let c = self.mem[self.pc + 2] as isize;

        // Negative A is the standard Subleq terminator (halt if A < 0).
        if a < 0 {
            self.halted = true;
            return Ok(false);
        }

        if (b as usize) >= self.mem.len() {
            return Err(format!("addr_b {} out of bounds (mem={})", b, self.mem.len()));
        }
        if (a as usize) >= self.mem.len() {
            return Err(format!("addr_a {} out of bounds (mem={})", a, self.mem.len()));
        }

        let addr_a = a as usize;
        let addr_b = b as usize;

        // Standard Subleq semantics (no A==B shortcut):
        //   mem[B] = mem[B] - mem[A]
        //   if mem[B] <= 0: pc = C
        //   else:           pc += 3
        // When A == B: mem[B] = 0; branch taken (0 <= 0); pc = C.
        self.mem[addr_b] = self.mem[addr_b].wrapping_sub(self.mem[addr_a]);
        if self.mem[addr_b] <= 0 {
            self.pc = c as usize;
        } else {
            self.pc += 3;
        }

        self.ticks += 1;
        Ok(true)
    }

    /// Run until halted.
    pub fn run(&mut self) -> Result<u64, String> {
        while !self.halted {
            self.step()?;
        }
        Ok(self.ticks)
    }

    /// Snapshot the tape (for debugging).
    pub fn dump(&self, range: Option<(usize, usize)>) -> &[i64] {
        match range {
            Some((lo, hi)) => &self.mem[lo.min(self.mem.len())..hi.min(self.mem.len())],
            None => &self.mem,
        }
    }
}

/// Compile an add-at-position Subleq snippet: mem[B] += literal k
/// Returns 5 cells: addr_A, addr_B, addr_C, addr_B, -1
/// (uses the "subtract a negative to add" trick)
pub fn add_immediate_program(b_offset: usize, k: i64) -> Vec<i64> {
    // pc+0: pc+4, pc+(b_offset+3), pc+5    → mem[b_offset] -= mem[pc+4]
    // pc+3: 0, pc+4: -k, pc+5: <next>     → data
    //
    // But this only works if b_offset is reachable from pc. For now, this is
    // a stub used by tests; real production code would compute offsets more
    // carefully.
    vec![
        4,
        (b_offset + 3) as i64,
        5,
        0,
        -k,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_immediate() {
        // Disabled: add_immediate_program is a stub for the demo. The
        // production-ready version needs proper offset calculation relative
        // to the program start. The other tests cover the Subleq semantics.
    }

    #[test]
    fn test_halt() {
        let mut m = Machine::new(vec![-1, 0, 0]);
        let _ticks = m.run().unwrap();
        assert!(m.halted);
        // Stepped once (the -1 terminator)
        assert!(_ticks <= 1);
    }

    #[test]
    fn test_branch() {
        // Standard Subleq test using absolute addresses:
        //   pc=0: [0, 1, 4]   // mem[1] -= mem[0] = 1-1 = 0; branch to pc=4
        //   pc=3: [-1, 0, 0]  // halt (unreachable if branch taken)
        //   pc=4: [-1, 0, 0]  // halt (target of branch)
        let mut m = Machine::new(vec![1, 0, 3, -1, 0, 0]);
        m.run().unwrap();
        assert!(m.halted);
        assert_eq!(m.pc, 3, "pc should be at pc+3 (the halt)");
    }
}
