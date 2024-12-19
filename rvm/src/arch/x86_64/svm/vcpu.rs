use core::fmt::{Debug, Formatter, Result};

use bilge::prelude::u52;
use x86_64::registers::control::Cr0Flags;

use crate::arch::msr::Msr;
use crate::arch::x86_64::svm::svm;
use crate::arch::x86_64::svm::vmcb::{Vmcb, VmcbControlArea, VmcbStateSaveArea};
use crate::mm::PhysFrame;
use crate::{RvmHal, RvmResult};

use super::structs::SvmRegion;
use super::SvmPreCpuState;

#[repr(C)]
pub struct SvmVcpu<H: RvmHal> {
    vmcb: SvmRegion<H>,
    host_state: PhysFrame<H>,
}

impl<H: RvmHal> SvmVcpu<H> {
    pub(crate) fn new(_percpu: &SvmPreCpuState<H>) -> RvmResult<Self> {
        let mut vcpu = Self {
            vmcb: SvmRegion::new()?,
            host_state: PhysFrame::alloc_zero()?,
        };

        vcpu.setup_vmcb();
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
            info!("[RVM] Setting VM_HSAVE_PA MSR...");
            let vm_hsave_pa_msr = self.host_state.start_paddr();
            debug!("[RVM] VM_HSAVE_PA MSR: {:#x}", vm_hsave_pa_msr);
            svm::set_vm_hsave_pa_msr(vm_hsave_pa_msr);

            info!("[RVM] Ready to switch to Guest...");
            svm::vmload(vmcb_address);
            svm::vmrun(vmcb_address);
        };
    }
}

impl<H: RvmHal> SvmVcpu<H> {
    fn setup_vmcb(&mut self) {
        let vmcb_address = self.vmcb.phys_addr();
        let vmcb = unsafe { &mut *(vmcb_address as *mut Vmcb) };

        self.setup_vmcb_control_area(vmcb);
        self.setup_vmcb_state_save_area(vmcb);
    }
    fn setup_vmcb_control_area(&mut self, vmcb: &mut Vmcb) {
        vmcb.control_area
            .avic_control_flags
            .set_apic_bar(u52::new(0x1));
    }
    fn setup_vmcb_state_save_area(&mut self, vmcb: &mut Vmcb) {
        vmcb.state_save_area.cr0 = (Cr0Flags::EXTENSION_TYPE | Cr0Flags::NUMERIC_ERROR).bits();

        [
            vmcb.state_save_area.es,
            vmcb.state_save_area.cs,
            vmcb.state_save_area.ss,
            vmcb.state_save_area.ds,
            vmcb.state_save_area.fs,
            vmcb.state_save_area.gs,
        ]
        .iter_mut()
        .for_each(|segment| {
            segment.set_base(0x93);
            segment.set_limit(0xffff);
            segment.set_attrib(0xffff);
            segment.set_selector(0xffff);
        });

        vmcb.state_save_area.tr.set_base(0x8b);
        vmcb.state_save_area.ldtr.set_base(0x83);

        vmcb.state_save_area.gdtr.set_limit(0xffff);
        vmcb.state_save_area.idtr.set_limit(0xffff);

        vmcb.state_save_area.cr3 = 0;
        vmcb.state_save_area.dr7 = 0x400;
        vmcb.state_save_area.rsp = 0;
        vmcb.state_save_area.rip = 0;
        vmcb.state_save_area.rfalgs = 0x2;
        vmcb.state_save_area.dbgextnctl = 0;

        vmcb.state_save_area.sysenter_esp = 0;
        vmcb.state_save_area.sysenter_eip = 0;
        vmcb.state_save_area.sysenter_cs = 0;

        vmcb.state_save_area.g_pat = Msr::IA32_PAT.read() as u64;
        vmcb.state_save_area.efer = 0;
    }
}

impl<H: RvmHal> Debug for SvmVcpu<H> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        (|| -> RvmResult<Result> { Ok(f.debug_struct("SvmVcpu").finish()) })().unwrap()
    }
}
