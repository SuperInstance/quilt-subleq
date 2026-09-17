//! End-to-end tests for the Quilt-Subleq substrate.

use super::*;

#[test]
fn subleq_runs_until_halt() {
    use subleq::Machine;
    // Program: halt immediately
    let m = Machine::new(vec![-1, 0, 0]);
    let mut m = m;
    let ticks = m.run().unwrap();
    assert!(m.halted);
}

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
    b.write(200).unwrap();
    // Note: write to local PortBlock buffer doesn't update the registry;
    // in a real impl, the write would push back to the remote.
}

#[test]
fn cell_runs_subleq_program() {
    use cell::SubleqCell;
    use quilt_as_subleq::{bind_program, OP_BIND};

    let prog = bind_program(10, 7);
    let mut cell = SubleqCell::new("test", prog, 20);
    let ticks = cell.run().unwrap();
    assert!(ticks > 0);
}

#[test]
fn all_eleven_opcodes_compile() {
    use quilt_as_subleq::*;
    for op in [OP_BIND, OP_LINK, OP_EFFECT, OP_VIEW, OP_TICK, OP_FORGET, OP_PROOF, OP_ROUTE, OP_CRDT, OP_WORLD, OP_TIME] {
        let p = compile(op, 0, 0);
        assert!(p.len() % 3 == 0, "op {} produced non-aligned program (len={})", op, p.len());
    }
}

#[test]
fn port_registry_roundtrip() {
    use port::{Port, PortRegistry};

    let mut reg = PortRegistry::new();
    reg.register(Port::new("quilt://alpha", "input", 13));
    assert!(reg.get("quilt://alpha", "input").is_some());
    assert_eq!(reg.get("quilt://alpha", "input").unwrap().buffer.get(), 13);
}
