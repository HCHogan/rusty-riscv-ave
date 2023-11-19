/// Memory has two function: store and load. Only store and load a 8-bit,
/// 16-bit, 32-bit and 64-bit are allowed.
use crate::{
    exception::Exception,
    param::{DRAM_BASE, DRAM_SIZE},
};

pub struct Dram {
    pub dram: Vec<u8>,
}

impl Dram {
    /// Create a new dram with the given code
    pub fn new(code: Vec<u8>) -> Dram {
        let mut dram = vec![0; DRAM_SIZE as usize];
        dram[..code.len()].copy_from_slice(&code);
        Self { dram }
    }

    /// Load data of size from addr in memory
    // addr/size must be valid. Check in bus
    pub fn load(&self, addr: u64, size: u64) -> Result<u64, Exception> {
        if ![8, 16, 32, 64].contains(&size) {
            return Err(Exception::LoadAccessFault(addr));
        }

        let nbytes = size / 8;
        let index = (addr - DRAM_BASE) as usize;
        let mut code: u64 = 0;
        (0..nbytes).for_each(|i| {
            code |= (self.dram[index + i as usize] as u64) << (8 * i);
        });

        Ok(code)
    }

    /// Store value of size to addr in memory
    // addr/size must be valid. Check in bus
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        if ![8, 16, 32, 64].contains(&size) {
            return Err(Exception::StoreAMOAccessFault(addr));
        }

        let nbytes = size / 8;
        let index = (addr - DRAM_BASE) as usize;
        (0..nbytes).for_each(|i| {
            self.dram[index + i as usize] = ((value >> (8 * i)) & 0xff) as u8;
        });
        Ok(())
    }

    /// Return dram size
    pub fn len(&self) -> usize {
        self.dram.len()
    }
}
