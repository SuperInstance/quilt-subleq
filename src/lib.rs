//! quilt-subleq
//!
//! Quilt on Subleq, and Subleq on Quilt.
//!
//! Two directions of one insight:
//! 1. Quilt's 11 opcodes compile to Subleq programs (substrate-free proof).
//! 2. Subleq gets a scaling function that resolves typed pointers into blocks.
//! 3. Blocks are memory, cells, or remote quilts' input-ports — uniform.
//!
//! Casey: "Quilt becomes more than a registry. It's a distribution of reality."

pub mod subleq;
pub mod block;
pub mod scaling;
pub mod cell;
pub mod port;
pub mod quilt_as_subleq;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod trace_test {
    use super::*;
    
    #[test]
    fn trace_link() {
        let prog = quilt_as_subleq::link_program(50, 999);
        eprintln!("link_program (24 cells):");
        for (i, v) in prog.iter().enumerate() {
            if i % 3 == 0 { eprint!("\n  pc+{}:", i); }
            eprint!(" {}", v);
        }
        eprintln!();
        
        let mut cell = cell::SubleqCell::new("trace", prog, 1000);
        let r = cell.run();
        eprintln!("run result: {:?}", r);
        eprintln!("mem[12] = {}", cell.tape[12]);
        eprintln!("mem[50] = {}", cell.tape[50]);
    }
}

#[cfg(test)]
mod trace_test2 {
    use super::*;
    
    #[test]
    fn trace_link_2() {
        let prog = quilt_as_subleq::link_program(50, 999);
        let mut tape = vec![0i64; 1000];
        for (i, &v) in prog.iter().enumerate() {
            tape[i] = v;
        }
        let mut m = subleq::Machine::new(tape);
        for step in 0..10 {
            if m.halted { eprintln!("halted after {} steps", step); break; }
            eprintln!("step {}: pc={} instr={},{},{} mem[12]={} mem[50]={}", 
                step, m.pc, m.mem[m.pc], m.mem[m.pc+1], m.mem[m.pc+2], m.mem[12], m.mem[50]);
            m.step().unwrap();
        }
        eprintln!("final: mem[12] = {} mem[50] = {}", m.mem[12], m.mem[50]);
    }
}

#[cfg(test)]
mod trace_bind {
    use super::*;
    
    #[test]
    fn trace_bind() {
        let prog = quilt_as_subleq::bind_program(50, 7);
        let mut tape = vec![0i64; 100];
        for (i, &v) in prog.iter().enumerate() {
            tape[i] = v;
        }
        let mut m = subleq::Machine::new(tape);
        for step in 0..10 {
            if m.halted { eprintln!("halted after {} steps", step); break; }
            eprintln!("step {}: pc={} instr={},{},{} mem[12]={} mem[50]={}", 
                step, m.pc, m.mem[m.pc], m.mem[m.pc+1], m.mem[m.pc+2], m.mem[12], m.mem[50]);
            m.step().unwrap();
        }
        eprintln!("final: mem[12] = {} mem[50] = {}", m.mem[12], m.mem[50]);
    }
}
