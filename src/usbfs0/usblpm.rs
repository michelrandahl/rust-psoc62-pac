#[doc = r"Register block"]
#[repr(C)]
pub struct USBLPM {
    power_ctl: POWER_CTL,
    _reserved1: [u8; 0x04],
    usbio_ctl: USBIO_CTL,
    flow_ctl: FLOW_CTL,
    lpm_ctl: LPM_CTL,
    lpm_stat: LPM_STAT,
    _reserved5: [u8; 0x08],
    intr_sie: INTR_SIE,
    intr_sie_set: INTR_SIE_SET,
    intr_sie_mask: INTR_SIE_MASK,
    intr_sie_masked: INTR_SIE_MASKED,
    intr_lvl_sel: INTR_LVL_SEL,
    intr_cause_hi: INTR_CAUSE_HI,
    intr_cause_med: INTR_CAUSE_MED,
    intr_cause_lo: INTR_CAUSE_LO,
    _reserved13: [u8; 0x30],
    dft_ctl: DFT_CTL,
}
impl USBLPM {
    #[doc = "0x00 - Power Control Register"]
    #[inline(always)]
    pub const fn power_ctl(&self) -> &POWER_CTL {
        &self.power_ctl
    }
    #[doc = "0x08 - USB IO Control Register"]
    #[inline(always)]
    pub const fn usbio_ctl(&self) -> &USBIO_CTL {
        &self.usbio_ctl
    }
    #[doc = "0x0c - Flow Control Register"]
    #[inline(always)]
    pub const fn flow_ctl(&self) -> &FLOW_CTL {
        &self.flow_ctl
    }
    #[doc = "0x10 - LPM Control Register"]
    #[inline(always)]
    pub const fn lpm_ctl(&self) -> &LPM_CTL {
        &self.lpm_ctl
    }
    #[doc = "0x14 - LPM Status register"]
    #[inline(always)]
    pub const fn lpm_stat(&self) -> &LPM_STAT {
        &self.lpm_stat
    }
    #[doc = "0x20 - USB SOF, BUS RESET and EP0 Interrupt Status"]
    #[inline(always)]
    pub const fn intr_sie(&self) -> &INTR_SIE {
        &self.intr_sie
    }
    #[doc = "0x24 - USB SOF, BUS RESET and EP0 Interrupt Set"]
    #[inline(always)]
    pub const fn intr_sie_set(&self) -> &INTR_SIE_SET {
        &self.intr_sie_set
    }
    #[doc = "0x28 - USB SOF, BUS RESET and EP0 Interrupt Mask"]
    #[inline(always)]
    pub const fn intr_sie_mask(&self) -> &INTR_SIE_MASK {
        &self.intr_sie_mask
    }
    #[doc = "0x2c - USB SOF, BUS RESET and EP0 Interrupt Masked"]
    #[inline(always)]
    pub const fn intr_sie_masked(&self) -> &INTR_SIE_MASKED {
        &self.intr_sie_masked
    }
    #[doc = "0x30 - Select interrupt level for each interrupt source"]
    #[inline(always)]
    pub const fn intr_lvl_sel(&self) -> &INTR_LVL_SEL {
        &self.intr_lvl_sel
    }
    #[doc = "0x34 - High priority interrupt Cause register"]
    #[inline(always)]
    pub const fn intr_cause_hi(&self) -> &INTR_CAUSE_HI {
        &self.intr_cause_hi
    }
    #[doc = "0x38 - Medium priority interrupt Cause register"]
    #[inline(always)]
    pub const fn intr_cause_med(&self) -> &INTR_CAUSE_MED {
        &self.intr_cause_med
    }
    #[doc = "0x3c - Low priority interrupt Cause register"]
    #[inline(always)]
    pub const fn intr_cause_lo(&self) -> &INTR_CAUSE_LO {
        &self.intr_cause_lo
    }
    #[doc = "0x70 - DFT control"]
    #[inline(always)]
    pub const fn dft_ctl(&self) -> &DFT_CTL {
        &self.dft_ctl
    }
}
#[doc = "POWER_CTL (rw) register accessor: Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_ctl`]
module"]
pub type POWER_CTL = crate::Reg<power_ctl::POWER_CTL_SPEC>;
#[doc = "Power Control Register"]
pub mod power_ctl;
#[doc = "USBIO_CTL (rw) register accessor: USB IO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbio_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbio_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbio_ctl`]
module"]
pub type USBIO_CTL = crate::Reg<usbio_ctl::USBIO_CTL_SPEC>;
#[doc = "USB IO Control Register"]
pub mod usbio_ctl;
#[doc = "FLOW_CTL (rw) register accessor: Flow Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flow_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flow_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flow_ctl`]
module"]
pub type FLOW_CTL = crate::Reg<flow_ctl::FLOW_CTL_SPEC>;
#[doc = "Flow Control Register"]
pub mod flow_ctl;
#[doc = "LPM_CTL (rw) register accessor: LPM Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpm_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpm_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpm_ctl`]
module"]
pub type LPM_CTL = crate::Reg<lpm_ctl::LPM_CTL_SPEC>;
#[doc = "LPM Control Register"]
pub mod lpm_ctl;
#[doc = "LPM_STAT (r) register accessor: LPM Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpm_stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpm_stat`]
module"]
pub type LPM_STAT = crate::Reg<lpm_stat::LPM_STAT_SPEC>;
#[doc = "LPM Status register"]
pub mod lpm_stat;
#[doc = "INTR_SIE (rw) register accessor: USB SOF, BUS RESET and EP0 Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_sie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_sie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_sie`]
module"]
pub type INTR_SIE = crate::Reg<intr_sie::INTR_SIE_SPEC>;
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Status"]
pub mod intr_sie;
#[doc = "INTR_SIE_SET (rw) register accessor: USB SOF, BUS RESET and EP0 Interrupt Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_sie_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_sie_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_sie_set`]
module"]
pub type INTR_SIE_SET = crate::Reg<intr_sie_set::INTR_SIE_SET_SPEC>;
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Set"]
pub mod intr_sie_set;
#[doc = "INTR_SIE_MASK (rw) register accessor: USB SOF, BUS RESET and EP0 Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_sie_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_sie_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_sie_mask`]
module"]
pub type INTR_SIE_MASK = crate::Reg<intr_sie_mask::INTR_SIE_MASK_SPEC>;
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Mask"]
pub mod intr_sie_mask;
#[doc = "INTR_SIE_MASKED (r) register accessor: USB SOF, BUS RESET and EP0 Interrupt Masked\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_sie_masked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_sie_masked`]
module"]
pub type INTR_SIE_MASKED = crate::Reg<intr_sie_masked::INTR_SIE_MASKED_SPEC>;
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Masked"]
pub mod intr_sie_masked;
#[doc = "INTR_LVL_SEL (rw) register accessor: Select interrupt level for each interrupt source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_lvl_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_lvl_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_lvl_sel`]
module"]
pub type INTR_LVL_SEL = crate::Reg<intr_lvl_sel::INTR_LVL_SEL_SPEC>;
#[doc = "Select interrupt level for each interrupt source"]
pub mod intr_lvl_sel;
#[doc = "INTR_CAUSE_HI (r) register accessor: High priority interrupt Cause register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_cause_hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cause_hi`]
module"]
pub type INTR_CAUSE_HI = crate::Reg<intr_cause_hi::INTR_CAUSE_HI_SPEC>;
#[doc = "High priority interrupt Cause register"]
pub mod intr_cause_hi;
#[doc = "INTR_CAUSE_MED (r) register accessor: Medium priority interrupt Cause register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_cause_med::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cause_med`]
module"]
pub type INTR_CAUSE_MED = crate::Reg<intr_cause_med::INTR_CAUSE_MED_SPEC>;
#[doc = "Medium priority interrupt Cause register"]
pub mod intr_cause_med;
#[doc = "INTR_CAUSE_LO (r) register accessor: Low priority interrupt Cause register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_cause_lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cause_lo`]
module"]
pub type INTR_CAUSE_LO = crate::Reg<intr_cause_lo::INTR_CAUSE_LO_SPEC>;
#[doc = "Low priority interrupt Cause register"]
pub mod intr_cause_lo;
#[doc = "DFT_CTL (rw) register accessor: DFT control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dft_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dft_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dft_ctl`]
module"]
pub type DFT_CTL = crate::Reg<dft_ctl::DFT_CTL_SPEC>;
#[doc = "DFT control"]
pub mod dft_ctl;
