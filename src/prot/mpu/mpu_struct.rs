#[doc = r"Register block"]
#[repr(C)]
pub struct MPU_STRUCT {
    addr: ADDR,
    att: ATT,
}
impl MPU_STRUCT {
    #[doc = "0x00 - MPU region address"]
    #[inline(always)]
    pub const fn addr(&self) -> &ADDR {
        &self.addr
    }
    #[doc = "0x04 - MPU region attrributes"]
    #[inline(always)]
    pub const fn att(&self) -> &ATT {
        &self.att
    }
}
#[doc = "ADDR (rw) register accessor: MPU region address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "MPU region address"]
pub mod addr;
#[doc = "ATT (rw) register accessor: MPU region attrributes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`att::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`att::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@att`]
module"]
pub type ATT = crate::Reg<att::ATT_SPEC>;
#[doc = "MPU region attrributes"]
pub mod att;
