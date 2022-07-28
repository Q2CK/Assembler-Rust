use std::fs;
use std::io::{stdin/*, stdout, Write*/};
mod isa;
mod parse;

fn main() {
    let cpu_name = "Maszyna W+";

    let isa = isa::ISA::new(cpu_name);

    /*let isa = ISA { 
        name: cpu_name,
        instr: vec![
            InstrType {asm: "ADD", opcode: "0000", bit_lengths: vec![2, 3, 3, 3],},
            InstrType {asm: "SUB", opcode: "0001", bit_lengths: vec![2, 3, 3, 3],},
            InstrType {asm: "AND", opcode: "0010", bit_lengths: vec![2, 3, 3, 3],},
            //i tak dalej
        ],
    };*/

    println!("{} {}", isa.instr[2].asm, isa.instr[2].opcode);

    /*let x = 100;
    let padding: usize = 8;
    let s = format!("{x:b}");
    println!("{s:0>0$}", padding);*/

    // Get assembly path
    println!("Assembly file name: ");
    let mut asm_file = String::new();
    stdin().read_line(&mut asm_file).expect("Invalid file name");
    let path_asm = &asm_file[..(asm_file.len() - 2)];

    // Create output path
    let path_bin = String::from("bin_") + &asm_file[..(&asm_file.len() - 2)];
    let path_hex = String::from("hex_") + &asm_file[..(&asm_file.len() - 2)];
    fs::write(&path_bin, String::from("LINE|INSTRUCTIONS\n\n")).expect("Failed to write");
    fs::write(&path_hex, String::from("LINE|INSTRUCTIONS\n\n")).expect("Failed to write");

    // Get assembly line
    let bin_output = parse::Parse(&path_asm);

    println!("{}", bin_output);
    
    fs::write(&path_bin, bin_output).expect("Failed to write");
}
