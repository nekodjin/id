pub enum Instr {
    I,
    D,
}

pub fn instrs(input: &str) -> Vec<Instr> {
    let mut instrs = Vec::new();

    for c in input.chars() {
        match c {
            'I' => instrs.push(Instr::I),
            'D' => instrs.push(Instr::D),
            _ => (),
        }
    }

    instrs
}
