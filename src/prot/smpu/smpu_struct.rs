#[doc = r"Register block"]
#[repr(C)]
pub struct SMPU_STRUCT {
    addr0: ADDR0,
    att0: ATT0,
    _reserved2: [u8; 0x18],
    addr1: ADDR1,
    att1: ATT1,
}
impl SMPU_STRUCT {
    #[doc = "0x00 - SMPU region address 0 (slave structure)"]
    #[inline(always)]
    pub const fn addr0(&self) -> &ADDR0 {
        &self.addr0
    }
    #[doc = "0x04 - SMPU region attributes 0 (slave structure)"]
    #[inline(always)]
    pub const fn att0(&self) -> &ATT0 {
        &self.att0
    }
    #[doc = "0x20 - SMPU region address 1 (master structure)"]
    #[inline(always)]
    pub const fn addr1(&self) -> &ADDR1 {
        &self.addr1
    }
    #[doc = "0x24 - SMPU region attributes 1 (master structure)"]
    #[inline(always)]
    pub const fn att1(&self) -> &ATT1 {
        &self.att1
    }
}
#[doc = "ADDR0 (rw) register accessor: SMPU region address 0 (slave structure)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr0`]
module"]
pub type ADDR0 = crate::Reg<addr0::ADDR0_SPEC>;
#[doc = "SMPU region address 0 (slave structure)"]
pub mod addr0;
#[doc = "ATT0 (rw) register accessor: SMPU region attributes 0 (slave structure)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`att0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`att0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@att0`]
module"]
pub type ATT0 = crate::Reg<att0::ATT0_SPEC>;
#[doc = "SMPU region attributes 0 (slave structure)"]
pub mod att0;
#[doc = "ADDR1 (r) register accessor: SMPU region address 1 (master structure)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr1`]
module"]
pub type ADDR1 = crate::Reg<addr1::ADDR1_SPEC>;
#[doc = "SMPU region address 1 (master structure)"]
pub mod addr1;
#[doc = "ATT1 (rw) register accessor: SMPU region attributes 1 (master structure)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`att1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`att1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@att1`]
module"]
pub type ATT1 = crate::Reg<att1::ATT1_SPEC>;
#[doc = "SMPU region attributes 1 (master structure)"]
pub mod att1;
