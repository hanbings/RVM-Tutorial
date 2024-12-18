use bilge::prelude::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Vmcb {
    pub control_area: VmcbControlArea,
    pub state_save_area: VmcbStateSaveArea,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct VmcbControlArea {
    // intercept vector 0
    pub intercept_cr_read: u16,
    pub intercept_cr_write: u16,
    // intercept vector 1
    pub intercept_dr_read: u16,
    pub intercept_dr_write: u16,
    // intercept vector 2
    pub intercept_exception: u32,
    // intercept vector 3
    pub intercept_vector3: VmcbInterceptVector3,
    // intercept vector 4
    pub intercept_vector4: VmcbInterceptVector4,
    // intercept vector 5
    pub intercept_vector5: VmcbInterceptVector5,

    // reserved 0x18 - 0x3b
    pub reserved_0x18_0x3b: [u8; 0x3b - 0x18],

    // PAUSE Filter Threshold
    pub pause_filter_threshold: u16,
    // PAUSE Filter Count
    pub pause_filter_count: u16,

    // Physical base address, bits 11:0 are ignored
    // Physical base address of IOPM
    pub iopm_base_pa: u64,
    pub msrpm_base_pa: u64,
    pub tsc_offset: u64,

    // Guest state
    pub guest: VmcbGuest,
    pub vm_interrupt_control_flags: VmInterruptControlFlags,
    pub vm_shadow_interrupt: VmShadowInterrupt,
    pub exit_code: u64,
    pub exit_info_1: u64,
    pub exit_info_2: u64,
    pub exit_init_info: u64,
    pub nested_paging_flags: NestedPagingFlags,
    pub avic_control_flags: AvicControlFlags,

    // Guest Physical Address of GHCB
    pub ghcb_pa: u64,
    // EVENTINJ—Event injection
    pub event_injection: u64,
    // nested paging
    pub n_cr3: u64,

    // LBR Virtualization Flags
    pub lbr_virtualization_flags: LbrVirtualizationFlags,

    // Clean Bits
    pub clean_bits: VmcbCleanBits,

    // Next sequential instruction pointer
    pub n_rip: u64,

    // Instruction Fetch Count
    pub instruction_fetch_count: InstructionFetchCount,

    // AVIC APIC_BACKING_PAGE Pointer
    pub avic_backing_page_pointer: AvicBackingPagePointer,

    // reserved 0xef - 0xe8
    pub reserved_0xef_0xe8: [u8; 0xef - 0xe8],

    // AVIC Logical Backing Page Pointer
    pub avic_logical_backing_page_pointer: AvicBackingPagePointer,
    // AVIC Physical Backing Page Pointer
    pub avic_physical_backing_page_pointer: AvicBackingPagePointer,

    // reserved 0x100 - 0x107
    pub reserved_0x100_0x107: [u8; 0x107 - 0x100],

    // VMSA Pointer
    pub vmsa_pointer: VmcbStateSaveAreaPointer,

    pub vmgexit_rax: u64,
    pub vmgrexit_cpl: u8,

    // bus lock threshold counter
    pub bus_lock_threshold_counter: u16,

    // reserved 0x133 - 0x128
    pub reserved_0x138_0x128: [u8; 0x133 - 0x128],

    // update irr ??? len = 0
    pub update_irr: [u8; 0x138 - 0x133],

    // SEV
    pub sev_features_mask: SevFeaturesMask,
    pub guest_sev_features: GuestSevFeatures,

    // reserved 0x148 - 0x150
    pub reserved_0x148_0x150: [u8; 0x150 - 0x148],

    // reserved 0x170 - 0x150
    pub requested_irr: [u8; 0x170 - 0x150],

    // reserved 0x3df - 0x170
    pub reserved_0x120_0x3df: [u8; 0x3df - 0x170],

    // reserved for host usage
    pub reserved_0x3e0_0x3ff: [u8; 0x3ff - 0x3e0],
}

impl VmcbControlArea {
    pub fn tlb_control_description(&self) -> &'static str {
        match self.guest.tlb_control() {
            0x00 => "Do nothing",
            0x01 => "Flush entire TLB (all entries, all ASIDs) on VMRUN",
            0x03 => "Flush this guest's TLB entries",
            0x07 => "Flush this guest's non-global TLB entries",
            _ => "Reserved or Invalid encoding",
        }
    }
}

#[repr(C)]
#[bitsize(32)]
#[derive(Clone, Copy, PartialEq, FromBits)]
pub struct VmcbInterceptVector3 {
    pub intercept_intr: u1,
    pub intercept_nmi: u1,
    pub intercept_smi: u1,
    pub intercept_init: u1,
    pub intercept_vintr: u1,
    // intercept CR0 writes that change bits other than CR0.TS or CR0.MP
    pub intercept_cr0_writes: u1,

    // descriptor tables intercept
    pub intercept_idtr_reads: u1,
    pub intercept_gdtr_reads: u1,
    pub intercept_ldtr_reads: u1,
    pub intercept_tr_reads: u1,
    pub intercept_idtr_writes: u1,
    pub intercept_gdtr_writes: u1,
    pub intercept_ldtr_writes: u1,
    pub intercept_tr_writes: u1,

    // intercept instruction
    pub intercept_rdtsc: u1,
    pub intercept_rdpmc: u1,
    pub intercept_pushf: u1,
    pub intercept_popf: u1,
    pub intercept_cpuid: u1,
    pub intercept_rsm: u1,
    pub intercept_iret: u1,
    pub intercept_intn: u1,
    pub intercept_invd: u1,
    pub intercept_pause: u1,
    pub intercept_hlt: u1,
    pub intercept_invlpg: u1,
    pub intercept_invlpga: u1,
    pub intercept_ioio_prot: u1,
    pub intercept_msr_prot: u1,
    pub intercept_task_switch: u1,

    // intercept processor "freezing" during legacy FERR handling
    pub intercept_ferr_freeze: u1,
    pub intercept_shutdown_event: u1,
}

#[repr(C)]
#[bitsize(32)]
#[derive(Clone, Copy, PartialEq, FromBits)]
pub struct VmcbInterceptVector4 {
    pub intercept_vmrun: u1,
    pub intercept_vmmcall: u1,
    pub intercept_vmload: u1,
    pub intercept_vmsave: u1,
    pub intercept_stgi: u1,
    pub intercept_clgi: u1,
    pub intercept_skinit: u1,
    pub intercept_rdtscp: u1,
    pub intercept_icebp: u1,
    // WBINVD and WBNOINVD
    pub intercept_wbinvd_wbnoinvd: u1,
    // MONITOR / MONITORX
    pub intercept_monitor: u1,
    // MWAIT / MWAITX instruction unconditional
    pub intercept_mwait: u1,
    // MWAIT / MWAITX instruction if monitor hardware is armed
    pub intercept_mwait_hw_armed: u1,
    pub intercept_xsetbv: u1,
    pub intercept_rdpru: u1,
    pub intercept_efer_writes: u1,
    pub intercept_cr0: u1,
    pub intercept_cr1: u1,
    pub intercept_cr2: u1,
    pub intercept_cr3: u1,
    pub intercept_cr4: u1,
    pub intercept_cr5: u1,
    pub intercept_cr6: u1,
    pub intercept_cr7: u1,
    pub intercept_dr8: u1,
    pub intercept_dr9: u1,
    pub intercept_dr10: u1,
    pub intercept_dr11: u1,
    pub intercept_dr12: u1,
    pub intercept_dr13: u1,
    pub intercept_dr14: u1,
    pub intercept_dr15: u1,
}

#[repr(C)]
#[bitsize(32)]
#[derive(Clone, Copy, PartialEq, FromBits)]
pub struct VmcbInterceptVector5 {
    pub intercept_invlpgb: u1,
    pub intercept_illegal_invlpgb: u1,
    pub intercept_invpcid: u1,
    pub intercept_mcommit: u1,
    // Intercept TLB invalidation. Presence of this bit
    // is indicated by CPUID Fn8000_000A, EDX[24] = 1
    pub intercept_tlbsync: u1,
    // Intercept HLT instruction if a virtual interrupt is not pending
    pub intercept_hlt: u1,
    pub reserve: u26,
}

#[repr(C)]
#[bitsize(64)]
#[derive(Clone, Copy, PartialEq, FromBits)]
pub struct VmcbGuest {
    // Guest Address Space Identifier (Guest ASID)
    pub guest_asid: u32,
    // TLB Control (bits 39:32)
    // todo: Parse TLB Control
    pub tlb_control: u8,
    // Reserved bits (bits 63:40)
    pub reserved: u24,
}

#[repr(C)]
#[bitsize(64)]
#[derive(Clone, Copy, PartialEq, FromBits)]
pub struct VmInterruptControlFlags {
    // Virtual TPR (bits 7:0)
    // 4-bit virtual TPR value in bits 3:0
    // bits 7:4 are reserved (SBZ)
    pub v_tpr: u4,
    pub reserved_tpr: u4,

    // V_IRQ (bit 8)
    // Nonzero if virtual INTR is pending
    pub v_irq: u1,

    // VGIF (bit 9)
    // Virtual interrupts are masked/unmasked (0: masked, 1: unmasked)
    pub vgif: u1,

    // V_NMI (bit 11)
    // Nonzero if virtual NMI is pending
    pub v_nmi: u1,

    // V_NMI_MASK (bit 12)
    // Nonzero if virtual NMI is masked
    pub v_nmi_mask: u1,

    // Reserved bits 13:15
    // Reserved (SBZ)
    pub reserved_nmi: u3,

    // V_INTR_PRIO (bits 19:16)
    // 4-bit priority for virtual interrupt
    pub v_intr_prio: u4,

    // V_IGN_TPR (bit 20)
    // Nonzero if the current virtual interrupt ignores the (virtual) TPR
    pub v_ign_tpr: u1,

    // Reserved bits 21:23
    // Reserved (SBZ)
    pub reserved_intr: u3,

    // V_INTR_MASKING (bit 24)
    // Virtualize masking of INTR interrupts
    pub v_intr_masking: u1,

    // AMD Virtual GIF enabled (bit 25)
    // Nonzero if AMD Virtual GIF is enabled for this guest
    pub v_gif_enabled: u1,

    // V_NMI_ENABLE (bit 26)
    // Nonzero if NMI virtualization is enabled
    pub v_nmi_enable: u1,

    // Reserved bits 27:29
    // Reserved (SBZ)
    pub reserved_nmi_enable: u3,

    // x2AVIC Enable (bit 30)
    // Nonzero if x2AVIC is enabled
    pub x2avic_enable: u1,

    // AVIC Enable (bit 31)
    // Nonzero if AVIC is enabled
    pub avic_enable: u1,

    // V_INTR_VECTOR (bits 39:32)
    // 8-bit vector to use for this interrupt
    pub v_intr_vector: u8,

    // Reserved bits 40:63
    // Reserved (SBZ)
    pub reserved: u25,
}

#[repr(C)]
#[bitsize(64)]
#[derive(Clone, Copy, PartialEq, FromBits)]
pub struct VmShadowInterrupt {
    // INTERRUPT_SHADOW (bit 0)
    // Guest is in an interrupt shadow (1: true, 0: false)
    pub interrupt_shadow: u1,

    // GUEST_INTERRUPT_MASK (bit 1)
    // Value of RFLAGS.IF for the guest
    pub guest_interrupt_mask: u1,

    // Reserved bits (bits 63:2)
    // 62 bits reserved (SBZ)
    pub reserved: u62,
}

#[repr(C)]
#[bitsize(64)]
#[derive(Clone, Copy, PartialEq, FromBits)]
pub struct NestedPagingFlags {
    /// Enable nested paging
    pub np_enable: u1,

    /// Enable Secure Encrypted Virtualization (SEV)
    pub enable_sev: u1,

    /// Enable encrypted state for Secure Encrypted Virtualization (SEV)
    pub enable_encrypted_state: u1,

    /// Guest Mode Execute Trap
    pub guest_mode_execute_trap: u1,

    /// Enable supervisor shadow stack restrictions in nested page tables.
    /// Support for this feature is indicated by CPUID Fn8000_000A_EDX[19] (SSSCheck).
    pub sss_check_en: u1,

    /// Virtual Transparent Encryption
    pub virtual_transparent_encryption: u1,

    /// Enable Read Only Guest Page Tables.
    /// See "Nested Table Walk" for more information.
    pub enable_read_only_guest_page_tables: u1,

    /// Enable INVLPGB/TLBSYNC.
    ///
    /// 0 - INVLPGB and TLBSYNC will result in #UD.
    /// 1 - INVLPGB and TLBSYNC can be executed in guest.
    ///
    /// Presence of this bit is indicated by CPUID bit 8000_000A, EDX[24] = 1.
    /// When in SEV-ES guest or this bit is not present, INVLPGB/TLBSYNC is always enabled in guest
    pub enable_inv_lpgb_tlbsync: u1,

    /// Reserved (SBZ)
    pub reserved: u56,
}

#[repr(C)]
#[bitsize(64)]
#[derive(Clone, Copy, PartialEq, FromBits)]
pub struct AvicControlFlags {
    /// Reserved (SBZ)
    /// Bits 63:52 are reserved (SBZ)
    pub reserved: u12,

    /// AVIC APIC_BAR
    ///
    /// Address of the APIC base for the AVIC (AMD Virtual APIC).
    /// The APIC_BAR is used by AVIC to access the APIC and other
    /// related functionality within the virtualized environment.
    ///
    /// Bits 51:0 are for AVIC APIC_BAR
    pub apic_bar: u52,
}

#[repr(C)]
#[bitsize(64)]
#[derive(Clone, Copy, PartialEq, FromBits)]
pub struct LbrVirtualizationFlags {
    /// LBR Virtualization Enable
    ///
    /// 0 — Do nothing.
    /// 1 — Enable LBR virtualization hardware acceleration.
    pub lbr_virtualization_enable: u1,

    /// Virtualized VMSAVE/VMLOAD Enable
    ///
    /// Enables virtualized versions of VMSAVE and VMLOAD instructions.
    pub virtualized_vmsave_vmload_enable: u1,

    /// Virtualized Instruction-Based Sampling Enable
    ///
    /// Enables virtualized instruction-based sampling.
    /// See "Instruction-Based Sampling Virtualization" for details.
    pub virtualized_instruction_sampling_enable: u1,

    /// Reserved (SBZ)
    /// Reserved bits (63:3)
    pub reserved: u61,
}

#[repr(C)]
#[bitsize(64)]
#[derive(Clone, Copy, PartialEq, FromBits)]
pub struct VmcbCleanBits {
    /// VMCB Clean Bits
    ///
    /// Represents the clean bits in the VMCB.
    /// These bits are used to track the state of the VMCB, indicating which fields
    /// have been modified or need to be written back to memory.
    /// Bits 31:0 for VMCB Clean Bits
    pub vmcb_clean_bits: u32,

    /// Reserved (SBZ)
    /// Bits 63:32 are reserved (SBZ)
    pub reserved: u32,
}

#[repr(C)]
#[bitsize(128)]
#[derive(Clone, Copy, PartialEq, FromBits)]
pub struct InstructionFetchCount {
    // Number of bytes fetched
    pub number_of_bytes_fetched: u8,
    // Guest instruction bytes
    pub guest_instruction_bytes: u120,
}

#[repr(C)]
#[bitsize(64)]
#[derive(Clone, Copy, PartialEq, FromBits)]
pub struct AvicBackingPagePointer {
    /// AVIC APIC_BACKING_PAGE Pointer
    ///
    /// This field holds the pointer to the APIC backing page for AVIC (AMD Virtual APIC).
    /// It points to the structure used by AVIC to emulate the APIC and related functionality
    /// in a virtualized environment.
    /// Bits 51:0 for AVIC APIC_BACKING_PAGE pointer
    pub apic_backing_page_pointer: u52,
    pub apic_backing_page_pointer_reserved: u12,
}

#[repr(C)]
#[bitsize(64)]
#[derive(Clone, Copy, PartialEq, FromBits)]
pub struct AvicLogicalTablePointer {
    pub avic_logical_table_pointer_reserved_1: u12,
    pub avic_logical_table_pointer: u40,
    pub avic_logical_table_pointer_reserved_2: u12,
}

#[repr(C)]
#[bitsize(64)]
#[derive(Clone, Copy, PartialEq, FromBits)]
pub struct AvicPhysicalTablePointer {
    pub avic_physical_table_pointer_reserved_1: u8,
    pub avic_physical_table_pointer: u40,
    pub avic_physical_table_pointer_reserved_2: u8,
    pub avic_physical_max_index: u8,
}

#[repr(C)]
#[bitsize(64)]
#[derive(Clone, Copy, PartialEq, FromBits)]
pub struct SevFeaturesMask {
    pub allowed_sev_features_mask: u62,
    pub reserved: u1,
    pub allowed_sev_features_en: u1,
}

#[repr(C)]
#[bitsize(64)]
#[derive(Clone, Copy, PartialEq, FromBits)]
pub struct GuestSevFeatures {
    pub guest_sev_features: u62,
    pub reserved: u2,
}

#[repr(C)]
#[bitsize(64)]
#[derive(Clone, Copy, PartialEq, FromBits)]
pub struct VmcbStateSaveAreaPointer {
    pub vmsa_pointer_reserved_1: u12,
    pub vmsa_pointer: u40,
    pub vmsa_pointer_reserved_2: u12,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct VmcbStateSaveArea {
    // registers
    pub es: VmcbRegister,
    pub cs: VmcbRegister,
    pub ss: VmcbRegister,
    pub ds: VmcbRegister,
    pub fs: VmcbRegister,
    pub gs: VmcbRegister,
    pub gdtr: VmcbRegister,
    pub ldtr: VmcbRegister,
    pub idtr: VmcbRegister,
    pub tr: VmcbRegister,
    // reserved 0xa0 - 0xca
    pub reserved_0xa0_0xca: [u8; 0xca - 0xa0],
    // If the guest is real-mode then the CPL is forced to 0;
    // if the guest is virtual-mode then the CPL is forced to 3.
    pub cpl: u8,
    // reserved 0xcc
    pub reserved_0xcc: u32,
    pub efer: u64,
    // reserved 0xd8 - 0x147
    pub reserved_0xd8_0xdf: [u8; 0xdf - 0xd8],
    // perf control
    pub perf_ctl0: u64,
    pub perf_ctr0: u64,
    pub perf_ctl1: u64,
    pub perf_ctr1: u64,
    pub perf_ctl2: u64,
    pub perf_ctr2: u64,
    pub perf_ctl3: u64,
    pub perf_ctr3: u64,
    pub perf_ctl4: u64,
    pub perf_ctr4: u64,
    pub perf_ctl5: u64,
    pub perf_ctr5: u64,
    // control registers and data registers
    pub cr4: u64,
    pub cr3: u64,
    pub cr0: u64,
    pub dr7: u64,
    pub dr6: u64,
    pub rfalgs: u64,
    pub rip: u64,
    // reserved 0x180 - 0x1bf
    pub reserved_0x180_0x1bf: [u8; 0x1bf - 0x180],
    pub instr_retired_ctr: u64,
    pub perf_ctr_global_sts: u64,
    pub perf_ctr_global_ctl: u64,
    // reserved 0x1d4 - 1d7
    pub reserved_0x1d4_0x1d7: [u8; 0x1d7 - 0x1d4],
    pub rsp: u64,
    pub s_cet: u64,
    pub ssp: u64,
    pub isst_addr: u64,
    pub rax: u64,
    pub star: u64,
    pub lstar: u64,
    pub cstar: u64,
    pub sfmask: u64,
    pub kernel_gs_base: u64,
    pub sysenter_cs: u64,
    pub sysenter_esp: u64,
    pub sysenter_eip: u64,
    pub cr2: u64,
    // reserved 248h–267h
    pub reserved_0x248_0x267: [u8; 0x267 - 0x248],
    pub g_pat: u64,
    pub dbgctl: u64,
    pub br_from: u64,
    pub br_to: u64,
    pub lastexcpfrom: u64,
    pub lastexcpto: u64,
    pub dbgextnctl: u64,
    // reserved 2a0h–2dfh
    pub reserved_0x2a0_0x2df: [u8; 0x2df - 0x2a0],
    pub spec_ctrl: u64,
    // reserved 2e8h–66fh
    pub reserved_0x2e8_0x66f: [u8; 0x66f - 0x2e8],
    // 670h - 76fh
    pub lbr_stack: [u8; 256],
    pub lbr_select: u64,
    pub ibs_fetch_ctl: u64,
    pub ibs_fetch_linaddr: u64,
    pub ibs_op_ctl: u64,
    pub ibs_op_rip: u64,
    pub ibs_op_data: u64,
    pub ibs_op_data2: u64,
    pub ibs_op_data3: u64,
    pub ibs_dc_linaddr: u64,
    pub bp_ibstgt_rip: u64,
    pub ic_ibst_extd_ctl: u64,
}

#[repr(C)]
#[bitsize(128)]
#[derive(Clone, Copy, PartialEq, FromBits)]
pub struct VmcbRegister {
    pub selector: u16,
    pub attrib: u16,
    pub limit: u32,
    pub base: u64,
}
