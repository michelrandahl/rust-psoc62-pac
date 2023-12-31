#[doc = r"Register block"]
#[repr(C)]
pub struct CH_STRUCT {
    ch_ctl: CH_CTL,
    ch_status: CH_STATUS,
    ch_idx: CH_IDX,
    ch_curr_ptr: CH_CURR_PTR,
    intr: INTR,
    intr_set: INTR_SET,
    intr_mask: INTR_MASK,
    intr_masked: INTR_MASKED,
    sram_data0: SRAM_DATA0,
    sram_data1: SRAM_DATA1,
    tr_cmd: TR_CMD,
}
impl CH_STRUCT {
    #[doc = "0x00 - Channel control"]
    #[inline(always)]
    pub const fn ch_ctl(&self) -> &CH_CTL {
        &self.ch_ctl
    }
    #[doc = "0x04 - Channel status"]
    #[inline(always)]
    pub const fn ch_status(&self) -> &CH_STATUS {
        &self.ch_status
    }
    #[doc = "0x08 - Channel current indices"]
    #[inline(always)]
    pub const fn ch_idx(&self) -> &CH_IDX {
        &self.ch_idx
    }
    #[doc = "0x0c - Channel current descriptor pointer"]
    #[inline(always)]
    pub const fn ch_curr_ptr(&self) -> &CH_CURR_PTR {
        &self.ch_curr_ptr
    }
    #[doc = "0x10 - Interrupt"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0x14 - Interrupt set"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &INTR_SET {
        &self.intr_set
    }
    #[doc = "0x18 - Interrupt mask"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &INTR_MASK {
        &self.intr_mask
    }
    #[doc = "0x1c - Interrupt masked"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &INTR_MASKED {
        &self.intr_masked
    }
    #[doc = "0x20 - SRAM data 0"]
    #[inline(always)]
    pub const fn sram_data0(&self) -> &SRAM_DATA0 {
        &self.sram_data0
    }
    #[doc = "0x24 - SRAM data 1"]
    #[inline(always)]
    pub const fn sram_data1(&self) -> &SRAM_DATA1 {
        &self.sram_data1
    }
    #[doc = "0x28 - Channel software trigger"]
    #[inline(always)]
    pub const fn tr_cmd(&self) -> &TR_CMD {
        &self.tr_cmd
    }
}
#[doc = "CH_CTL (rw) register accessor: Channel control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ctl`]
module"]
pub type CH_CTL = crate::Reg<ch_ctl::CH_CTL_SPEC>;
#[doc = "Channel control"]
pub mod ch_ctl;
#[doc = "CH_STATUS (r) register accessor: Channel status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_status`]
module"]
pub type CH_STATUS = crate::Reg<ch_status::CH_STATUS_SPEC>;
#[doc = "Channel status"]
pub mod ch_status;
#[doc = "CH_IDX (rw) register accessor: Channel current indices\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_idx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_idx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_idx`]
module"]
pub type CH_IDX = crate::Reg<ch_idx::CH_IDX_SPEC>;
#[doc = "Channel current indices"]
pub mod ch_idx;
#[doc = "CH_CURR_PTR (rw) register accessor: Channel current descriptor pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_curr_ptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_curr_ptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_curr_ptr`]
module"]
pub type CH_CURR_PTR = crate::Reg<ch_curr_ptr::CH_CURR_PTR_SPEC>;
#[doc = "Channel current descriptor pointer"]
pub mod ch_curr_ptr;
#[doc = "INTR (rw) register accessor: Interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: Interrupt set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_set`]
module"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt set"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mask`]
module"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt mask"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: Interrupt masked\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_masked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_masked`]
module"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt masked"]
pub mod intr_masked;
#[doc = "SRAM_DATA0 (rw) register accessor: SRAM data 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_data0`]
module"]
pub type SRAM_DATA0 = crate::Reg<sram_data0::SRAM_DATA0_SPEC>;
#[doc = "SRAM data 0"]
pub mod sram_data0;
#[doc = "SRAM_DATA1 (rw) register accessor: SRAM data 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_data1`]
module"]
pub type SRAM_DATA1 = crate::Reg<sram_data1::SRAM_DATA1_SPEC>;
#[doc = "SRAM data 1"]
pub mod sram_data1;
#[doc = "TR_CMD (rw) register accessor: Channel software trigger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_cmd`]
module"]
pub type TR_CMD = crate::Reg<tr_cmd::TR_CMD_SPEC>;
#[doc = "Channel software trigger"]
pub mod tr_cmd;
