pub mod cpu;
pub mod bus;
pub mod dram;
pub mod exception;
pub mod param;

use std::{
    env,
    fs::File,
    io::{self, Read},
};
use cpu::Cpu;
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

    while cpu.get_pc() < cpu.dram_size() as u64 {
        let inst = cpu.fetch().unwrap();
        cpu.execute(inst);
        cpu.step();
    }

    cpu.dump_registers();

    Ok(())
}
