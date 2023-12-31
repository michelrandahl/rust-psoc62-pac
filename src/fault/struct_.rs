#[doc = r"Register block"]
#[repr(C)]
pub struct STRUCT {
    ctl: CTL,
    _reserved1: [u8; 0x08],
    status: STATUS,
    data: [DATA; 4],
    _reserved3: [u8; 0x20],
    pending0: PENDING0,
    pending1: PENDING1,
    pending2: PENDING2,
    _reserved6: [u8; 0x04],
    mask0: MASK0,
    mask1: MASK1,
    mask2: MASK2,
    _reserved9: [u8; 0x64],
    intr: INTR,
    intr_set: INTR_SET,
    intr_mask: INTR_MASK,
    intr_masked: INTR_MASKED,
}
impl STRUCT {
    #[doc = "0x00 - Fault control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &CTL {
        &self.ctl
    }
    #[doc = "0x0c - Fault status"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x10..0x20 - Fault data"]
    #[inline(always)]
    pub const fn data(&self, n: usize) -> &DATA {
        &self.data[n]
    }
    #[doc = "0x40 - Fault pending 0"]
    #[inline(always)]
    pub const fn pending0(&self) -> &PENDING0 {
        &self.pending0
    }
    #[doc = "0x44 - Fault pending 1"]
    #[inline(always)]
    pub const fn pending1(&self) -> &PENDING1 {
        &self.pending1
    }
    #[doc = "0x48 - Fault pending 2"]
    #[inline(always)]
    pub const fn pending2(&self) -> &PENDING2 {
        &self.pending2
    }
    #[doc = "0x50 - Fault mask 0"]
    #[inline(always)]
    pub const fn mask0(&self) -> &MASK0 {
        &self.mask0
    }
    #[doc = "0x54 - Fault mask 1"]
    #[inline(always)]
    pub const fn mask1(&self) -> &MASK1 {
        &self.mask1
    }
    #[doc = "0x58 - Fault mask 2"]
    #[inline(always)]
    pub const fn mask2(&self) -> &MASK2 {
        &self.mask2
    }
    #[doc = "0xc0 - Interrupt"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0xc4 - Interrupt set"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &INTR_SET {
        &self.intr_set
    }
    #[doc = "0xc8 - Interrupt mask"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &INTR_MASK {
        &self.intr_mask
    }
    #[doc = "0xcc - Interrupt masked"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &INTR_MASKED {
        &self.intr_masked
    }
}
#[doc = "CTL (rw) register accessor: Fault control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Fault control"]
pub mod ctl;
#[doc = "STATUS (rw) register accessor: Fault status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Fault status"]
pub mod status;
#[doc = "DATA (rw) register accessor: Fault data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Fault data"]
pub mod data;
#[doc = "PENDING0 (r) register accessor: Fault pending 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pending0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pending0`]
module"]
pub type PENDING0 = crate::Reg<pending0::PENDING0_SPEC>;
#[doc = "Fault pending 0"]
pub mod pending0;
#[doc = "PENDING1 (r) register accessor: Fault pending 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pending1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pending1`]
module"]
pub type PENDING1 = crate::Reg<pending1::PENDING1_SPEC>;
#[doc = "Fault pending 1"]
pub mod pending1;
#[doc = "PENDING2 (r) register accessor: Fault pending 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pending2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pending2`]
module"]
pub type PENDING2 = crate::Reg<pending2::PENDING2_SPEC>;
#[doc = "Fault pending 2"]
pub mod pending2;
#[doc = "MASK0 (rw) register accessor: Fault mask 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask0`]
module"]
pub type MASK0 = crate::Reg<mask0::MASK0_SPEC>;
#[doc = "Fault mask 0"]
pub mod mask0;
#[doc = "MASK1 (rw) register accessor: Fault mask 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask1`]
module"]
pub type MASK1 = crate::Reg<mask1::MASK1_SPEC>;
#[doc = "Fault mask 1"]
pub mod mask1;
#[doc = "MASK2 (rw) register accessor: Fault mask 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask2`]
module"]
pub type MASK2 = crate::Reg<mask2::MASK2_SPEC>;
#[doc = "Fault mask 2"]
pub mod mask2;
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
