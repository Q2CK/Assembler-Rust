use std::fs;
use std::io::{stdin/*, stdout, Write*/};
mod isa;
mod parse;

fn main() {
    let cpu_name = "Maszyna W+";

    let isa = isa::ISA::new(cpu_name);

    // Create output strings
    let mut bin_output = String::new();
    let mut hex_output = String::new();

    // Get assembly path
    println!("Assembly file name: ");
    let mut asm_file = String::new();
    stdin().read_line(&mut asm_file).expect("Invalid file name");
    let path_asm = &asm_file[..(asm_file.len() - 2)];

    // Create output path
    let path_bin = String::from("bin_") + &asm_file[..(&asm_file.len() - 2)];
    let path_hex = String::from("hex_") + &asm_file[..(&asm_file.len() - 2)];
    bin_output += "LINE|INSTRUCTIONS\n\n";
    hex_output += "LINE|INSTRUCTIONS\n\n";

    // Get assembly line
    bin_output += &parse::Parse(&path_asm, &isa).0;
    hex_output += &parse::Parse(&path_asm, &isa).1;

    // Write results
    match fs::write(&path_bin, bin_output) {
        Ok(_) => println!("Succesfully written to {}", path_bin),
        Err(_) => println!("Failed to write to {}", path_bin),
    }
    match fs::write(&path_hex, hex_output) {
        Ok(_) => println!("Succesfully written to {}", path_hex),
        Err(_) => println!("Failed to write to {}", path_hex),
    }
}
