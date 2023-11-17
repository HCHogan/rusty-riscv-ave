/// Our initial memory size
// TODO: Make this configurable
pub const DRAM_SIZE: usize = 1024 * 1024 * 1024;

struct Cpu {
    /// 32 64-bit registers
    regs: [u64; 32],
    /// Program counter
    pc: u64,
    /// Our phisical memory
    dram: Vec<u8>,
}

impl Cpu {
    /// Create a new CPU with the given code
    fn new(code: Vec<u8>) -> Self {
        let mut regs = [0; 32];
        regs[2] = DRAM_SIZE - 1;
        Self {
            regs,
            pc: 0,
            dram: code,
        }
    }

    /// Fetch the next instruction
    /// A RISCV instruction is 32-bit long, So we read 4 bytes from the memory
    fn fetch(&self) -> u32 {
        let index = self.pc as usize;
        let inst = self.dram[index] as u32
            | ((self.dram[index + 1] as u32) << 8)
            | ((self.dram[index + 2] as u32) << 16)
            | ((self.dram[index + 3] as u32) << 24);
        return inst;
    }
}


