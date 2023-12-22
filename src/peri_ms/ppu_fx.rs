#[doc = r"Register block"]
#[repr(C)]
pub struct PPU_FX {
    sl_addr: SL_ADDR,
    sl_size: SL_SIZE,
    _reserved2: [u8; 0x08],
    sl_att0: SL_ATT0,
    sl_att1: SL_ATT1,
    sl_att2: SL_ATT2,
    sl_att3: SL_ATT3,
    ms_addr: MS_ADDR,
    ms_size: MS_SIZE,
    _reserved8: [u8; 0x08],
    ms_att0: MS_ATT0,
    ms_att1: MS_ATT1,
    ms_att2: MS_ATT2,
    ms_att3: MS_ATT3,
}
impl PPU_FX {
    #[doc = "0x00 - Slave region, base address"]
    #[inline(always)]
    pub const fn sl_addr(&self) -> &SL_ADDR {
        &self.sl_addr
    }
    #[doc = "0x04 - Slave region, size"]
    #[inline(always)]
    pub const fn sl_size(&self) -> &SL_SIZE {
        &self.sl_size
    }
    #[doc = "0x10 - Slave attributes 0"]
    #[inline(always)]
    pub const fn sl_att0(&self) -> &SL_ATT0 {
        &self.sl_att0
    }
    #[doc = "0x14 - Slave attributes 1"]
    #[inline(always)]
    pub const fn sl_att1(&self) -> &SL_ATT1 {
        &self.sl_att1
    }
    #[doc = "0x18 - Slave attributes 2"]
    #[inline(always)]
    pub const fn sl_att2(&self) -> &SL_ATT2 {
        &self.sl_att2
    }
    #[doc = "0x1c - Slave attributes 3"]
    #[inline(always)]
    pub const fn sl_att3(&self) -> &SL_ATT3 {
        &self.sl_att3
    }
    #[doc = "0x20 - Master region, base address"]
    #[inline(always)]
    pub const fn ms_addr(&self) -> &MS_ADDR {
        &self.ms_addr
    }
    #[doc = "0x24 - Master region, size"]
    #[inline(always)]
    pub const fn ms_size(&self) -> &MS_SIZE {
        &self.ms_size
    }
    #[doc = "0x30 - Master attributes 0"]
    #[inline(always)]
    pub const fn ms_att0(&self) -> &MS_ATT0 {
        &self.ms_att0
    }
    #[doc = "0x34 - Master attributes 1"]
    #[inline(always)]
    pub const fn ms_att1(&self) -> &MS_ATT1 {
        &self.ms_att1
    }
    #[doc = "0x38 - Master attributes 2"]
    #[inline(always)]
    pub const fn ms_att2(&self) -> &MS_ATT2 {
        &self.ms_att2
    }
    #[doc = "0x3c - Master attributes 3"]
    #[inline(always)]
    pub const fn ms_att3(&self) -> &MS_ATT3 {
        &self.ms_att3
    }
}
#[doc = "SL_ADDR (r) register accessor: Slave region, base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sl_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sl_addr`]
module"]
pub type SL_ADDR = crate::Reg<sl_addr::SL_ADDR_SPEC>;
#[doc = "Slave region, base address"]
pub mod sl_addr;
#[doc = "SL_SIZE (r) register accessor: Slave region, size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sl_size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sl_size`]
module"]
pub type SL_SIZE = crate::Reg<sl_size::SL_SIZE_SPEC>;
#[doc = "Slave region, size"]
pub mod sl_size;
#[doc = "SL_ATT0 (rw) register accessor: Slave attributes 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sl_att0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sl_att0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sl_att0`]
module"]
pub type SL_ATT0 = crate::Reg<sl_att0::SL_ATT0_SPEC>;
#[doc = "Slave attributes 0"]
pub mod sl_att0;
#[doc = "SL_ATT1 (rw) register accessor: Slave attributes 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sl_att1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sl_att1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sl_att1`]
module"]
pub type SL_ATT1 = crate::Reg<sl_att1::SL_ATT1_SPEC>;
#[doc = "Slave attributes 1"]
pub mod sl_att1;
#[doc = "SL_ATT2 (rw) register accessor: Slave attributes 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sl_att2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sl_att2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sl_att2`]
module"]
pub type SL_ATT2 = crate::Reg<sl_att2::SL_ATT2_SPEC>;
#[doc = "Slave attributes 2"]
pub mod sl_att2;
#[doc = "SL_ATT3 (rw) register accessor: Slave attributes 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sl_att3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sl_att3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sl_att3`]
module"]
pub type SL_ATT3 = crate::Reg<sl_att3::SL_ATT3_SPEC>;
#[doc = "Slave attributes 3"]
pub mod sl_att3;
#[doc = "MS_ADDR (r) register accessor: Master region, base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms_addr`]
module"]
pub type MS_ADDR = crate::Reg<ms_addr::MS_ADDR_SPEC>;
#[doc = "Master region, base address"]
pub mod ms_addr;
#[doc = "MS_SIZE (r) register accessor: Master region, size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms_size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms_size`]
module"]
pub type MS_SIZE = crate::Reg<ms_size::MS_SIZE_SPEC>;
#[doc = "Master region, size"]
pub mod ms_size;
#[doc = "MS_ATT0 (rw) register accessor: Master attributes 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms_att0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms_att0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms_att0`]
module"]
pub type MS_ATT0 = crate::Reg<ms_att0::MS_ATT0_SPEC>;
#[doc = "Master attributes 0"]
pub mod ms_att0;
#[doc = "MS_ATT1 (rw) register accessor: Master attributes 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms_att1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms_att1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms_att1`]
module"]
pub type MS_ATT1 = crate::Reg<ms_att1::MS_ATT1_SPEC>;
#[doc = "Master attributes 1"]
pub mod ms_att1;
#[doc = "MS_ATT2 (rw) register accessor: Master attributes 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms_att2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms_att2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms_att2`]
module"]
pub type MS_ATT2 = crate::Reg<ms_att2::MS_ATT2_SPEC>;
#[doc = "Master attributes 2"]
pub mod ms_att2;
#[doc = "MS_ATT3 (rw) register accessor: Master attributes 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms_att3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms_att3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms_att3`]
module"]
pub type MS_ATT3 = crate::Reg<ms_att3::MS_ATT3_SPEC>;
#[doc = "Master attributes 3"]
pub mod ms_att3;
