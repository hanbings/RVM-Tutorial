use crate::{mm::PhysFrame, HostPhysAddr, RvmHal, RvmResult};

#[derive(Debug)]
pub struct SvmRegion<H: RvmHal> {
    frame: PhysFrame<H>,
}

impl<H: RvmHal> SvmRegion<H> {
    pub fn new() -> RvmResult<Self> {
        let frame = PhysFrame::alloc_zero()?;
        Ok(Self { frame })
    }

    pub const unsafe fn uninit() -> Self {
        Self {
            frame: PhysFrame::uninit(),
        }
    }

    pub fn phys_addr(&self) -> HostPhysAddr {
        self.frame.start_paddr()
    }
}
