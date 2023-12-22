#[doc = r"Register block"]
#[repr(C)]
pub struct PRT {
    port_sel0: PORT_SEL0,
    port_sel1: PORT_SEL1,
}
impl PRT {
    #[doc = "0x00 - Port selection 0"]
    #[inline(always)]
    pub const fn port_sel0(&self) -> &PORT_SEL0 {
        &self.port_sel0
    }
    #[doc = "0x04 - Port selection 1"]
    #[inline(always)]
    pub const fn port_sel1(&self) -> &PORT_SEL1 {
        &self.port_sel1
    }
}
#[doc = "PORT_SEL0 (rw) register accessor: Port selection 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`port_sel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`port_sel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@port_sel0`]
module"]
pub type PORT_SEL0 = crate::Reg<port_sel0::PORT_SEL0_SPEC>;
#[doc = "Port selection 0"]
pub mod port_sel0;
#[doc = "PORT_SEL1 (rw) register accessor: Port selection 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`port_sel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`port_sel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@port_sel1`]
module"]
pub type PORT_SEL1 = crate::Reg<port_sel1::PORT_SEL1_SPEC>;
#[doc = "Port selection 1"]
pub mod port_sel1;
