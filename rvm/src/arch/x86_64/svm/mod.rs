use core::marker::PhantomData;

use raw_cpuid::CpuId;
use vmcb::Vmcb;
use x86::vmx::VmFail;

use crate::mm::PhysFrame;
use crate::{HostPhysAddr, RvmError, RvmHal, RvmResult};

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
        CpuId::new().get_svm_info().is_some()
    }

    pub fn hardware_enable(&mut self) -> RvmResult {
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
