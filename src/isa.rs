pub struct InstrType {
    pub asm: &'static str,
    pub opcode: &'static str,
    pub bit_lengths: Vec<i8>,
}

pub struct ISA {
    pub name: &'static str,
    pub instr: Vec<InstrType>,
}

impl ISA {
    // TODO: ISA from file
    pub fn new(cpu_name: &'static str) -> ISA {
        ISA { 
            name: cpu_name,
            instr: vec![
                InstrType {asm: "STP", opcode: "00000000", bit_lengths: vec![-4],},
                InstrType {asm: "SLP", opcode: "00000001", bit_lengths: vec![-4],},
                InstrType {asm: "GUW", opcode: "00000010", bit_lengths: vec![-4],},
                InstrType {asm: "CFT", opcode: "00000011", bit_lengths: vec![-4],},
                InstrType {asm: "CFF", opcode: "00000100", bit_lengths: vec![-4],},
                InstrType {asm: "RTN", opcode: "00000101", bit_lengths: vec![-4],},
                InstrType {asm: "RSH", opcode: "00000110", bit_lengths: vec![-4],},
                InstrType {asm: "NIN", opcode: "00000111", bit_lengths: vec![-2, 2],},
                InstrType {asm: "AIN", opcode: "00001000", bit_lengths: vec![-2, 2],},
                InstrType {asm: "CIN", opcode: "00001001", bit_lengths: vec![-2, 2],},
                InstrType {asm: "RIN", opcode: "00001010", bit_lengths: vec![-2, 2],},
                InstrType {asm: "GIN", opcode: "00001011", bit_lengths: vec![-2, 2],},
                InstrType {asm: "FIN", opcode: "00001100", bit_lengths: vec![-2, 2],},
                InstrType {asm: "PIN", opcode: "00001101", bit_lengths: vec![-2, 2],},
                InstrType {asm: "ADD", opcode: "0001", bit_lengths: vec![8],},
                InstrType {asm: "SUB", opcode: "0010", bit_lengths: vec![8],},
                InstrType {asm: "AND", opcode: "0011", bit_lengths: vec![8],},
                InstrType {asm: "IOR", opcode: "0100", bit_lengths: vec![8],},
                InstrType {asm: "XOR", opcode: "0101", bit_lengths: vec![8],},
                InstrType {asm: "CAL", opcode: "0110", bit_lengths: vec![8],},
                InstrType {asm: "PUS", opcode: "0111", bit_lengths: vec![8],},
                InstrType {asm: "POP", opcode: "1000", bit_lengths: vec![8],},
                InstrType {asm: "LOD", opcode: "1001", bit_lengths: vec![8],},
                InstrType {asm: "STR", opcode: "1010", bit_lengths: vec![8],},
                InstrType {asm: "JMF", opcode: "1011", bit_lengths: vec![8],},
                InstrType {asm: "JMP", opcode: "1100", bit_lengths: vec![8],},
                InstrType {asm: "JMO", opcode: "1101", bit_lengths: vec![8],},
                InstrType {asm: "JMM", opcode: "1110", bit_lengths: vec![8],},
                InstrType {asm: "JMZ", opcode: "1111", bit_lengths: vec![8],},
            ],
        }
    }
}