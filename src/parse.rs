use std::fs;
use std::u32;
use crate::isa::ISA;

fn to_hex(val: &str, len: usize) -> String {
    let n: u32 = u32::from_str_radix(val, 2).unwrap();
    format!("{:01$x}", n, len)
}

pub fn Parse(path_asm: &str, isa: &ISA) -> (String, String){
    let mut bin_output = String::new();
    let mut hex_output = String::new();

    let asm = match fs::read(&path_asm) {
        Ok(v) => String::from_utf8(v).expect("Invalid UTF-8"),
        Err(_) => String::from("Failed to read file"),
    };

    for (i, line) in asm.split("\n").enumerate() {
        let tokens: Vec<&str> = line.split(" ").collect();

        let mut bin_line = String::new();

        for instr in &isa.instr {
            if &tokens[0][..3] == instr.asm {
                bin_line += instr.opcode;
                let mut i_token = 0;
                for arg_length in &instr.bit_lengths {
                    let abs_arg_length: usize = arg_length.abs().try_into().unwrap();
                    let zero = "0";
                    match arg_length > &0 {
                        false => bin_line += &format!("{zero:0>0$}", abs_arg_length),
                        true => {
                            i_token += 1;
                            let arg: u8 = tokens[i_token].trim_end().parse().unwrap();
                            let bin_arg = format!("{arg:b}");
                            bin_line += &format!("{bin_arg:0>0$}", abs_arg_length);
                        }
                    }
                }
            }       
        }
        hex_output += &(format!("{i:0>3}> ") + &to_hex(&bin_line, 3).to_uppercase() + "\n");
        bin_output += &(format!("{i:0>3}> ") + &bin_line + "\n");
    }
    (bin_output, hex_output)
}