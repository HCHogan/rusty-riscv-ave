/// All kinds of exceptions, an unusual condition occurring at run
/// time associated with an instruction in the current hardware thread.
#[derive(Debug)]
pub enum Exception {
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault(u64),
    StoreAMOAddressMisaligned,
    StoreAMOAccessFault(u64),
    EnvironmentCallFromUMode,
    EnvironmentCallFromSMode,
    EnvironmentCallFromMMode,
    InstructionPageFault,
    LoadPageFault,
    StoreAMOPageFault,
}
