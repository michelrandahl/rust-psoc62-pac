#[doc = r"Register block"]
#[repr(C)]
pub struct AREF {
    aref_ctrl: AREF_CTRL,
}
impl AREF {
    #[doc = "0x00 - global AREF control"]
    #[inline(always)]
    pub const fn aref_ctrl(&self) -> &AREF_CTRL {
        &self.aref_ctrl
    }
}
#[doc = "AREF_CTRL (rw) register accessor: global AREF control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aref_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aref_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aref_ctrl`]
module"]
pub type AREF_CTRL = crate::Reg<aref_ctrl::AREF_CTRL_SPEC>;
#[doc = "global AREF control"]
pub mod aref_ctrl;
