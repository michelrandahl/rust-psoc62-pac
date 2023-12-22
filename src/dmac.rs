#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctl: CTL,
    _reserved1: [u8; 0x04],
    active: ACTIVE,
    _reserved2: [u8; 0x0ff4],
    ch: (),
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &CTL {
        &self.ctl
    }
    #[doc = "0x08 - Active channels"]
    #[inline(always)]
    pub const fn active(&self) -> &ACTIVE {
        &self.active
    }
    #[doc = "0x1000..0x1240 - DMA controller channel"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4096)
                .add(256 * n)
                .cast()
        }
    }
}
#[doc = "CTL (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "ACTIVE (r) register accessor: Active channels\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`active::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@active`]
module"]
pub type ACTIVE = crate::Reg<active::ACTIVE_SPEC>;
#[doc = "Active channels"]
pub mod active;
#[doc = "DMA controller channel"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "DMA controller channel"]
pub mod ch;
