use core::arch::asm;

pub unsafe fn vmrun(vmcb_address: usize) {
    asm!(
        "mov rax, {0}",
        "vmrun",
        in(reg) vmcb_address
    );
}

pub unsafe fn vmexit() {
    asm!("vmexit");
}

pub fn read_msr(msr: u32) -> u64 {
    let mut low: u32;
    let mut high: u32;

    unsafe {
        asm!(
            "rdmsr",
            in("ecx") msr,
            out("eax") low,
            out("edx") high,
        );
    }

    ((high as u64) << 32) | (low as u64)
}

pub fn enable_svm() {
    let mut low: u32;
    let mut high: u32;

    unsafe {
        asm!(
            "rdmsr",
            in("ecx") 0xC0000080u32,
            out("eax") low,
            out("edx") high,
        );
    }

    low |= 1 << 12;

    unsafe {
        asm!(
            "wrmsr",
            in("ecx") 0xC0000080u32,
            in("eax") low,
            in("edx") high,
        );
    }
}
