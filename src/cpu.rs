use tracing::{debug, error, info, span, warn, Level};

/// Our initial memory size
// TODO: Make this configurable
pub const DRAM_SIZE: usize = 1024 * 1024 * 1024;

const RVABI: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", 
    "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5", 
    "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", 
    "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6",
];

pub struct Cpu {
    /// 32 64-bit registers
    regs: [u64; 32],
    /// Program counter
    pc: u64,
    /// Our phisical memory
    dram: Vec<u8>,
}

impl Cpu {
    /// Create a new CPU with the given code
    pub fn new(code: Vec<u8>) -> Self {
        let mut regs = [0 as u64; 32];
        regs[2] = DRAM_SIZE as u64 - 1;
        Self {
            regs,
            pc: 0,
            dram: code,
        }
    }

    /// Dump the registers in a readable format.
    pub fn dump_registers(&mut self) {
        println!("{:-^80}", "registers");
        let mut output = String::new();
        self.regs[0] = 0;

        for i in (0..32).step_by(4) {
            let i0 = format!("x{}", i);
            let i1 = format!("x{}", i + 1); 
            let i2 = format!("x{}", i + 2);
            let i3 = format!("x{}", i + 3); 
            let line = format!(
                "{:3}({:^4}) = {:<#18x} {:3}({:^4}) = {:<#18x} {:3}({:^4}) = {:<#18x} {:3}({:^4}) = {:<#18x}\n",
                i0, RVABI[i], self.regs[i], 
                i1, RVABI[i + 1], self.regs[i + 1], 
                i2, RVABI[i + 2], self.regs[i + 2], 
                i3, RVABI[i + 3], self.regs[i + 3],
            );
            output = output + &line;
        }
        info!("{}", output);
    }

    // Return dram size
    pub fn dram_size(&self) -> usize {
        self.dram.len()
    }

    pub fn set_pc(&mut self, pc: u64) {
        self.pc = pc;
    }

    // Return current pc value
    pub fn get_pc(&self) -> u64 {
        self.pc
    }

    // add pc by 4
    pub fn step(&mut self) {
        self.pc += 4;
    }

    /// Fetch the next instruction in little-endian format
    /// A RISCV instruction is 32-bit long, So we read 4 bytes from the memory
    pub fn fetch(&self) -> u32 {
        let index = self.pc as usize;
        self.dram[index] as u32
            | ((self.dram[index + 1] as u32) << 8)
            | ((self.dram[index + 2] as u32) << 16)
            | ((self.dram[index + 3] as u32) << 24)
    }

    pub fn execute(&mut self, inst: u32) {
        // decode instruction as R-type
        let opcode = inst & 0x7f;
        let rd = ((inst >> 7) & 0x1f) as usize;
        let rs1 = ((inst >> 15) & 0x1f) as usize;
        let rs2 = ((inst >> 20) & 0x1f) as usize;
        let funct3 = (inst >> 12) & 0x7;
        let funct7 = (inst >> 25) & 0x7f;

        // x0 is hardwired as 0
        self.regs[0] = 0;

        // execute stage
        match opcode {
            0x13 => {
                // addi
                // Adds the sign-extended immediate to register x[rs1] and writes the result to x[rd].
                // Arithmetic overflow is ignored.
                let imm = ((inst & 0xfff0_0000) as i64 >> 20) as u64;
                self.regs[rd] = self.regs[rs1].wrapping_add(imm);
            }
            0x33 => {
                // add
                // Adds register x[rs2] to register x[rs1] and writes the result to x[rd].
                // Arithmetic overflow is ignored.
                self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
            }

            _ => {
                // not implemented yet.
                error!("Invalid opcode: {:#x}", opcode);
            }
        }
    }
}
