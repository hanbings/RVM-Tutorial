use core::arch::asm;

pub unsafe fn vmload(vmcb_address: usize) {
    asm!(
        "vmload",
        in("rax") vmcb_address
    );
}

pub unsafe fn vmrun(vmcb_address: usize) {
    asm!(
        "vmrun",
        in("rax") vmcb_address
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

pub fn is_enabled_svm() -> bool {
    read_msr(0xC0000080) & 1 << 12 != 0
}

pub unsafe fn enable_svm() {
    let mut low: u32;
    let mut high: u32;

    asm!(
        "rdmsr",
        in("ecx") 0xC0000080u32,
        out("eax") low,
        out("edx") high,
    );

    low |= 1 << 12;

    asm!(
        "wrmsr",
        in("ecx") 0xC0000080u32,
        in("eax") low,
        in("edx") high,
    );
}

pub unsafe fn disable_svm() {
    let mut low: u32;
    let mut high: u32;

    asm!(
        "rdmsr",
        in("ecx") 0xC0000080u32,
        out("eax") low,
        out("edx") high,
    );

    low &= !(1 << 12);

    asm!(
        "wrmsr",
        in("ecx") 0xC0000080u32,
        in("eax") low,
        in("edx") high,
    );
}
