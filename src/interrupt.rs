use crate::param::*;

/// When a hart is executing in privilege mode x, interrupts are globally enabled when xIE=1 and globally disabled
/// when xIE=0. Interrupts for lower-privilege modes, w < x, are always globally disabled regardless
/// of the setting of any global wIE bit for the lower-privilege mode. Interrupts for higher-privilege modes,
/// y > x, are always globally enabled regardless of the setting of the global yIE bit for the higher-privilege
/// mode. Higher-privilege-level code can use separate per-interrupt enable bits to disable selected higher-privilege-mode
/// interrupts before ceding control to a lower-privilege mode.
///
/// An interrupt i will trap to M-mode (causing the privilege mode to change to M-mode) if all of the following are true:
/// (a) either the current privilege mode is M and the MIE bit in the mstatus register is set, or the current privilege mode has less privilege than M-mode;
/// (b) bit i is set in both mip and mie;
/// (c) if register mideleg exists, bit i is not set in mideleg.
///
/// Trap in S-mode is quite similarly.
/// Multiple simultaneous interrupts destined for M-mode are handled in the following decreasing priority order: MEI, MSI, MTI, SEI, SSI, STI.
///
/// Read the Section 3.1.6.1, 3.1.9 and 4.1.3 of RISC-V Privileged for more information.
pub enum Interrupt {
    SupervisorSoftwareInterrupt,
    MachineSoftwareInterrupt,
    SupervisorTimerInterrupt,
    MachineTimerInterrupt,
    SupervisorExternalInterrupt,
    MachineExternalInterrupt,
}

impl Interrupt {
    pub fn code(self) -> u64 {
        use Interrupt::*;
        match self {
            SupervisorSoftwareInterrupt => 1 | MASK_INTERRUPT_BIT,
            MachineSoftwareInterrupt => 3 | MASK_INTERRUPT_BIT,
            SupervisorTimerInterrupt => 5 | MASK_INTERRUPT_BIT,
            MachineTimerInterrupt => 7 | MASK_INTERRUPT_BIT,
            SupervisorExternalInterrupt => 9 | MASK_INTERRUPT_BIT,
            MachineExternalInterrupt => 11 | MASK_INTERRUPT_BIT,
        }
    }
}
