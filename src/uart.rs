use crate::{param::*, exception::Exception};
use std::{
    io::{self, Read, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex,
    },
    thread::{self}, 
};

pub struct Uart {
    /// Pair of an array for UART buffer and a conditional variable.
    uart: Arc<(Mutex<[u8; UART_SIZE as usize]>, Condvar)>,
    /// Bit if an interrupt happens.
    interrupt: Arc<AtomicBool>,
}

impl Uart {
    /// Create a new UART.
    pub fn new() -> Self {
        let mut array = [0; UART_SIZE as usize];
        array[UART_LSR as usize] |= MASK_UART_LSR_TX;

        let uart = Arc::new((Mutex::new(array), Condvar::new()));
        let interrupt = Arc::new(AtomicBool::new(false));

        // receive part
        let read_uart = Arc::clone(&uart);
        let read_interrupt = Arc::clone(&interrupt);
        let mut byte = [0];
        thread::spawn(move || loop {
            match io::stdin().read(&mut byte) {
                Ok(_) => {
                    let (uart, cvar) = &*read_uart;
                    let mut array = uart.lock().unwrap();
                    // if data have been received but not yet be transferred.
                    // this thread wait for it to be transferred.
                    while (array[UART_LSR as usize] & MASK_UART_LSR_RX) == 1 {
                        array = cvar.wait(array).unwrap();
                    }
                    // data have been transferred, so receive the next one.
                    array[UART_RHR as usize] = byte[0];
                    // set the read_interrupt to true.
                    read_interrupt.store(true, Ordering::Release);
                    // set the RX bit in LSR.
                    array[UART_LSR as usize] |= MASK_UART_LSR_RX;

                }
                Err(e) => println!("{}", e),
            }
        });

        Self { uart, interrupt }
    }

    pub fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        if size != 8 {
            return Err(Exception::LoadAccessFault(addr));
        }
        let (uart, cvar) = &*self.uart;
        let mut array = uart.lock().unwrap();
        let index = addr - UART_BASE;
        // a read happends
        match index {
            UART_RHR => {
                // TODO: move this down to the end of this branch.
                cvar.notify_one();
                // Read the data from RHR and clear the RX bit in LSR.
                array[UART_LSR as usize] &= !MASK_UART_LSR_RX;
                Ok(array[UART_RHR as usize] as u64)
            }
            _ => Ok(array[index as usize] as u64),
        }
    }

    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        if size != 8 {
            return Err(Exception::StoreAMOAccessFault(addr));
        }
        let (uart, _cvar) = &*self.uart;
        let mut array = uart.lock().unwrap();
        let index = addr - UART_BASE;
        match index {
            UART_THR => {
                print!("{}", value as u8 as char);
                io::stdout().flush().unwrap();
                Ok(())
            }
            _ => {
                array[index as usize] = value as u8;
                Ok(())
            }
        }
    }

    pub fn is_interrupting(&self) -> bool {
        self.interrupt.swap(false, Ordering::Acquire)
    }
}
