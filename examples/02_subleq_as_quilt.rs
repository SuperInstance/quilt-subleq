//! Example 02 — Subleq as Quilt.
//!
//! Subleq's three operands are no longer memory addresses. They are typed
//! pointers (BlockRef). The scaling function resolves them to memory cells,
//! Quilt cells, or remote ports — uniformly.

use quilt_subleq::block::{BlockRef, Encoding};
use quilt_subleq::scaling::{ScalingContext, subleq};

fn main() {
    println!("=== 02 — Subleq as Quilt ===");
    println!("The Subleq instruction now operates on typed pointers.");
    println!("Memory cells, Quilt cells, and remote ports all look the same to it.");
    println!();

    let mut tape = vec![0i64; 32];
    let mut cells = std::collections::HashMap::new();
    cells.insert("quilt://alpha.cell.value".to_string(), 50);
    cells.insert("quilt://alpha.cell.delta".to_string(), 10);
    let mut ports = std::collections::HashMap::new();
    ports.insert("quilt://beta.input".to_string(), 100);
    ports.insert("quilt://beta.output".to_string(), 0);

    // Construct the operands:
    //   A = Cell("quilt://alpha.cell.value")  -- currently 50
    //   B = Cell("quilt://alpha.cell.delta")  -- currently 10
    //   C = Port("quilt://beta.output")       -- branch target if B <= 0
    //
    // The Subleq semantic: B = B - A; if B <= 0, jump to C.
    //   10 - 50 = -40 <= 0  → jump to port.
    //   Port's value becomes 1 (signal "branch taken").
    let a = BlockRef::Cell("quilt://alpha.cell.value".into());
    let b = BlockRef::Cell("quilt://alpha.cell.delta".into());
    // The port isn't the branch target — it's the signal cell.
    // We use a Memory location (pc) as the branch target, and the port stores the signal.
    let c = BlockRef::Memory(20);

    {
        let mut ctx = ScalingContext::new(&mut tape, &mut cells, &mut ports);
        let branch_taken = subleq(&a, &b, &c, &mut ctx).unwrap();
        println!("Subleq(Cell, Cell, Memory) result:");
        println!("  B - A = 10 - 50 = -40");
        println!("  Branch taken: {}", branch_taken != usize::MAX - 1);
        println!("  Branch target: mem://20");
    }

    // Same instruction, different operand kinds:
    //   A = Memory(10) -- 5
    //   B = Memory(11) -- 3
    //   C = Memory(0)  -- pc 0 (loop back)
    //
    //   3 - 5 = -2 <= 0 → branch to pc 0
    tape[10] = 5;
    tape[11] = 3;
    let a = BlockRef::Memory(10);
    let b = BlockRef::Memory(11);
    let c = BlockRef::Memory(0);
    let mut cells = std::collections::HashMap::new();
    let mut ports = std::collections::HashMap::new();
    {
        let mut ctx = ScalingContext::new(&mut tape, &mut cells, &mut ports);
        let branch_taken = subleq(&a, &b, &c, &mut ctx).unwrap();
        println!();
        println!("Subleq(Memory, Memory, Memory) result:");
        println!("  B - A = 3 - 5 = -2 <= 0");
        println!("  Branch taken: {}", branch_taken != usize::MAX - 1);
        println!("  Branch target: mem://0");
    }

    println!();
    println!("Three operand flavors. One instruction. Same shape.");
    println!("The scaling function unifies them.");
}
