use core::fmt::{Debug, Formatter, Result};

pub struct SvmInstructionError(i32);

impl SvmInstructionError {
    pub fn as_str(&self) -> &str {
        match self.0 {
            0x52 => "Machine check exception",
            0x60 => "Physical INTR",
            0x61 => "Physical NMI",
            0x62 => "Physical SMI",
            0x63 => "Physical INIT",
            0x64 => "Virtual INTR",
            0x77 => "PAUSE instruction",
            0x78 => "HLT instruction",
            0x7f => "Shutdown",
            0x8f => "EFER write trap",
            0x90 => "CR0 write trap",
            0x91 => "CR1 write trap",
            0x92 => "CR2 write trap",
            0x93 => "CR3 write trap",
            0x94 => "CR4 write trap",
            0x95 => "CR5 write trap",
            0x96 => "CR6 write trap",
            0x97 => "CR7 write trap",
            0x98 => "CR8 write trap",
            0x99 => "CR9 write trap",
            0x9a => "CR10 write trap",
            0x9b => "CR11 write trap",
            0x9c => "CR12 write trap",
            0x9d => "CR13 write trap",
            0x9e => "CR14 write trap",
            0x9f => "CR15 write trap",
            0xa5 => "Bus Lock Threshold",
            0xa6 => "HLT instruction of idle",
            0x400 => "Only if PFCODE[3] is 0",
            0x403 => "VMGEXIT instruction",
            -1 => "Invalid guest state",
            -2 => "Busy bit was set in VMSA",
            -3 => "The sibling thread is not idle",
            -4 => "Invalid PMC state",
            _ => "[INVALID]",
        }
    }
}

impl From<i32> for SvmInstructionError {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl Debug for SvmInstructionError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "SvmInstructionError({}, {:?})", self.0, self.as_str())
    }
}

numeric_enum_macro::numeric_enum! {
#[repr(i32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum SvmExitReason {
    VMEXIT_MC = 0x52,
    VMEXIT_INTR = 0x60,
    VMEXIT_NMI = 0x61,
    VMEXIT_SMI = 0x62,
    VMEXIT_INIT = 0x63,
    VMEXIT_VINTR = 0x64,
    VMEXIT_PAUSE = 0x77,
    VMEXIT_HLT = 0x78,
    VMEXIT_SHUTDOWN = 0x7f,
    VMEXIT_EFER_WRITE_TRAP = 0x8f,
    VMEXIT_CR0_WRITE_TRAP = 0x90,
    VMEXIT_CR1_WRITE_TRAP = 0x91,
    VMEXIT_CR2_WRITE_TRAP = 0x92,
    VMEXIT_CR3_WRITE_TRAP = 0x93,
    VMEXIT_CR4_WRITE_TRAP = 0x94,
    VMEXIT_CR5_WRITE_TRAP = 0x95,
    VMEXIT_CR6_WRITE_TRAP = 0x96,
    VMEXIT_CR7_WRITE_TRAP = 0x97,
    VMEXIT_CR8_WRITE_TRAP = 0x98,
    VMEXIT_CR9_WRITE_TRAP = 0x99,
    VMEXIT_CR10_WRITE_TRAP = 0x9a,
    VMEXIT_CR11_WRITE_TRAP = 0x9b,
    VMEXIT_CR12_WRITE_TRAP = 0x9c,
    VMEXIT_CR13_WRITE_TRAP = 0x9d,
    VMEXIT_CR14_WRITE_TRAP = 0x9e,
    VMEXIT_CR15_WRITE_TRAP = 0x9f,
    VMEXIT_BUSLOCK = 0xa5,
    VMEXIT_IDLE_HLT = 0xa6,
    VMEXIT_NPF = 0x400,
    VMEXIT_VMGEXIT = 0x403,
    VMEXIT_INVALID = -1,
    VMEXIT_BUSY = -2,
    VMEXIT_IDLE_REQUIRED = -3,
    VMEXIT_INVALID_PMC = -4,
}
}

#[derive(Debug)]
pub struct SvmExitInfo {
    pub entry_failure: bool,
    pub exit_reason: SvmExitReason,
    pub exit_instruction_length: u32,
    pub guest_rip: usize,
}
