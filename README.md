# quilt-subleq

> **Quilt becomes more than a registry. It's a distribution of reality.**

A Subleq substrate for Quilt, and a Quilt substrate for Subleq. Two directions of the same insight.

## The chain

1. **Quilt as Subleq.** Every Quilt opcode (BIND, LINK, EFFECT, VIEW, TICK, FORGET, PROOF, ROUTE, CRDT, WORLD, TIME) implemented as a Subleq program on a memory tape. Proves the cell model is substrate-agnostic — runs on the simplest possible computer.

2. **Subleq as Quilt.** Subleq's `[B] = [B] - [A]; if [B] <= 0: goto C` becomes a CELL with a scaling function. The three operands (A, B, C) are no longer memory addresses — they are pointers into a typed block space: memory cells, Quilt cells, or other quilts' input-ports, however those are encoded or reached.

3. **Distribution of reality.** Quilt stops being a registry of cells. It becomes a uniform substrate that spans memory, cells, and remote quilts. The scaling function is the address-resolution layer; the cell tree is the audit trail; the substrate is one continuous fabric.

## Build

```bash
cd /workspace/repos/quilt-subleq
cargo build --release
cargo test
cargo run --example 01_quilt_as_subleq
cargo run --example 02_subleq_as_quilt
cargo run --example 03_distribution_of_reality
```

## Layout

| File | What it does |
|------|--------------|
| `src/subleq.rs` | Pure Subleq interpreter (one instruction: `subleq A B C`) |
| `src/quilt_as_subleq.rs` | The 11 Quilt opcodes compiled to Subleq programs |
| `src/cell.rs` | A Quilt cell that runs Subleq internally |
| `src/block.rs` | The `Block` trait — memory, cell, port, all uniform |
| `src/scaling.rs` | The scaling function — typed-address resolution |
| `src/port.rs` | A remote quilt's input-port, addressed by URL + signature |
| `examples/01_quilt_as_subleq.rs` | Demo: BIND/LINK/VIEW as Subleq |
| `examples/02_subleq_as_quilt.rs` | Demo: Subleq with a scaling function |
| `examples/03_distribution_of_reality.rs` | Demo: cross-quilt call via scaling |

## Why Subleq

Subleq (`SUBtract and branch if Less-than or EQual`) is one instruction. Turing complete. Memory is a flat tape of integers. The instruction is `[B] = [B] - [A]; if [B] <= 0: goto C; else pc += 3`. Three operands. No registers. No decoder. The whole CPU is a subtract, a conditional, and a counter.

If Quilt's 11 opcodes compile cleanly to Subleq, then the cell model is substrate-free. The cell model is the pattern, not the hardware. The hardware is the smallest computer that can run a pattern.

## Why a scaling function

Standard Subleq has three integer operands. Each is a memory address. The hardware does subtraction.

Quilt-Subleq replaces the integers with typed pointers. A pointer can be:
- a memory cell (local integer tape)
- a Quilt cell (reactive graph node)
- a remote quilt's input-port (URL + signature + encoding)

The scaling function is the address resolver. It takes a typed pointer and returns a block that the instruction can read or write. The instruction's subtract happens between the resolved values, not between the addresses.

This is what Casey means by "distribution of reality." The Quilt sheet is no longer asking "what cell is this?" It's asking "what block does this point to, however it's encoded or reached?"

## Why this is a distribution, not a registry

A registry knows the cells it registered. A distribution knows how to reach any block whose address it can resolve. The scaling function is the resolution layer. The merkle tree is the audit trail. Witness cells catch the resolutions that diverge from the substrate.

Subleq on a memory tape is local. Subleq with a scaling function can call across the network. Quilt stops being a list of cells in a sheet. Quilt becomes a continuous fabric where memory, cells, and remote ports are all the same kind of thing.

That's the distribution.

## License

MIT
