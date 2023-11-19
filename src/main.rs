pub mod bus;
pub mod cpu;
pub mod dram;
pub mod exception;
pub mod param;
pub mod csr;

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
        let inst = match cpu.fetch() {
            Ok(inst) => inst,
            Err(e) => {
                error!("fetch instruction failed: {:?}", e);
                break;
            }
        };

        match cpu.execute(inst) {
            Ok(new_pc) => cpu.set_pc(new_pc),
            Err(e) => {
                error!("execute instruction failed: {:?}", e);
                break;
            }
        };
    }

    cpu.dump_registers();

    Ok(())
}
