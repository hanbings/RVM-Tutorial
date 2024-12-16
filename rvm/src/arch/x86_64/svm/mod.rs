use core::arch::asm;
use core::marker::PhantomData;

use raw_cpuid::CpuId;
use svm::read_msr;
use vmcb::Vmcb;
use x86::vmx::VmFail;
use x86_64::registers::control::{Cr4, Cr4Flags};

use crate::mm::PhysFrame;
use crate::{HostPhysAddr, RvmError, RvmHal, RvmResult};

pub mod svm;
pub mod vmcb;

pub use self::SvmPreCpuState as ArchPerCpuState;

#[allow(dead_code)]
pub fn has_hardware_support() -> bool {
    CpuId::new().get_svm_info().is_some()
}

pub struct SvmPreCpuState<H: RvmHal> {
    vmcb: Vmcb,
    hal: PhantomData<H>,
}

impl<H: RvmHal> SvmPreCpuState<H> {
    pub fn new() -> Self {
        let frame: PhysFrame<H> = PhysFrame::alloc_zero().unwrap();
        unsafe {
            let ptr = frame.as_mut_ptr() as *mut u8;
            let vmcb_ptr = ptr as *mut Vmcb;

            Self {
                vmcb: *vmcb_ptr,
                hal: PhantomData,
            }
        }
    }

    pub fn vmcb(&self) -> &Vmcb {
        &self.vmcb
    }

    pub fn vmcb_mut(&mut self) -> &mut Vmcb {
        &mut self.vmcb
    }

    pub fn phys_addr(&self) -> HostPhysAddr {
        self.vmcb() as *const Vmcb as HostPhysAddr
    }

    pub fn is_enabled(&self) -> bool {
        Cr4::read().contains(Cr4Flags::VIRTUAL_MACHINE_EXTENSIONS)
    }

    pub fn hardware_enable(&mut self) -> RvmResult {
        if !has_hardware_support() {
            return rvm_err!(Unsupported, "CPU does not support feature SVM");
        }

        if self.is_enabled() {
            return rvm_err!(ResourceBusy, "SVM is already turned on");
        }

        // Check if the CPU supports SVM
        let cpuid = x86::cpuid::CpuIdResult::from(x86::cpuid::cpuid!(0x80000001));
        let ecx = cpuid.ecx;
        let svm = cpuid.ecx & (1 << 2);

        info!("[RVM] CPUID: {:#x}, SVM Feature: {:#?}", ecx, svm != 0);
        if svm == 0 {
            return rvm_err!(Unsupported, "CPU does not support feature SVM");
        }

        // Check if the BIOS enabled SVM
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

        let vmcb_address = self.phys_addr();

        fn get_cpl() -> u32 {
            let mut cpl: u32;
            unsafe {
                asm!("mov {}, cs", out(reg) cpl);
            }
            cpl & 3
        }

        // Enable SVM
        unsafe {
            info!("[RVM] Enabling SVM...");
            svm::enable_svm();
            info!("[RVM] CPL value: {:#x}", get_cpl());
            info!("[RVM] Ready to turn on SVM.");
            svm::vmrun(vmcb_address);
        };

        info!("[RVM] successed to turn on SVM.");

        Ok(())
    }

    pub fn hardware_disable(&mut self) -> RvmResult {
        Ok(())
    }
}

impl From<VmFail> for RvmError {
    fn from(err: VmFail) -> Self {
        rvm_err_type!(BadState, format_args!("VMX instruction failed: {:?}", err))
    }
}
