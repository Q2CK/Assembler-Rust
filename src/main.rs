use std::fs;
use std::io::{stdin/*, stdout, Write*/};

struct InstrType {
    asm: &'static str,
    opcode: &'static str,
    bit_lengths: Vec<u8>,
}

struct ISA {
    name: &'static str,
    instr: Vec<InstrType>,
}

fn main() {
    let CPU_name = "Maszyna W+";

    let isa = ISA { 
        name: CPU_name,
        instr: vec![
            InstrType {asm: "ADD", opcode: "0000", bit_lengths: vec![2, 3, 3, 3],},
            InstrType {asm: "SUB", opcode: "0001", bit_lengths: vec![2, 3, 3, 3],},
            InstrType {asm: "AND", opcode: "0010", bit_lengths: vec![2, 3, 3, 3],},
            //i tak dalej
        ],
    };

    //println!("{}", ISA.instr[2].asm);

    println!("Assembly file name: ");

    let mut asm_file = String::new();
    stdin().read_line(&mut asm_file).expect("Invalid file name");
    let path = &asm_file[..(asm_file.len() - 2)];

    let test = format!("bajojajo{}", 2);
    
    fs::write(path, test).expect("Failed to write");
}
