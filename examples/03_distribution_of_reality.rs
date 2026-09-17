//! Example 03 — Distribution of Reality.
//!
//! Subleq on a memory tape is local. Subleq with a scaling function can call
//! across the network. Quilt stops being a list of cells in a sheet. Quilt
//! becomes a continuous fabric where memory, cells, and remote ports are
//! all the same kind of thing.
//!
//! This example shows three Quilt nodes talking through a scaling function
//! without knowing what kind of block they're addressing.

use quilt_subleq::block::{BlockRef, Encoding};
use quilt_subleq::scaling::ScalingContext;
use std::collections::HashMap;

struct Node {
    name: String,
    tape: Vec<i64>,
    cells: HashMap<String, i64>,
    ports: HashMap<String, i64>,
}

impl Node {
    fn new(name: &str, tape_size: usize) -> Self {
        Self {
            name: name.into(),
            tape: vec![0i64; tape_size],
            cells: HashMap::new(),
            ports: HashMap::new(),
        }
    }

    /// A scaling call: an instruction that resolves to memory / cell / port.
    fn scaling_subleq(
        &mut self,
        a: &BlockRef,
        b: &BlockRef,
        c: &BlockRef,
    ) -> Result<i64, String> {
        let ctx = &mut ScalingContext::new(
            &mut self.tape,
            &mut self.cells,
            &mut self.ports,
        );
        // Resolve a first; release before resolving b.
        let va = ctx.resolve(a).map_err(|e| format!("a: {}", e))?.read()?;
        let new_b = {
            let mut blk_b = ctx.resolve(b).map_err(|e| format!("b: {}", e))?;
            let vb = blk_b.read()?;
            let r = vb.wrapping_sub(va);
            blk_b.write(r)?;
            r
        };

        if new_b <= 0 {
            // Branch: write 1 to the target c (we treat c as a status cell)
            ctx.resolve(c).map_err(|e| format!("c: {}", e))?.write(1)?;
            Ok(1)
        } else {
            Ok(0)
        }
    }
}

fn main() {
    println!("=== 03 — Distribution of Reality ===");
    println!("Three Quilt nodes. Memory / cells / ports. Same instruction.");
    println!();

    // Node A: holds a memory cell with value 50
    let mut node_a = Node::new("A", 16);
    node_a.tape[5] = 50;

    // Node B: holds a Quilt cell "delta" with value 10
    let mut node_b = Node::new("B", 0);
    node_b.cells.insert("delta".into(), 10);

    // Node C: holds a port that will receive the "branch taken" signal
    let mut node_c = Node::new("C", 0);
    node_c.ports.insert("branch_signal".into(), 0);

    // Cross-node call: Subleq( A.mem[5], B.cell.delta, C.port.branch_signal )
    //
    // In a real impl, the scaling function would resolve across the network.
    // For this demo, we model it as the three nodes being available locally
    // through a shared scaling context.
    //
    // Simulate by setting up a shared context that can see all three nodes.
    let mut tape = node_a.tape.clone();
    let mut cells = node_b.cells.clone();
    let mut ports = node_c.ports.clone();

    let a = BlockRef::Memory(5);            // 50 (in A's tape)
    let b = BlockRef::Cell("delta".into()); // 10 (in B's cells)
    let c = BlockRef::Port {
        url: "node-c".into(),
        sig: "branch_signal".into(),
        encoding: Encoding::Raw,
    };                                      // branch target (in C's ports)

    let mut ctx = ScalingContext::new(&mut tape, &mut cells, &mut ports);
    let va = ctx.resolve(&a).unwrap().read().unwrap();
    let (new_b, branch_signal) = {
        let mut blk_b = ctx.resolve(&b).unwrap();
        let vb = blk_b.read().unwrap();
        let r = vb.wrapping_sub(va);
        blk_b.write(r).unwrap();
        (r, r <= 0)
    };
    println!("Instruction: Subleq(mem[5]=50, cell.delta=10, port.branch_signal)");
    println!("  B - A = {} - {} = {}", 10, 50, new_b);
    println!("  Result: {} (branch taken = {})", new_b, branch_signal);
    if branch_signal {
        ctx.resolve(&c).unwrap().write(1).unwrap();
        println!("  Branch signal written to {} (now = {})", c, ports.get("branch_signal").unwrap());
    }

    println!();
    println!("The instruction didn't know whether A was memory,");
    println!("a Quilt cell, or a port. The scaling function resolved it.");
    println!("The substrate is uniform: any block, however encoded or reached.");
}
