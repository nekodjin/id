mod instr;
mod machine;
mod page;

use std::io::{self, Read};

use atty::Stream;
use instr::instrs;
use machine::Machine;

fn main() {
    let interactive = atty::is(Stream::Stdin);

    if interactive {
        println!("Enter program below:");
    }

    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("error reading from stdin");

    if interactive {
        println!("Beginning interpreter...");
    }

    let mut machine = Machine::new();
    let instrs = instrs(&buffer);

    machine.run(instrs);
}
