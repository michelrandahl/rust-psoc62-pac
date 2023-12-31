#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    config: CONFIG,
    status: STATUS,
    _reserved2: [u8; 0x08],
    intr: INTR,
    intr_set: INTR_SET,
    intr_mask: INTR_MASK,
    intr_masked: INTR_MASKED,
    _reserved6: [u8; 0x20],
    cmp0_ctrl: CMP0_CTRL,
    _reserved7: [u8; 0x0c],
    cmp0_sw: CMP0_SW,
    cmp0_sw_clear: CMP0_SW_CLEAR,
    _reserved9: [u8; 0x28],
    cmp1_ctrl: CMP1_CTRL,
    _reserved10: [u8; 0x0c],
    cmp1_sw: CMP1_SW,
    cmp1_sw_clear: CMP1_SW_CLEAR,
}
impl RegisterBlock {
    #[doc = "0x00 - LPCOMP Configuration Register"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    #[doc = "0x04 - LPCOMP Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x10 - LPCOMP Interrupt request register"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0x14 - LPCOMP Interrupt set register"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &INTR_SET {
        &self.intr_set
    }
    #[doc = "0x18 - LPCOMP Interrupt request mask"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &INTR_MASK {
        &self.intr_mask
    }
    #[doc = "0x1c - LPCOMP Interrupt request masked"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &INTR_MASKED {
        &self.intr_masked
    }
    #[doc = "0x40 - Comparator 0 control Register"]
    #[inline(always)]
    pub const fn cmp0_ctrl(&self) -> &CMP0_CTRL {
        &self.cmp0_ctrl
    }
    #[doc = "0x50 - Comparator 0 switch control"]
    #[inline(always)]
    pub const fn cmp0_sw(&self) -> &CMP0_SW {
        &self.cmp0_sw
    }
    #[doc = "0x54 - Comparator 0 switch control clear"]
    #[inline(always)]
    pub const fn cmp0_sw_clear(&self) -> &CMP0_SW_CLEAR {
        &self.cmp0_sw_clear
    }
    #[doc = "0x80 - Comparator 1 control Register"]
    #[inline(always)]
    pub const fn cmp1_ctrl(&self) -> &CMP1_CTRL {
        &self.cmp1_ctrl
    }
    #[doc = "0x90 - Comparator 1 switch control"]
    #[inline(always)]
    pub const fn cmp1_sw(&self) -> &CMP1_SW {
        &self.cmp1_sw
    }
    #[doc = "0x94 - Comparator 1 switch control clear"]
    #[inline(always)]
    pub const fn cmp1_sw_clear(&self) -> &CMP1_SW_CLEAR {
        &self.cmp1_sw_clear
    }
}
#[doc = "CONFIG (rw) register accessor: LPCOMP Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "LPCOMP Configuration Register"]
pub mod config;
#[doc = "STATUS (r) register accessor: LPCOMP Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "LPCOMP Status Register"]
pub mod status;
#[doc = "INTR (rw) register accessor: LPCOMP Interrupt request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "LPCOMP Interrupt request register"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: LPCOMP Interrupt set register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_set`]
module"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "LPCOMP Interrupt set register"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: LPCOMP Interrupt request mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mask`]
module"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "LPCOMP Interrupt request mask"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: LPCOMP Interrupt request masked\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_masked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_masked`]
module"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "LPCOMP Interrupt request masked"]
pub mod intr_masked;
#[doc = "CMP0_CTRL (rw) register accessor: Comparator 0 control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp0_ctrl`]
module"]
pub type CMP0_CTRL = crate::Reg<cmp0_ctrl::CMP0_CTRL_SPEC>;
#[doc = "Comparator 0 control Register"]
pub mod cmp0_ctrl;
#[doc = "CMP0_SW (rw) register accessor: Comparator 0 switch control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp0_sw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp0_sw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp0_sw`]
module"]
pub type CMP0_SW = crate::Reg<cmp0_sw::CMP0_SW_SPEC>;
#[doc = "Comparator 0 switch control"]
pub mod cmp0_sw;
#[doc = "CMP0_SW_CLEAR (rw) register accessor: Comparator 0 switch control clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp0_sw_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp0_sw_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp0_sw_clear`]
module"]
pub type CMP0_SW_CLEAR = crate::Reg<cmp0_sw_clear::CMP0_SW_CLEAR_SPEC>;
#[doc = "Comparator 0 switch control clear"]
pub mod cmp0_sw_clear;
#[doc = "CMP1_CTRL (rw) register accessor: Comparator 1 control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1_ctrl`]
module"]
pub type CMP1_CTRL = crate::Reg<cmp1_ctrl::CMP1_CTRL_SPEC>;
#[doc = "Comparator 1 control Register"]
pub mod cmp1_ctrl;
#[doc = "CMP1_SW (rw) register accessor: Comparator 1 switch control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1_sw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1_sw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1_sw`]
module"]
pub type CMP1_SW = crate::Reg<cmp1_sw::CMP1_SW_SPEC>;
#[doc = "Comparator 1 switch control"]
pub mod cmp1_sw;
#[doc = "CMP1_SW_CLEAR (rw) register accessor: Comparator 1 switch control clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1_sw_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1_sw_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1_sw_clear`]
module"]
pub type CMP1_SW_CLEAR = crate::Reg<cmp1_sw_clear::CMP1_SW_CLEAR_SPEC>;
#[doc = "Comparator 1 switch control clear"]
pub mod cmp1_sw_clear;
