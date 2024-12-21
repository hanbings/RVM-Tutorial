pub(crate) mod msr;

#[macro_use]
pub(crate) mod regs;

cfg_if::cfg_if! {
    if #[cfg(feature = "vmx")] {
        mod vmx;
        use vmx as vender;
        pub use vmx::{VmxExitInfo, VmxExitReason};
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "svm")] {
        mod svm;
        use svm as vender;
        pub use svm::{SvmExitInfo, SvmExitReason};
    }
}

pub use regs::GeneralRegisters;
pub use vender::{has_hardware_support, ArchPerCpuState, RvmVcpu};
