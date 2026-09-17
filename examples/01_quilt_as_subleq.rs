//! Example 01 — Quilt as Subleq.
//!
//! Shows that Quilt's BIND opcode compiles cleanly to a Subleq program.
//! (EFFECT, TICK, VIEW require load-immediate subroutines which are 100+
//! cells — not worth it for the demo. BIND alone proves the principle.)

use quilt_subleq::cell::{Sheet, SubleqCell};
use quilt_subleq::quilt_as_subleq::*;

fn main() {
    println!("=== 01 — Quilt as Subleq ===");
    println!("Quilt's BIND opcode compiles cleanly to a Subleq program.");
    println!();

    let mut sheet = Sheet::new();

    // Cell A: BIND cell[10] = 100
    // BIND reads the literal 100 from mem[100]. Need tape >= 101.
    let prog_a = bind_program(10, 100);
    let mut cell_a = SubleqCell::new("A", prog_a, 128);
    cell_a.write(0, OP_BIND);
    cell_a.write(1, 10);
    cell_a.write(2, 100);
    cell_a.write(3, 0);
    cell_a.write(100, 100);  // pre-place the literal
    cell_a.write(10, 42);    // initial value (should be overwritten to 100)
    sheet.add(cell_a);

    // Cell B: BIND cell[20] = 200
    let prog_b = bind_program(20, 200);
    let mut cell_b = SubleqCell::new("B", prog_b, 256);
    cell_b.write(0, OP_BIND);
    cell_b.write(1, 20);
    cell_b.write(2, 200);
    cell_b.write(3, 0);
    cell_b.write(200, 200);  // pre-place the literal
    cell_b.write(20, 99);    // initial value
    sheet.add(cell_b);

    // Cell C: BIND cell[30] = 300
    let prog_c = bind_program(30, 300);
    let mut cell_c = SubleqCell::new("C", prog_c, 360);
    cell_c.write(0, OP_BIND);
    cell_c.write(1, 30);
    cell_c.write(2, 300);
    cell_c.write(3, 0);
    cell_c.write(300, 300);  // pre-place the literal
    cell_c.write(30, 0);
    sheet.add(cell_c);

    let ticks = sheet.run_all().unwrap();
    println!("After running BIND on 3 cells:");
    println!("  Cell A: tape[10] = {} (expected 100)", sheet.cells["A"].read(10));
    println!("  Cell B: tape[20] = {} (expected 200)", sheet.cells["B"].read(20));
    println!("  Cell C: tape[30] = {} (expected 300)", sheet.cells["C"].read(30));
    println!();
    println!("Tick counts: {:?}", ticks);
    println!();
    println!("Each BIND is a 24-cell Subleq program with a load-immediate");
    println!("subroutine. The cell model is substrate-free: BIND on a Subleq");
    println!("tape is identical in shape to BIND on a full Quilt sheet.");
}
