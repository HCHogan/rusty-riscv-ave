pub mod bus;
pub mod cpu;
pub mod dram;
pub mod exception;
pub mod param;
pub mod csr;
pub mod uart;
pub mod clint;
pub mod plic;
pub mod interrupt;

use cpu::Cpu;
use std::{
    env,
    fs::File,
    io::{self, Read},
};
use tracing::{debug, error, info, span, warn, Level};
use tracing_subscriber;

#[tracing::instrument]
fn main() -> io::Result<()> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!(
            "Usage:\n\
            - cargo run <filename>"
        );
        return Ok(());
    }

    let mut file = File::open(&args[1])?;
    let mut code = Vec::new();
    file.read_to_end(&mut code)?;

    let mut cpu = Cpu::new(code);

    loop {
        // fetch
        let inst = match cpu.fetch() {
            Ok(inst) => inst,
            Err(e) => {
                cpu.handle_exception(e);
                if e.is_fatal() {
                    error!("{}", e);
                    break;
                }
                continue;
            }
        };

        // execute
        match cpu.execute(inst) {
            Ok(new_pc) => cpu.set_pc(new_pc),
            Err(e) => {
                cpu.handle_exception(e);
                if e.is_fatal() {
                    error!("{}", e);
                    break;
                }
            }
        };

        match cpu.check_pending_interrupt() {
            Some(interrupt) => cpu.handle_exception(interrupt),
            None => (),
        }
    }

    cpu.dump_registers();

    Ok(())
}
