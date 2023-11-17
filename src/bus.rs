/// Bus is a communication medium of CPU and various IO devices.
/// Bus allocates different address for differet devices.
/// By sending instruction through bus, CPU can operate the IO devices indirectly.
/// Bus also provides two function: store and load.
use crate::{dram::Dram, exception::Exception, param::{DRAM_BASE, DRAM_END}};

pub struct Bus {
    dram: Dram,
}

impl Bus {
    /// Create a bus from given code.
    pub fn new(code: Vec<u8>) -> Bus {
        Self {
            dram: Dram::new(code),
        }
    }

    /// Checks the address and call load on dram.
    pub fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        match addr {
            DRAM_BASE..=DRAM_END => self.dram.load(addr, size),
            _ => Err(Exception::LoadAccessFault(addr)),
        }
    }

    /// Checks the address and call store on dram.
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        match addr {
            DRAM_BASE..=DRAM_END => self.dram.store(addr, size, value),
            _ => Err(Exception::StoreAMOAccessFault(addr)),
        }
    }

    /// Get the dram size.
    pub fn dram_size(&self) -> usize {
        self.dram.len()
    }
}
