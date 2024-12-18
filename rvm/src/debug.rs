use core::arch::asm;

#[derive(Debug)]
pub enum DebugOprationMode {
    RealMode,
    ProtectedMode,
    LongMode,
}

pub fn operating_mode() -> DebugOprationMode {
    let mut cr0: u64;

    unsafe {
        asm!(
            "mov {0}, cr0",
            out(reg) cr0
        );
    }

    if cr0 & 1 != 0 {
        return DebugOprationMode::ProtectedMode;
    }

    let mut efer: u64;
    unsafe {
        asm!(
            "mov {0}, 0xC0000080",
            out(reg) efer
        );
    }

    if efer & 1 != 0 {
        return DebugOprationMode::LongMode;
    }

    return DebugOprationMode::RealMode;
}
