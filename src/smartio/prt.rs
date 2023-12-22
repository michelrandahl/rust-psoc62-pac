#[doc = r"Register block"]
#[repr(C)]
pub struct PRT {
    ctl: CTL,
    _reserved1: [u8; 0x0c],
    sync_ctl: SYNC_CTL,
    _reserved2: [u8; 0x0c],
    lut_sel: [LUT_SEL; 8],
    lut_ctl: [LUT_CTL; 8],
    _reserved4: [u8; 0x60],
    du_sel: DU_SEL,
    du_ctl: DU_CTL,
    _reserved6: [u8; 0x28],
    data: DATA,
}
impl PRT {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &CTL {
        &self.ctl
    }
    #[doc = "0x10 - Synchronization control register"]
    #[inline(always)]
    pub const fn sync_ctl(&self) -> &SYNC_CTL {
        &self.sync_ctl
    }
    #[doc = "0x20..0x40 - LUT component input selection"]
    #[inline(always)]
    pub const fn lut_sel(&self, n: usize) -> &LUT_SEL {
        &self.lut_sel[n]
    }
    #[doc = "0x40..0x60 - LUT component control register"]
    #[inline(always)]
    pub const fn lut_ctl(&self, n: usize) -> &LUT_CTL {
        &self.lut_ctl[n]
    }
    #[doc = "0xc0 - Data unit component input selection"]
    #[inline(always)]
    pub const fn du_sel(&self) -> &DU_SEL {
        &self.du_sel
    }
    #[doc = "0xc4 - Data unit component control register"]
    #[inline(always)]
    pub const fn du_ctl(&self) -> &DU_CTL {
        &self.du_ctl
    }
    #[doc = "0xf0 - Data register"]
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
}
#[doc = "CTL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "SYNC_CTL (rw) register accessor: Synchronization control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync_ctl`]
module"]
pub type SYNC_CTL = crate::Reg<sync_ctl::SYNC_CTL_SPEC>;
#[doc = "Synchronization control register"]
pub mod sync_ctl;
#[doc = "LUT_SEL (rw) register accessor: LUT component input selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut_sel`]
module"]
pub type LUT_SEL = crate::Reg<lut_sel::LUT_SEL_SPEC>;
#[doc = "LUT component input selection"]
pub mod lut_sel;
#[doc = "LUT_CTL (rw) register accessor: LUT component control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lut_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut_ctl`]
module"]
pub type LUT_CTL = crate::Reg<lut_ctl::LUT_CTL_SPEC>;
#[doc = "LUT component control register"]
pub mod lut_ctl;
#[doc = "DU_SEL (rw) register accessor: Data unit component input selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`du_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`du_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@du_sel`]
module"]
pub type DU_SEL = crate::Reg<du_sel::DU_SEL_SPEC>;
#[doc = "Data unit component input selection"]
pub mod du_sel;
#[doc = "DU_CTL (rw) register accessor: Data unit component control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`du_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`du_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@du_ctl`]
module"]
pub type DU_CTL = crate::Reg<du_ctl::DU_CTL_SPEC>;
#[doc = "Data unit component control register"]
pub mod du_ctl;
#[doc = "DATA (rw) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data register"]
pub mod data;
