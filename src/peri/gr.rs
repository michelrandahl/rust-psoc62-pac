#[doc = r"Register block"]
#[repr(C)]
pub struct GR {
    clock_ctl: CLOCK_CTL,
    _reserved1: [u8; 0x0c],
    sl_ctl: SL_CTL,
}
impl GR {
    #[doc = "0x00 - Clock control"]
    #[inline(always)]
    pub const fn clock_ctl(&self) -> &CLOCK_CTL {
        &self.clock_ctl
    }
    #[doc = "0x10 - Slave control"]
    #[inline(always)]
    pub const fn sl_ctl(&self) -> &SL_CTL {
        &self.sl_ctl
    }
}
#[doc = "CLOCK_CTL (rw) register accessor: Clock control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_ctl`]
module"]
pub type CLOCK_CTL = crate::Reg<clock_ctl::CLOCK_CTL_SPEC>;
#[doc = "Clock control"]
pub mod clock_ctl;
#[doc = "SL_CTL (rw) register accessor: Slave control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sl_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sl_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sl_ctl`]
module"]
pub type SL_CTL = crate::Reg<sl_ctl::SL_CTL_SPEC>;
#[doc = "Slave control"]
pub mod sl_ctl;
