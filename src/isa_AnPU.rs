pub struct InstrType {
    pub asm: &'static str,
    pub opcode: &'static str,
    pub bit_lengths: Vec<i8>,
}

pub struct ISA {
    pub name: &'static str,
    pub instr: Vec<InstrType>,
    pub opcode_length: usize,
    pub hex_len: usize,
}

impl ISA {
    // TODO: ISA from file
    pub fn new(cpu_name: &'static str) -> ISA {
        ISA {
            name: cpu_name,
            instr: vec![
                InstrType {asm: "CHILL", opcode: "0000000", bit_lengths: vec![-9],},
                InstrType {asm: "NFADD", opcode: "0000100", bit_lengths: vec![3, 3, 3],},
                InstrType {asm: "NFSUB", opcode: "0000101", bit_lengths: vec![3, 3, 3],},
                InstrType {asm: "NFAND", opcode: "0000110", bit_lengths: vec![3, 3, 3],},
                InstrType {asm: "NFNOR", opcode: "0000111", bit_lengths: vec![3, 3, 3],},
                InstrType {asm: "NFXOR", opcode: "0001000", bit_lengths: vec![3, 3, 3],},
                InstrType {asm: "NFNOR", opcode: "0001001", bit_lengths: vec![3, 3, 3],},
                InstrType {asm: "NFPCA", opcode: "0001010", bit_lengths: vec![3, 3, 3],},
                InstrType {asm: "NFPCS", opcode: "0001011", bit_lengths: vec![3, 3, 3],},
                InstrType {asm: "NFMOV", opcode: "0001100", bit_lengths: vec![3, -3, 3],},
                InstrType {asm: "NFINC", opcode: "0001101", bit_lengths: vec![3, -3, 3],},
                InstrType {asm: "NFDEC", opcode: "0001110", bit_lengths: vec![3, -3, 3],},
                InstrType {asm: "NFRSH", opcode: "0001111", bit_lengths: vec![3, -3, 3],},
                InstrType {asm: "LDIMM", opcode: "00100", bit_lengths: vec![8, 3],},
                InstrType {asm: "FLADD", opcode: "0010100", bit_lengths: vec![3, 3, 3],},
                InstrType {asm: "FLSUB", opcode: "0010101", bit_lengths: vec![3, 3, 3],},
                InstrType {asm: "FLAND", opcode: "0010110", bit_lengths: vec![3, 3, 3],},
                InstrType {asm: "FLNOR", opcode: "0010111", bit_lengths: vec![3, 3, 3],},
                InstrType {asm: "FLXOR", opcode: "0011000", bit_lengths: vec![3, 3, 3],},
                InstrType {asm: "FLNOR", opcode: "0011001", bit_lengths: vec![3, 3, 3],},
                InstrType {asm: "FLPCA", opcode: "0011010", bit_lengths: vec![3, 3, 3],},
                InstrType {asm: "FLPCS", opcode: "0011011", bit_lengths: vec![3, 3, 3],},
                InstrType {asm: "FLMOV", opcode: "0011100", bit_lengths: vec![3, -3, 3],},
                InstrType {asm: "FLINC", opcode: "0011101", bit_lengths: vec![3, -3, 3],},
                InstrType {asm: "FLDEC", opcode: "0011110", bit_lengths: vec![3, -3, 3],},
                InstrType {asm: "FLRSH", opcode: "0011111", bit_lengths: vec![3, -3, 3],},
                InstrType {asm: "BRNCH", opcode: "01000", bit_lengths: vec![-1, 7, 3],},
                InstrType {asm: "DIMRD", opcode: "01001", bit_lengths: vec![5, -3, 3],},
                InstrType {asm: "DIMUP", opcode: "01010", bit_lengths: vec![5, 3, -3],},
                InstrType {asm: "INMRD", opcode: "01011", bit_lengths: vec![2, -6, 3],},
                InstrType {asm: "INMUP", opcode: "01100", bit_lengths: vec![2, -3, 3, -3],},
                InstrType {asm: "PTUPD", opcode: "01101", bit_lengths: vec![-5, 3, -3],},
                InstrType {asm: "HLTCL", opcode: "01110", bit_lengths: vec![1, -10],},
                InstrType {asm: "INOUT", opcode: "01111", bit_lengths: vec![-1, 1, 3, 3, 3],},
                InstrType {asm: "PCALL", opcode: "10000", bit_lengths: vec![-1, 7, 3],},
                InstrType {asm: "PRTRN", opcode: "10001", bit_lengths: vec![-8, 3],},
                InstrType {asm: "DROUT", opcode: "10010", bit_lengths: vec![2, 3, 3, -3],},
            ],
            opcode_length: 5,
            hex_len: 4,
        }
    }
}