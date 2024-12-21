use svm::read_msr;
use x86::vmx::VmFail;

use crate::{debug, RvmError, RvmHal, RvmResult};

pub mod definitions;
pub mod structs;
pub mod svm;
pub mod vcpu;
pub mod vmcb;

pub use self::definitions::SvmExitInfo;
pub use self::definitions::SvmExitReason;
pub use self::vcpu::SvmVcpu as RvmVcpu;
pub use self::SvmPreCpuState as ArchPerCpuState;

#[allow(dead_code)]
pub fn has_hardware_support() -> bool {
    let cpuid = x86::cpuid::CpuIdResult::from(x86::cpuid::cpuid!(0x80000001));
    let ecx = cpuid.ecx;
    let svm = ecx & (1 << 2);

    svm != 0
}

pub struct SvmPreCpuState<H: RvmHal> {
    _phantom: core::marker::PhantomData<H>,
}

impl<H: RvmHal> SvmPreCpuState<H> {
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }

    pub fn is_enabled(&self) -> bool {
        // Cr4::read().contains(Cr4Flags::VIRTUAL_MACHINE_EXTENSIONS)
        svm::is_enabled_svm()
    }

    pub fn hardware_enable(&mut self) -> RvmResult {
        debug!("[RVM] Running in {:?}", debug::operating_mode());

        // Check if the CPU supports SVM
        if !has_hardware_support() {
            return rvm_err!(Unsupported, "CPU does not support feature SVM");
        }

        // Check if the BIOS enabled SVM
        if self.is_enabled() {
            return rvm_err!(ResourceBusy, "SVM is already turned on");
        }

        let msr_value = read_msr(0xC0010114);
        let svm_disabled = msr_value & (1 << 4);

        info!(
            "[RVM] VM_CR register value: {:#x}, SVM Enabled: {:?}",
            msr_value,
            svm_disabled == 0
        );
        if svm_disabled != 0 {
            return rvm_err!(ResourceBusy, "SVM is not enabled in VM_CR register");
        }

        // Enable SVM
        unsafe {
            info!("[RVM] Enabling SVM...");
            svm::enable_svm();
        };

        info!("[RVM] successed to turn on SVM.");
        Ok(())
    }

    pub fn hardware_disable(&mut self) -> RvmResult {
        unsafe {
            info!("[RVM] Disabling SVM...");
            svm::disable_svm();
        }

        info!("[RVM] successed to turn off SVM.");
        Ok(())
    }
}

impl From<VmFail> for RvmError {
    fn from(err: VmFail) -> Self {
        rvm_err_type!(BadState, format_args!("VMX instruction failed: {:?}", err))
    }
}
