#![allow(unused)]

use modular_bitfield::prelude::{B1, B12, B120, B128, B16, B23, B24, B25, B27, B3, B32, B4, B40, B52, B56, B61, B62, B64, B8};

#[modular_bitfield::bitfield]
#[derive(Debug, Clone, Copy)]
pub struct VmcbControlArea {
    // intercept vector 0
    intercept_cr_read: B16,
    intercept_cr_write: B16,
    // intercept vector 1
    intercept_dr_read: B16,
    intercept_dr_write: B16,

    // intercept vector 2
    intercept_exception: B16,

    // intercept vector 3
    #[bits = 32]
    intercept_vector3: VmcbInterceptVector3,

    // intercept vector 4
    #[bits = 32]
    intercept_vector4: VmcbInterceptVector4,

    // intercept vector 5
    #[bits = 32]
    intercept_vector5: VmcbInterceptVector5,

    // reserved 0x18 - 0x3b
    reserved_0x18: B128,
    reserved_0x3c: B8,

    // PAUSE Filter Threshold
    pause_filter_threshold: B16,
    // PAUSE Filter Count
    pause_filter_count: B16,

    // Physical base address, bits 11:0 are ignored
    // Physical base address of IOPM
    iopm_base_pa: B64,
    msrpm_base_pa: B64,
    tsc_offset: B64,

    // Guest Address Space Identifier (Guest ASID)
    guest_asid: B32,
    // TLB Control (bits 39:32)
    tlb_control: B8,
    // Reserved bits (bits 63:40)
    reserved: B24,

    // Vm Interrupt Control Flags
    #[bits = 64]
    vm_interrupt_control_flags: VmInterruptControlFlags,

    // Vm Shadow Interrupt
    #[bits = 64]
    vm_shadow_interrupt: VmShadowInterrupt,

    exit_code: B64,
    exit_info_1: B64,
    exit_info_2: B64,
    exit_init_info: B64,

    // Nested Page Table
    #[bits = 64]
    nested_paging_flags: NestedPagingFlags,

    // AVIC Control Flags
    #[bits = 64]
    avic_control_flags: AvicControlFlags,

    // Guest Physical Address of GHCB
    ghcb_pa: B64,

    // EVENTINJ—Event injection
    event_injection: B64,

    // nested paging
    n_cr3: B64,

    // LBR Virtualization Flags
    #[bits = 64]
    lbr_control: LbrVirtualizationFlags,

    // Clean Bits
    clean_bits: VmcbCleanBits,

    // Next sequential instruction pointer
    n_rip: B64,

    // Instruction Fetch Count
    #[bits = 128]
    instruction_fetch_count: InstructionFetchCount,

    /// AVIC APIC_BACKING_PAGE Pointer
    ///
    /// This field holds the pointer to the APIC backing page for AVIC (AMD Virtual APIC).
    /// It points to the structure used by AVIC to emulate the APIC and related functionality
    /// in a virtualized environment.
    /// Bits 51:0 for AVIC APIC_BACKING_PAGE pointer
    apic_backing_page_pointer: B52,
    apic_backing_page_pointer_reserved: B12,

    // reserved 0xef - 0xe8
    reserved_0xef: B8,

    /// AVIC LOGICAL_TABLE Pointer
    avci_logical_table_pointer_reserved_1: B8,
    avci_logical_table_pointer: B40,
    avci_logical_table_pointer_reserved_2: B8,

    /// AVIC PHYSICAL_TABLE Pointer
    avci_physical_table_pointer_reserved_1: B8,
    avci_physical_table_pointer: B40,
    avci_physical_table_pointer_reserved_2: B8,
    avci_physical_max_index: B8,

    // reserved 0x100 - 0x107
    reserved_0x100: B8,

    /// VMSA Pointer
    vmsa_pointer_reserved_1: B8,
    vmsa_pointer: B40,
    vmsa_pointer_reserved_2: B8,

    // all other fields up to 0x3df
    // TODO: RESERVED BUT USED

    // reserved 3e0 - 3ef
    reserved_0x3e0: B16,
}

impl VmcbControlArea {
    pub fn tlb_control_description(&self) -> &'static str {
        match self.tlb_control() {
            0x00 => "Do nothing",
            0x01 => "Flush entire TLB (all entries, all ASIDs) on VMRUN",
            0x03 => "Flush this guest's TLB entries",
            0x07 => "Flush this guest's non-global TLB entries",
            _ => "Reserved or Invalid encoding",
        }
    }
}

#[modular_bitfield::bitfield]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct VmcbInterceptVector3 {
    intercept_intr: B1,
    intercept_nmi: B1,
    intercept_smi: B1,
    intercept_init: B1,
    intercept_vintr: B1,
    // intercept CR0 writes that change bits other than CR0.TS or CR0.MP
    intercept_cr0_writes: B1,
    
    // descriptor tables intercept
    intercept_idtr_reads: B1,
    intercept_gdtr_reads: B1,
    intercept_ldtr_reads: B1,
    intercept_tr_reads: B1,
    intercept_idtr_writes: B1,
    intercept_gdtr_writes: B1,
    intercept_ldtr_writes: B1,
    intercept_tr_writes: B1,
    
    // intercept instruction
    intercept_rdtsc: B1,
    intercept_rdpmc: B1,
    intercept_pushf: B1,
    intercept_popf: B1,
    intercept_cpuid: B1,
    intercept_rsm: B1,
    intercept_iret: B1,
    intercept_intn: B1,
    intercept_invd: B1,
    intercept_pause: B1,
    intercept_hlt: B1,
    intercept_invlpg: B1,
    intercept_invlpga: B1,
    intercept_ioio_prot: B1,
    intercept_msr_prot: B1,
    intercept_task_switch: B1,

    // intercept processor "freezing" during legacy FERR handling
    intercept_ferr_freeze: B1,
    intercept_shutdown_event: B1,
}

#[modular_bitfield::bitfield]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct VmcbInterceptVector4 {
    intercept_vmrun: B1,
    intercept_vmmcall: B1,
    intercept_vmload: B1,
    intercept_vmsave: B1,
    intercept_stgi: B1,
    intercept_clgi: B1,
    intercept_skinit: B1,
    intercept_rdtscp: B1,
    intercept_icebp: B1,
    // WBINVD and WBNOINVD
    intercept_wbinvd_wbnoinvd: B1,
    // MONITOR / MONITORX
    intercept_monitor: B1,
    // MWAIT / MWAITX instruction unconditional
    intercept_mwait: B1,
    // MWAIT / MWAITX instruction if monitor hardware is armed
    intercept_mwait_hw_armed: B1,
    intercept_xsetbv: B1,
    intercept_rdpru: B1,
    intercept_efer_writes: B1,
    intercept_cr0: B1,
    intercept_cr1: B1,
    intercept_cr2: B1,
    intercept_cr3: B1,
    intercept_cr4: B1,
    intercept_cr5: B1,
    intercept_cr6: B1,
    intercept_cr7: B1,
    intercept_dr8: B1,
    intercept_dr9: B1,
    intercept_dr10: B1,
    intercept_dr11: B1,
    intercept_dr12: B1,
    intercept_dr13: B1,
    intercept_dr14: B1,
    intercept_dr15: B1,
}

#[modular_bitfield::bitfield]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct VmcbInterceptVector5 {
    intercept_invlpgb: B1,
    intercept_illegal_invlpgb: B1,
    intercept_invpcid: B1,
    intercept_mcommit: B1,
    // Intercept TLB invalidation. Presence of this bit is indicated by CPUID Fn8000_000A, EDX[24] = 1
    intercept_tlbsync: B1,
    reserve: B27
}

#[modular_bitfield::bitfield]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
// TODO: It seems to be missing here, check it later
// AMD64 Architecture Programmer’s Manual, Volumes 1-5, 40332, 24592, 24593, 24594, 26568, 26569 page 1160
pub struct VmInterruptControlFlags {
    // Virtual TPR (bits 7:0)
    // 4-bit virtual TPR value in bits 3:0
    // bits 7:4 are reserved (SBZ)
    v_tpr: B4,
    reserved_tpr: B4,

    // V_IRQ (bit 8)
    // Nonzero if virtual INTR is pending
    v_irq: B1,

    // VGIF (bit 9)
    // Virtual interrupts are masked/unmasked (0: masked, 1: unmasked)
    vgif: B1,

    // V_NMI (bit 11)
    // Nonzero if virtual NMI is pending
    v_nmi: B1,

    // V_NMI_MASK (bit 12)
    // Nonzero if virtual NMI is masked
    v_nmi_mask: B1,

    // Reserved bits 13:15
    // Reserved (SBZ)
    reserved_nmi: B3,

    // V_INTR_PRIO (bits 19:16)
    // 4-bit priority for virtual interrupt
    v_intr_prio: B4,

    // V_IGN_TPR (bit 20)
    // Nonzero if the current virtual interrupt ignores the (virtual) TPR
    v_ign_tpr: B1,  

    // Reserved bits 21:23
    // Reserved (SBZ)
    reserved_intr: B3,

    // V_INTR_MASKING (bit 24)
    // Virtualize masking of INTR interrupts
    v_intr_masking: B1,  

    // AMD Virtual GIF enabled (bit 25)
    // Nonzero if AMD Virtual GIF is enabled for this guest
    v_gif_enabled: B1,  

    // V_NMI_ENABLE (bit 26)
    // Nonzero if NMI virtualization is enabled
    v_nmi_enable: B1,  

    // Reserved bits 27:29
    // Reserved (SBZ)
    reserved_nmi_enable: B3,  

    // x2AVIC Enable (bit 30)
    // Nonzero if x2AVIC is enabled
    x2avic_enable: B1,  

    // AVIC Enable (bit 31)
    // Nonzero if AVIC is enabled
    avic_enable: B1,  

    // V_INTR_VECTOR (bits 39:32)
    // 8-bit vector to use for this interrupt
    v_intr_vector: B8,  

    // Reserved bits 40:63
    // Reserved (SBZ)
    reserved: B25,  
}

#[modular_bitfield::bitfield]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct VmShadowInterrupt {
    // INTERRUPT_SHADOW (bit 0)
    // Guest is in an interrupt shadow (1: true, 0: false)
    interrupt_shadow: B1,

    // GUEST_INTERRUPT_MASK (bit 1)
    // Value of RFLAGS.IF for the guest
    guest_interrupt_mask: B1,

    // Reserved bits (bits 63:2)
    // 62 bits reserved (SBZ)
    reserved: B62,
}

#[modular_bitfield::bitfield]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct NestedPagingFlags {
    /// Enable nested paging
    np_enable: B1,

    /// Enable Secure Encrypted Virtualization (SEV)
    enable_sev: B1,

    /// Enable encrypted state for Secure Encrypted Virtualization (SEV)
    enable_encrypted_state: B1,

    /// Guest Mode Execute Trap
    guest_mode_execute_trap: B1,

    /// Enable supervisor shadow stack restrictions in nested page tables.
    /// Support for this feature is indicated by CPUID Fn8000_000A_EDX[19] (SSSCheck).
    sss_check_en: B1,

    /// Virtual Transparent Encryption
    virtual_transparent_encryption: B1,

    /// Enable Read Only Guest Page Tables.
    /// See "Nested Table Walk" for more information.
    enable_read_only_guest_page_tables: B1,

    /// Enable INVLPGB/TLBSYNC.
    /// 
    /// 0 - INVLPGB and TLBSYNC will result in #UD.
    /// 1 - INVLPGB and TLBSYNC can be executed in guest.
    /// 
    /// Presence of this bit is indicated by CPUID bit 8000_000A, EDX[24] = 1.
    /// When in SEV-ES guest or this bit is not present, INVLPGB/TLBSYNC is always enabled in guest
    enable_inv_lpgb_tlbsync: B1,

    /// Reserved (SBZ)
    reserved: B56,
}

#[modular_bitfield::bitfield]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct AvicControlFlags {
    /// Reserved (SBZ)
    /// Bits 63:52 are reserved (SBZ)
    reserved: B12,

    /// AVIC APIC_BAR
    /// 
    /// Address of the APIC base for the AVIC (AMD Virtual APIC).
    /// The APIC_BAR is used by AVIC to access the APIC and other
    /// related functionality within the virtualized environment.
    /// 
    /// Bits 51:0 are for AVIC APIC_BAR
    apic_bar: B52,  
}

#[modular_bitfield::bitfield]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct LbrVirtualizationFlags {
    /// LBR Virtualization Enable
    /// 
    /// 0 — Do nothing.
    /// 1 — Enable LBR virtualization hardware acceleration.
    lbr_virtualization_enable: B1,

    /// Virtualized VMSAVE/VMLOAD Enable
    /// 
    /// Enables virtualized versions of VMSAVE and VMLOAD instructions.
    virtualized_vmsave_vmload_enable: B1,

    /// Virtualized Instruction-Based Sampling Enable
    /// 
    /// Enables virtualized instruction-based sampling.
    /// See "Instruction-Based Sampling Virtualization" for details.
    virtualized_instruction_sampling_enable: B1,

    /// Reserved (SBZ)
    /// Reserved bits (63:3)
    reserved: B61,
}

#[modular_bitfield::bitfield]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct VmcbCleanBits {
    /// VMCB Clean Bits
    /// 
    /// Represents the clean bits in the VMCB.
    /// These bits are used to track the state of the VMCB, indicating which fields
    /// have been modified or need to be written back to memory.
    /// Bits 31:0 for VMCB Clean Bits
    vmcb_clean_bits: B32,

    /// Reserved (SBZ)
    /// Bits 63:32 are reserved (SBZ)
    reserved: B32,  
}

#[modular_bitfield::bitfield]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct InstructionFetchCount {
    // Number of bytes fetched
    number_of_bytes_fetched: B8,  
    // Guest instruction bytes
    guest_instruction_bytes: B120,
}

