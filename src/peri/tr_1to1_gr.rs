#[doc = r"Register block"]
#[repr(C)]
pub struct TR_1TO1_GR {
    tr_ctl: [TR_CTL; 256],
}
impl TR_1TO1_GR {
    #[doc = "0x00..0x400 - Trigger control register"]
    #[inline(always)]
    pub const fn tr_ctl(&self, n: usize) -> &TR_CTL {
        &self.tr_ctl[n]
    }
}
#[doc = "TR_CTL (rw) register accessor: Trigger control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_ctl`]
module"]
pub type TR_CTL = crate::Reg<tr_ctl::TR_CTL_SPEC>;
#[doc = "Trigger control register"]
pub mod tr_ctl;
