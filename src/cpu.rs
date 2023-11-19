use crate::{
    bus::Bus, 
    exception::Exception, 
    csr::Csr,
    param::{DRAM_END, DRAM_BASE}
};
use tracing::{
    debug, error, info, span, warn, Level
};


const RVABI: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", 
    "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5", 
    "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", 
    "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6",
];

/// The privileged mode.
#[derive(Debug, PartialEq, PartialOrd, Eq, Copy, Clone)]
pub enum Mode {
    User = 0b00,
    Supervisor = 0b01,
    Machine = 0b11,
}

/// Access type that is used in the virtual address translation process. It decides which exception
/// should raises (InstructionPageFault, LoadPageFault or StoreAMOPageFault).
#[derive(Debug, PartialEq, PartialOrd)]
pub enum AccessType {
    /// Raises the exception InstructionPageFault. It is used for an instruction fetch.
    Instruction,
    /// Raises the exception LoadPageFault.
    Load,
    /// Raises the exception StoreAMOPageFault.
    Store,
}

pub struct Cpu {
    /// 32 64-bit registers
    regs: [u64; 32],
    /// Program counter
    pc: u64,
    /// Our phisical memory
    bus: Bus,
    /// Current previledge mode
    // pub mode: Mode,
    /// Control and status registers. RISC-V ISA sets aside a 12-bit encoding space (csr[11:0]) for
    /// up to 4096 CSRs.
    pub csr: Csr,
}

impl Cpu {
    /// Create a new CPU with the given code
    pub fn new(code: Vec<u8>) -> Self {
        let mut regs = [0 as u64; 32];
        // set stack pointer to the end of dram
        regs[2] = DRAM_END;
        let bus = Bus::new(code);
        let csr = Csr::new();
        Self { regs, pc: DRAM_BASE, bus, csr }
    }

    /// Load a value from a CSR.
    pub fn load_csr(&self, addr: usize) -> u64 {
        match addr {
            SIE => self.csrs[MIE] & self.csrs[MIDELEG],
            _ => self.csrs[addr],
        }
    }

    /// Store a value to a CSR.
    pub fn store_csr(&mut self, addr: usize, value: u64) {
        match addr {
            SIE => {
                self.csrs[MIE] =
                    (self.csrs[MIE] & !self.csrs[MIDELEG]) | (value & self.csrs[MIDELEG]);
            }
            _ => self.csrs[addr] = value,
        }
    }

    /// Load a value from a dram.
    pub fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        self.bus.load(addr, size)
    }

    /// Store a value to a dram.
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        self.bus.store(addr, size, value)
    }

    /// Get an instruction from the dram.
    pub fn fetch(&mut self) -> Result<u64, Exception> {
        self.bus.load(self.pc, 32)
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
        self.bus.dram_size()
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

    /// Get a register value by name.
    pub fn reg(&self, r: &str) -> u64 {
        match RVABI.iter().position(|&x| x == r) {
            Some(i) => self.regs[i],
            None => match r {
                "pc" => self.pc,
                "fp" => self.reg("s0"),
                r if r.starts_with("x") => {
                    if let Ok(i) = r[1..].parse::<usize>() {
                        if i <= 31 { return self.regs[i]; }
                        panic!("Invalid register {}", r);
                    }
                    panic!("Invalid register {}", r);
                }
                _ => panic!("Invalid register {}", r),
            }
        }
    }

    pub fn execute(&mut self, inst: u64) -> Result<u64, Exception> {
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

        Ok(self.pc + 4)
    }
}

#[cfg(test)]
mod test {
    use std::{
        process::Command,
        io::{Write, Read},
        fs::File,
    };

    use super::*;
    
    fn generate_rv_assembly(c_src: &str) {
        let cc = "clang";
        let output = Command::new(cc)
            .arg("-S")
            .arg(c_src)
            .arg("-nostdlib")
            .arg("-march=rv64g")
            .arg("-mabi=lp64")
            .arg("--target=riscv64")
            .arg("-mno-relax")
            .output()
            .expect("Failed to generate rv assembly");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }

    fn generate_rv_obj(assembly: &str) {
        let cc = "clang";
        let pieces: Vec<&str> = assembly.split(".").collect();
        let output = Command::new(cc).arg("-Wl,-Ttext=0x0")
            .arg("-nostdlib")
            .arg("-march=rv64g")
            .arg("-mabi=lp64")
            .arg("--target=riscv64")
            .arg("-mno-relax")
            .arg("-o")
            .arg(&pieces[0])
            .arg(assembly)
            .output()
            .expect("Failed to generate rv object");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }

    fn generate_rv_binary(obj: &str) {
        let objcopy = "llvm-objcopy";
        let output = Command::new(objcopy).arg("-O")
            .arg("binary")
            .arg(obj)
            .arg(obj.to_owned() + ".bin")
            .output()
            .expect("Failed to generate rv binary");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }

    fn rv_helper(code: &str, testname: &str, n_clock: usize) -> Result<Cpu, std::io::Error> {
        let filename = testname.to_owned() + ".s";
        let mut file = File::create(&filename)?;
        file.write(&code.as_bytes())?;
        generate_rv_obj(&filename);
        generate_rv_binary(testname);
        let mut file_bin = File::open(testname.to_owned() + ".bin")?;
        let mut code = Vec::new();
        file_bin.read_to_end(&mut code)?;
        let mut cpu = Cpu::new(code);

        for _i in 0..n_clock {
            let inst = match cpu.fetch() {
                Ok(inst) => inst,
                Err(_err) => break,
            };
            match cpu.execute(inst) {
                Ok(new_pc) => cpu.pc = new_pc,
                Err(err) => error!("execute instruction failed: {:?}", err),
            };
        }

        return Ok(cpu);
    }

    macro_rules! riscv_test {
        ( $code:expr, $name:expr, $clock:expr, $($real:expr => $expect:expr),* ) => {
            match rv_helper($code, $name, $clock) {
                Ok(cpu) => { 
                    $(assert_eq!(cpu.reg($real), $expect);)*
                }
                Err(e) => { println!("error: {}", e); assert!(false); }
            } 
        };
    }

    #[test]
    fn test_addi() {
        let code = "addi x31, x0, 42";
        riscv_test!(code, "test_addi", 1, "x31" => 42);
    }
}
