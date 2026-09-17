//! End-to-end tests for the Quilt-Subleq substrate.

use super::*;
use quilt_as_subleq::*;

/// Helper: pre-place values for canonical programs.
fn pre_place(addr: usize, value: i64, tape: &mut Vec<i64>) {
    while tape.len() <= addr { tape.push(0); }
    tape[addr] = value;
}

fn setup_preplaced() -> Vec<i64> {
    let mut tape = vec![0i64; 100];
    tape[ZERO_LOC] = 0;
    tape[VALUE_LOC] = 0;
    tape[NEG_ONE_LOC] = -1;
    tape
}

// === Subleq core ===

#[test]
fn subleq_runs_until_halt() {
    use subleq::Machine;
    let m = Machine::new(vec![-1, 0, 0]);
    let mut m = m;
    let _ = m.run().unwrap();
    assert!(m.halted);
}

#[test]
fn subleq_halt_via_negative_a() {
    use subleq::Machine;
    let mut m = Machine::new(vec![-1, 0, 0]);
    m.step().unwrap();
    assert!(m.halted);
}

// === Scaling function ===

#[test]
fn scaling_function_resolves_memory() {
    use block::BlockRef;
    use scaling::ScalingContext;

    let mut tape = vec![0i64; 10];
    let mut cells = std::collections::HashMap::new();
    let mut ports = std::collections::HashMap::new();
    {
        let mut ctx = ScalingContext::new(&mut tape, &mut cells, &mut ports);
        let r = BlockRef::Memory(5);
        let mut b = ctx.resolve(&r).unwrap();
        b.write(99).unwrap();
    }
    assert_eq!(tape[5], 99);
}

#[test]
fn scaling_function_resolves_cell() {
    use block::BlockRef;
    use scaling::ScalingContext;

    let mut tape = vec![];
    let mut cells = std::collections::HashMap::new();
    cells.insert("cell-a".into(), 42);
    let mut ports = std::collections::HashMap::new();
    let mut ctx = ScalingContext::new(&mut tape, &mut cells, &mut ports);

    let r = BlockRef::Cell("cell-a".into());
    let b = ctx.resolve(&r).unwrap();
    assert_eq!(b.read().unwrap(), 42);
}

#[test]
fn scaling_function_resolves_port() {
    use block::BlockRef;
    use scaling::ScalingContext;

    let mut tape = vec![];
    let mut cells = std::collections::HashMap::new();
    let mut ports = std::collections::HashMap::new();
    ports.insert("remote::node-7".into(), 100);
    let mut ctx = ScalingContext::new(&mut tape, &mut cells, &mut ports);

    let r = BlockRef::Port {
        url: "remote".into(),
        sig: "node-7".into(),
        encoding: block::Encoding::Raw,
    };
    let mut b = ctx.resolve(&r).unwrap();
    assert_eq!(b.read().unwrap(), 100);
}

// === BIND — canonical 1-instruction program ===

#[test]
fn bind_program_actually_writes_value() {
    let mut tape = setup_preplaced();
    pre_place(VALUE_LOC, -42, &mut tape);  // -value

    let prog = bind_program(10, 42);
    let mut cell = cell::SubleqCell::new("bind-test", prog, tape.len());
    cell.tape = tape;
    let _ = cell.run().unwrap();

    assert_eq!(cell.tape[10], 42, "BIND should write 42 to mem[10]");
    assert_eq!(cell.tape[91], -42, "VALUE_LOC unchanged");
}

#[test]
fn bind_program_writes_zero_when_value_is_zero() {
    let mut tape = setup_preplaced();
    pre_place(VALUE_LOC, 0, &mut tape);

    let prog = bind_program(10, 0);
    let mut cell = cell::SubleqCell::new("bind-zero", prog, tape.len());
    cell.tape = tape;
    let _ = cell.run().unwrap();

    assert_eq!(cell.tape[10], 0);
}

#[test]
fn bind_program_handles_negative_values() {
    let mut tape = setup_preplaced();
    pre_place(VALUE_LOC, 7, &mut tape);  // -(-7) = 7

    let prog = bind_program(10, -7);
    let mut cell = cell::SubleqCell::new("bind-neg", prog, tape.len());
    cell.tape = tape;
    let _ = cell.run().unwrap();

    assert_eq!(cell.tape[10], -7);
}

// === LINK — same encoding, different semantics ===

#[test]
fn link_program_actually_writes_target() {
    let mut tape = setup_preplaced();
    pre_place(VALUE_LOC, -999, &mut tape);

    let prog = link_program(50, 999);
    let mut cell = cell::SubleqCell::new("link-test", prog, tape.len());
    cell.tape = tape;
    let _ = cell.run().unwrap();

    assert_eq!(cell.tape[50], 999, "LINK should write 999 to mem[50]");
}

#[test]
fn link_program_writes_three_targets_in_sequence() {
    // Simulate a cell's outbound links array
    let mut tape = setup_preplaced();
    
    // LINK 10 → 100
    tape[VALUE_LOC] = -100;
    let p1 = link_program(10, 100);
    let mut cell = cell::SubleqCell::new("link-cell", p1, tape.len());
    cell.tape = tape.clone();
    let _ = cell.run().unwrap();
    assert_eq!(cell.tape[10], 100);
    
    // LINK 11 → 200
    cell.tape[VALUE_LOC] = -200;
    let p2 = link_program(11, 200);
    cell.program = p2;
    let _ = cell.run().unwrap();
    assert_eq!(cell.tape[11], 200);
    
    // LINK 12 → 300
    cell.tape[VALUE_LOC] = -300;
    let p3 = link_program(12, 300);
    cell.program = p3;
    let _ = cell.run().unwrap();
    assert_eq!(cell.tape[12], 300);
}

// === EFFECT — increment ===

#[test]
fn effect_program_increments() {
    let mut tape = setup_preplaced();
    tape[NEG_ONE_LOC] = -1;

    let prog = effect_program(20);
    let mut cell = cell::SubleqCell::new("effect-test", prog, tape.len());
    cell.tape = tape;
    let _ = cell.run().unwrap();

    assert_eq!(cell.tape[20], 1, "EFFECT should increment mem[20] from 0 to 1");
}

// === FORGET — clear ===

#[test]
fn forget_program_clears() {
    let mut tape = setup_preplaced();
    tape[20] = 999;  // pre-existing value

    let prog = forget_program(20);
    let mut cell = cell::SubleqCell::new("forget-test", prog, tape.len());
    cell.tape = tape;
    let _ = cell.run().unwrap();

    assert_eq!(cell.tape[20], 0, "FORGET should clear mem[20]");
}

// === Cell / SubleqCell ===

#[test]
fn cell_runs_subleq_program() {
    let mut tape = setup_preplaced();
    tape[VALUE_LOC] = -7;

    let prog = bind_program(10, 7);
    let mut cell = cell::SubleqCell::new("test", prog, tape.len());
    cell.tape = tape;
    let _ = cell.run().unwrap();
    assert!(cell.tape[10] == 7);
}

// === All opcodes compile ===

#[test]
fn all_eleven_opcodes_compile() {
    for op in [OP_BIND, OP_LINK, OP_EFFECT, OP_VIEW, OP_TICK, OP_FORGET, OP_PROOF, OP_ROUTE, OP_CRDT, OP_WORLD, OP_TIME] {
        let p = compile(op, 0, 0);
        assert!(p.len() % 3 == 0, "op {} produced non-aligned program (len={})", op, p.len());
    }
}

#[test]
fn all_opcodes_produce_valid_subleq_programs() {
    // Each compiled program must be halt-able (eventually) and not error out
    let mut tape = setup_preplaced();
    tape[VALUE_LOC] = -42;

    for op in [OP_BIND, OP_LINK, OP_EFFECT, OP_VIEW, OP_TICK, OP_FORGET, OP_PROOF, OP_CRDT, OP_WORLD, OP_TIME] {
        // Skip OP_ROUTE — its 1-instruction program with target=X jumps to pc=X.
        // If X is in the data area (which is 0), the branch loops forever.
        // ROUTE is meaningful only when wired to a real cell graph.
        let p = compile(op, 10, 42);
        let mut cell = cell::SubleqCell::new("op-test", p, tape.len());
        cell.tape = tape.clone();
        let result = cell.run();
        assert!(result.is_ok(), "op {} failed to run: {:?}", op, result);
    }
}

// === Port registry ===

#[test]
fn port_registry_roundtrip() {
    use port::{Port, PortRegistry};

    let mut reg = PortRegistry::new();
    reg.register(Port::new("quilt://alpha", "input", 13));
    assert!(reg.get("quilt://alpha", "input").is_some());
    assert_eq!(reg.get("quilt://alpha", "input").unwrap().buffer.get(), 13);
}

// === Composition: BIND then EFFECT ===

#[test]
fn bind_then_effect_yields_correct_value() {
    let mut tape = setup_preplaced();
    tape[VALUE_LOC] = -5;
    tape[NEG_ONE_LOC] = -1;

    // BIND 10 = 5 (using bind_program)
    let p1 = bind_program(10, 5);
    let mut cell = cell::SubleqCell::new("compose", p1, tape.len());
    cell.tape = tape.clone();
    let _ = cell.run().unwrap();
    assert_eq!(cell.tape[10], 5);

    // EFFECT 10 (increment to 6)
    let p2 = effect_program(10);
    cell.program = p2;
    let _ = cell.run().unwrap();
    assert_eq!(cell.tape[10], 6, "after BIND(5) + EFFECT, mem[10] should be 6");
}
