/// Bus is a communication medium of CPU and various IO devices.
/// Bus allocates different address for differet devices.
/// By sending instruction through bus, CPU can operate the IO devices indirectly.
/// Bus also provides two function: store and load.
use crate::{
    clint::Clint,
    dram::Dram,
    exception::Exception,
    param::{DRAM_BASE, DRAM_END},
    plic::Plic,
    uart::Uart,
    param::*,
    virtio::*,
};

pub struct Bus {
    dram: Dram,
    clint: Clint,
    plic: Plic,
    pub uart: Uart,
    pub virtio_blk: VirtioBlock,
}

impl Bus {
    /// Create a bus from given code.
    pub fn new(code: Vec<u8>, disk_image: Vec<u8>) -> Bus {
        Self {
            dram: Dram::new(code),
            clint: Clint::new(),
            plic: Plic::new(),
            uart: Uart::new(),
            virtio_blk: VirtioBlock::new(disk_image),
        }
    }

    /// Checks the address and call load on dram.
    pub fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        match addr {
            CLINT_BASE..=CLINT_END => self.clint.load(addr, size),
            PLIC_BASE..=PLIC_END => self.plic.load(addr, size),
            DRAM_BASE..=DRAM_END => self.dram.load(addr, size),
            UART_BASE..=UART_END => self.uart.load(addr, size),
            VIRTIO_BASE..=VIRTIO_END => self.virtio_blk.load(addr, size),
            _ => Err(Exception::LoadAccessFault(addr)),
        }
    }

    /// Checks the address and call store on dram.
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        match addr {
            CLINT_BASE..=CLINT_END => self.clint.store(addr, size, value),
            PLIC_BASE..=PLIC_END => self.plic.store(addr, size, value),
            DRAM_BASE..=DRAM_END => self.dram.store(addr, size, value),
            UART_BASE..=UART_END => self.uart.store(addr, size, value),
            VIRTIO_BASE..=VIRTIO_END => self.virtio_blk.store(addr, size, value),
            _ => Err(Exception::StoreAMOAccessFault(addr)),
        }
    }

    /// Get the dram size.
    pub fn dram_size(&self) -> usize {
        self.dram.len()
    }
}
