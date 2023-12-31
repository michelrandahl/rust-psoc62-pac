#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    ctl: CTL,
    _reserved1: [u8; 0x0c],
    idx: IDX,
    src: SRC,
    dst: DST,
    _reserved4: [u8; 0x04],
    curr: CURR,
    _reserved5: [u8; 0x04],
    tr_cmd: TR_CMD,
    _reserved6: [u8; 0x14],
    descr_status: DESCR_STATUS,
    _reserved7: [u8; 0x1c],
    descr_ctl: DESCR_CTL,
    descr_src: DESCR_SRC,
    descr_dst: DESCR_DST,
    descr_x_size: DESCR_X_SIZE,
    descr_x_incr: DESCR_X_INCR,
    descr_y_size: DESCR_Y_SIZE,
    descr_y_incr: DESCR_Y_INCR,
    descr_next: DESCR_NEXT,
    intr: INTR,
    intr_set: INTR_SET,
    intr_mask: INTR_MASK,
    intr_masked: INTR_MASKED,
}
impl CH {
    #[doc = "0x00 - Channel control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &CTL {
        &self.ctl
    }
    #[doc = "0x10 - Channel current indices"]
    #[inline(always)]
    pub const fn idx(&self) -> &IDX {
        &self.idx
    }
    #[doc = "0x14 - Channel current source address"]
    #[inline(always)]
    pub const fn src(&self) -> &SRC {
        &self.src
    }
    #[doc = "0x18 - Channel current destination address"]
    #[inline(always)]
    pub const fn dst(&self) -> &DST {
        &self.dst
    }
    #[doc = "0x20 - Channel current descriptor pointer"]
    #[inline(always)]
    pub const fn curr(&self) -> &CURR {
        &self.curr
    }
    #[doc = "0x28 - Channle software trigger"]
    #[inline(always)]
    pub const fn tr_cmd(&self) -> &TR_CMD {
        &self.tr_cmd
    }
    #[doc = "0x40 - Channel descriptor status"]
    #[inline(always)]
    pub const fn descr_status(&self) -> &DESCR_STATUS {
        &self.descr_status
    }
    #[doc = "0x60 - Channel descriptor control"]
    #[inline(always)]
    pub const fn descr_ctl(&self) -> &DESCR_CTL {
        &self.descr_ctl
    }
    #[doc = "0x64 - Channel descriptor source"]
    #[inline(always)]
    pub const fn descr_src(&self) -> &DESCR_SRC {
        &self.descr_src
    }
    #[doc = "0x68 - Channel descriptor destination"]
    #[inline(always)]
    pub const fn descr_dst(&self) -> &DESCR_DST {
        &self.descr_dst
    }
    #[doc = "0x6c - Channel descriptor X size"]
    #[inline(always)]
    pub const fn descr_x_size(&self) -> &DESCR_X_SIZE {
        &self.descr_x_size
    }
    #[doc = "0x70 - Channel descriptor X increment"]
    #[inline(always)]
    pub const fn descr_x_incr(&self) -> &DESCR_X_INCR {
        &self.descr_x_incr
    }
    #[doc = "0x74 - Channel descriptor Y size"]
    #[inline(always)]
    pub const fn descr_y_size(&self) -> &DESCR_Y_SIZE {
        &self.descr_y_size
    }
    #[doc = "0x78 - Channel descriptor Y increment"]
    #[inline(always)]
    pub const fn descr_y_incr(&self) -> &DESCR_Y_INCR {
        &self.descr_y_incr
    }
    #[doc = "0x7c - Channel descriptor next pointer"]
    #[inline(always)]
    pub const fn descr_next(&self) -> &DESCR_NEXT {
        &self.descr_next
    }
    #[doc = "0x80 - Interrupt"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0x84 - Interrupt set"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &INTR_SET {
        &self.intr_set
    }
    #[doc = "0x88 - Interrupt mask"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &INTR_MASK {
        &self.intr_mask
    }
    #[doc = "0x8c - Interrupt masked"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &INTR_MASKED {
        &self.intr_masked
    }
}
#[doc = "CTL (rw) register accessor: Channel control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Channel control"]
pub mod ctl;
#[doc = "IDX (r) register accessor: Channel current indices\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idx::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idx`]
module"]
pub type IDX = crate::Reg<idx::IDX_SPEC>;
#[doc = "Channel current indices"]
pub mod idx;
#[doc = "SRC (r) register accessor: Channel current source address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src`]
module"]
pub type SRC = crate::Reg<src::SRC_SPEC>;
#[doc = "Channel current source address"]
pub mod src;
#[doc = "DST (r) register accessor: Channel current destination address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst`]
module"]
pub type DST = crate::Reg<dst::DST_SPEC>;
#[doc = "Channel current destination address"]
pub mod dst;
#[doc = "CURR (rw) register accessor: Channel current descriptor pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`curr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`curr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@curr`]
module"]
pub type CURR = crate::Reg<curr::CURR_SPEC>;
#[doc = "Channel current descriptor pointer"]
pub mod curr;
#[doc = "TR_CMD (rw) register accessor: Channle software trigger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_cmd`]
module"]
pub type TR_CMD = crate::Reg<tr_cmd::TR_CMD_SPEC>;
#[doc = "Channle software trigger"]
pub mod tr_cmd;
#[doc = "DESCR_STATUS (r) register accessor: Channel descriptor status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descr_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descr_status`]
module"]
pub type DESCR_STATUS = crate::Reg<descr_status::DESCR_STATUS_SPEC>;
#[doc = "Channel descriptor status"]
pub mod descr_status;
#[doc = "DESCR_CTL (r) register accessor: Channel descriptor control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descr_ctl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descr_ctl`]
module"]
pub type DESCR_CTL = crate::Reg<descr_ctl::DESCR_CTL_SPEC>;
#[doc = "Channel descriptor control"]
pub mod descr_ctl;
#[doc = "DESCR_SRC (r) register accessor: Channel descriptor source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descr_src::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descr_src`]
module"]
pub type DESCR_SRC = crate::Reg<descr_src::DESCR_SRC_SPEC>;
#[doc = "Channel descriptor source"]
pub mod descr_src;
#[doc = "DESCR_DST (r) register accessor: Channel descriptor destination\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descr_dst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descr_dst`]
module"]
pub type DESCR_DST = crate::Reg<descr_dst::DESCR_DST_SPEC>;
#[doc = "Channel descriptor destination"]
pub mod descr_dst;
#[doc = "DESCR_X_SIZE (r) register accessor: Channel descriptor X size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descr_x_size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descr_x_size`]
module"]
pub type DESCR_X_SIZE = crate::Reg<descr_x_size::DESCR_X_SIZE_SPEC>;
#[doc = "Channel descriptor X size"]
pub mod descr_x_size;
#[doc = "DESCR_X_INCR (r) register accessor: Channel descriptor X increment\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descr_x_incr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descr_x_incr`]
module"]
pub type DESCR_X_INCR = crate::Reg<descr_x_incr::DESCR_X_INCR_SPEC>;
#[doc = "Channel descriptor X increment"]
pub mod descr_x_incr;
#[doc = "DESCR_Y_SIZE (r) register accessor: Channel descriptor Y size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descr_y_size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descr_y_size`]
module"]
pub type DESCR_Y_SIZE = crate::Reg<descr_y_size::DESCR_Y_SIZE_SPEC>;
#[doc = "Channel descriptor Y size"]
pub mod descr_y_size;
#[doc = "DESCR_Y_INCR (r) register accessor: Channel descriptor Y increment\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descr_y_incr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descr_y_incr`]
module"]
pub type DESCR_Y_INCR = crate::Reg<descr_y_incr::DESCR_Y_INCR_SPEC>;
#[doc = "Channel descriptor Y increment"]
pub mod descr_y_incr;
#[doc = "DESCR_NEXT (r) register accessor: Channel descriptor next pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descr_next::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descr_next`]
module"]
pub type DESCR_NEXT = crate::Reg<descr_next::DESCR_NEXT_SPEC>;
#[doc = "Channel descriptor next pointer"]
pub mod descr_next;
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
