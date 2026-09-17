use quilt_subleq::quilt_as_subleq::*;

fn main() {
    let tape_size = 100;
    
    let ops = [
        (OP_BIND, "BIND", 10, 42),
        (OP_LINK, "LINK", 10, 42),
        (OP_EFFECT, "EFFECT", 10, 0),
        (OP_VIEW, "VIEW", 10, 11),
        (OP_TICK, "TICK", 10, 0),
        (OP_FORGET, "FORGET", 10, 0),
        (OP_PROOF, "PROOF", 10, 0),
        (OP_ROUTE, "ROUTE", 10, 0),
        (OP_CRDT, "CRDT", 10, 0),
        (OP_WORLD, "WORLD", 10, 0),
        (OP_TIME, "TIME", 10, 0),
    ];
    
    for (op, name, addr, val) in ops {
        let prog = compile(op, addr, val);
        println!("{}: prog len {}", name, prog.len());
        for (i, v) in prog.iter().enumerate() {
            if i % 3 == 0 { print!("  pc+{}:", i); }
            print!(" {}", v);
            if (i+1) % 3 == 0 { println!(); }
        }
    }
}
