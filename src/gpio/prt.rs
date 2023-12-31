#[doc = r"Register block"]
#[repr(C)]
pub struct PRT {
    out: OUT,
    out_clr: OUT_CLR,
    out_set: OUT_SET,
    out_inv: OUT_INV,
    in_: IN,
    intr: INTR,
    intr_mask: INTR_MASK,
    intr_masked: INTR_MASKED,
    intr_set: INTR_SET,
    _reserved9: [u8; 0x1c],
    intr_cfg: INTR_CFG,
    cfg: CFG,
    cfg_in: CFG_IN,
    cfg_out: CFG_OUT,
    cfg_sio: CFG_SIO,
    _reserved14: [u8; 0x04],
    cfg_in_autolvl: CFG_IN_AUTOLVL,
}
impl PRT {
    #[doc = "0x00 - Port output data register"]
    #[inline(always)]
    pub const fn out(&self) -> &OUT {
        &self.out
    }
    #[doc = "0x04 - Port output data clear register"]
    #[inline(always)]
    pub const fn out_clr(&self) -> &OUT_CLR {
        &self.out_clr
    }
    #[doc = "0x08 - Port output data set register"]
    #[inline(always)]
    pub const fn out_set(&self) -> &OUT_SET {
        &self.out_set
    }
    #[doc = "0x0c - Port output data invert register"]
    #[inline(always)]
    pub const fn out_inv(&self) -> &OUT_INV {
        &self.out_inv
    }
    #[doc = "0x10 - Port input state register"]
    #[inline(always)]
    pub const fn in_(&self) -> &IN {
        &self.in_
    }
    #[doc = "0x14 - Port interrupt status register"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0x18 - Port interrupt mask register"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &INTR_MASK {
        &self.intr_mask
    }
    #[doc = "0x1c - Port interrupt masked status register"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &INTR_MASKED {
        &self.intr_masked
    }
    #[doc = "0x20 - Port interrupt set register"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &INTR_SET {
        &self.intr_set
    }
    #[doc = "0x40 - Port interrupt configuration register"]
    #[inline(always)]
    pub const fn intr_cfg(&self) -> &INTR_CFG {
        &self.intr_cfg
    }
    #[doc = "0x44 - Port configuration register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &CFG {
        &self.cfg
    }
    #[doc = "0x48 - Port input buffer configuration register"]
    #[inline(always)]
    pub const fn cfg_in(&self) -> &CFG_IN {
        &self.cfg_in
    }
    #[doc = "0x4c - Port output buffer configuration register"]
    #[inline(always)]
    pub const fn cfg_out(&self) -> &CFG_OUT {
        &self.cfg_out
    }
    #[doc = "0x50 - Port SIO configuration register"]
    #[inline(always)]
    pub const fn cfg_sio(&self) -> &CFG_SIO {
        &self.cfg_sio
    }
    #[doc = "0x58 - Port input buffer AUTOLVL configuration register for S40E GPIO"]
    #[inline(always)]
    pub const fn cfg_in_autolvl(&self) -> &CFG_IN_AUTOLVL {
        &self.cfg_in_autolvl
    }
}
#[doc = "OUT (rw) register accessor: Port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`]
module"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "Port output data register"]
pub mod out;
#[doc = "OUT_CLR (rw) register accessor: Port output data clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_clr`]
module"]
pub type OUT_CLR = crate::Reg<out_clr::OUT_CLR_SPEC>;
#[doc = "Port output data clear register"]
pub mod out_clr;
#[doc = "OUT_SET (rw) register accessor: Port output data set register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_set`]
module"]
pub type OUT_SET = crate::Reg<out_set::OUT_SET_SPEC>;
#[doc = "Port output data set register"]
pub mod out_set;
#[doc = "OUT_INV (rw) register accessor: Port output data invert register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_inv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_inv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_inv`]
module"]
pub type OUT_INV = crate::Reg<out_inv::OUT_INV_SPEC>;
#[doc = "Port output data invert register"]
pub mod out_inv;
#[doc = "IN (r) register accessor: Port input state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`]
module"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "Port input state register"]
pub mod in_;
#[doc = "INTR (rw) register accessor: Port interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Port interrupt status register"]
pub mod intr;
#[doc = "INTR_MASK (rw) register accessor: Port interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mask`]
module"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Port interrupt mask register"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: Port interrupt masked status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_masked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_masked`]
module"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Port interrupt masked status register"]
pub mod intr_masked;
#[doc = "INTR_SET (rw) register accessor: Port interrupt set register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_set`]
module"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Port interrupt set register"]
pub mod intr_set;
#[doc = "INTR_CFG (rw) register accessor: Port interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cfg`]
module"]
pub type INTR_CFG = crate::Reg<intr_cfg::INTR_CFG_SPEC>;
#[doc = "Port interrupt configuration register"]
pub mod intr_cfg;
#[doc = "CFG (rw) register accessor: Port configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Port configuration register"]
pub mod cfg;
#[doc = "CFG_IN (rw) register accessor: Port input buffer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_in::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_in::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_in`]
module"]
pub type CFG_IN = crate::Reg<cfg_in::CFG_IN_SPEC>;
#[doc = "Port input buffer configuration register"]
pub mod cfg_in;
#[doc = "CFG_OUT (rw) register accessor: Port output buffer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_out`]
module"]
pub type CFG_OUT = crate::Reg<cfg_out::CFG_OUT_SPEC>;
#[doc = "Port output buffer configuration register"]
pub mod cfg_out;
#[doc = "CFG_SIO (rw) register accessor: Port SIO configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_sio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_sio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_sio`]
module"]
pub type CFG_SIO = crate::Reg<cfg_sio::CFG_SIO_SPEC>;
#[doc = "Port SIO configuration register"]
pub mod cfg_sio;
#[doc = "CFG_IN_AUTOLVL (rw) register accessor: Port input buffer AUTOLVL configuration register for S40E GPIO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_in_autolvl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_in_autolvl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_in_autolvl`]
module"]
pub type CFG_IN_AUTOLVL = crate::Reg<cfg_in_autolvl::CFG_IN_AUTOLVL_SPEC>;
#[doc = "Port input buffer AUTOLVL configuration register for S40E GPIO"]
pub mod cfg_in_autolvl;
