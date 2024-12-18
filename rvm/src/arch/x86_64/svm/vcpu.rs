use core::fmt::{Debug, Formatter, Result};

use crate::arch::x86_64::svm::svm;
use crate::arch::x86_64::svm::vmcb::{Vmcb, VmcbControlArea, VmcbStateSaveArea};
use crate::{RvmHal, RvmResult};

use super::structs::SvmRegion;
use super::SvmPreCpuState;

#[repr(C)]
pub struct SvmVcpu<H: RvmHal> {
    vmcb: SvmRegion<H>,
}

impl<H: RvmHal> SvmVcpu<H> {
    pub(crate) fn new(_percpu: &SvmPreCpuState<H>) -> RvmResult<Self> {
        let vcpu = Self {
            vmcb: SvmRegion::new()?,
        };

        info!("[RVM] created SvmVcpu(vmcb: {:#x})", vcpu.vmcb.phys_addr());
        Ok(vcpu)
    }

    pub fn run(&mut self) {
        debug!("[RVM] VMCB Size: {:#x}", core::mem::size_of::<Vmcb>());
        debug!(
            "[RVM] VMCA Size: {:#x}",
            core::mem::size_of::<VmcbControlArea>()
        );
        debug!(
            "[RVM] VMAS Size: {:#x}",
            core::mem::size_of::<VmcbStateSaveArea>()
        );

        let vmcb_address = self.vmcb.phys_addr();
        info!("[RVM] VMCB Address: {:#x}", vmcb_address);

        unsafe {
            info!("[RVM] Ready to switch to Guest...");
            svm::vmrun(vmcb_address);
        };
    }
}

impl<H: RvmHal> Debug for SvmVcpu<H> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        (|| -> RvmResult<Result> { Ok(f.debug_struct("SvmVcpu").finish()) })().unwrap()
    }
}
