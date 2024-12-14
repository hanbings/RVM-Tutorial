pub(crate) mod msr;

cfg_if::cfg_if! {
    if #[cfg(feature = "vmx")] {
        mod vmx;
        use vmx as vender;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "svm")] {
        mod svm;
        use svm as vender;
    }
}

pub use vender::{has_hardware_support, ArchPerCpuState};
