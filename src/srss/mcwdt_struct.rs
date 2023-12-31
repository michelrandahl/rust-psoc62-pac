#[doc = r"Register block"]
#[repr(C)]
pub struct MCWDT_STRUCT {
    _reserved0: [u8; 0x04],
    mcwdt_cntlow: MCWDT_CNTLOW,
    mcwdt_cnthigh: MCWDT_CNTHIGH,
    mcwdt_match: MCWDT_MATCH,
    mcwdt_config: MCWDT_CONFIG,
    mcwdt_ctl: MCWDT_CTL,
    mcwdt_intr: MCWDT_INTR,
    mcwdt_intr_set: MCWDT_INTR_SET,
    mcwdt_intr_mask: MCWDT_INTR_MASK,
    mcwdt_intr_masked: MCWDT_INTR_MASKED,
    mcwdt_lock: MCWDT_LOCK,
}
impl MCWDT_STRUCT {
    #[doc = "0x04 - Multi-Counter Watchdog Sub-counters 0/1"]
    #[inline(always)]
    pub const fn mcwdt_cntlow(&self) -> &MCWDT_CNTLOW {
        &self.mcwdt_cntlow
    }
    #[doc = "0x08 - Multi-Counter Watchdog Sub-counter 2"]
    #[inline(always)]
    pub const fn mcwdt_cnthigh(&self) -> &MCWDT_CNTHIGH {
        &self.mcwdt_cnthigh
    }
    #[doc = "0x0c - Multi-Counter Watchdog Counter Match Register"]
    #[inline(always)]
    pub const fn mcwdt_match(&self) -> &MCWDT_MATCH {
        &self.mcwdt_match
    }
    #[doc = "0x10 - Multi-Counter Watchdog Counter Configuration"]
    #[inline(always)]
    pub const fn mcwdt_config(&self) -> &MCWDT_CONFIG {
        &self.mcwdt_config
    }
    #[doc = "0x14 - Multi-Counter Watchdog Counter Control"]
    #[inline(always)]
    pub const fn mcwdt_ctl(&self) -> &MCWDT_CTL {
        &self.mcwdt_ctl
    }
    #[doc = "0x18 - Multi-Counter Watchdog Counter Interrupt Register"]
    #[inline(always)]
    pub const fn mcwdt_intr(&self) -> &MCWDT_INTR {
        &self.mcwdt_intr
    }
    #[doc = "0x1c - Multi-Counter Watchdog Counter Interrupt Set Register"]
    #[inline(always)]
    pub const fn mcwdt_intr_set(&self) -> &MCWDT_INTR_SET {
        &self.mcwdt_intr_set
    }
    #[doc = "0x20 - Multi-Counter Watchdog Counter Interrupt Mask Register"]
    #[inline(always)]
    pub const fn mcwdt_intr_mask(&self) -> &MCWDT_INTR_MASK {
        &self.mcwdt_intr_mask
    }
    #[doc = "0x24 - Multi-Counter Watchdog Counter Interrupt Masked Register"]
    #[inline(always)]
    pub const fn mcwdt_intr_masked(&self) -> &MCWDT_INTR_MASKED {
        &self.mcwdt_intr_masked
    }
    #[doc = "0x28 - Multi-Counter Watchdog Counter Lock Register"]
    #[inline(always)]
    pub const fn mcwdt_lock(&self) -> &MCWDT_LOCK {
        &self.mcwdt_lock
    }
}
#[doc = "MCWDT_CNTLOW (rw) register accessor: Multi-Counter Watchdog Sub-counters 0/1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcwdt_cntlow::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcwdt_cntlow::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcwdt_cntlow`]
module"]
pub type MCWDT_CNTLOW = crate::Reg<mcwdt_cntlow::MCWDT_CNTLOW_SPEC>;
#[doc = "Multi-Counter Watchdog Sub-counters 0/1"]
pub mod mcwdt_cntlow;
#[doc = "MCWDT_CNTHIGH (rw) register accessor: Multi-Counter Watchdog Sub-counter 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcwdt_cnthigh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcwdt_cnthigh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcwdt_cnthigh`]
module"]
pub type MCWDT_CNTHIGH = crate::Reg<mcwdt_cnthigh::MCWDT_CNTHIGH_SPEC>;
#[doc = "Multi-Counter Watchdog Sub-counter 2"]
pub mod mcwdt_cnthigh;
#[doc = "MCWDT_MATCH (rw) register accessor: Multi-Counter Watchdog Counter Match Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcwdt_match::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcwdt_match::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcwdt_match`]
module"]
pub type MCWDT_MATCH = crate::Reg<mcwdt_match::MCWDT_MATCH_SPEC>;
#[doc = "Multi-Counter Watchdog Counter Match Register"]
pub mod mcwdt_match;
#[doc = "MCWDT_CONFIG (rw) register accessor: Multi-Counter Watchdog Counter Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcwdt_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcwdt_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcwdt_config`]
module"]
pub type MCWDT_CONFIG = crate::Reg<mcwdt_config::MCWDT_CONFIG_SPEC>;
#[doc = "Multi-Counter Watchdog Counter Configuration"]
pub mod mcwdt_config;
#[doc = "MCWDT_CTL (rw) register accessor: Multi-Counter Watchdog Counter Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcwdt_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcwdt_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcwdt_ctl`]
module"]
pub type MCWDT_CTL = crate::Reg<mcwdt_ctl::MCWDT_CTL_SPEC>;
#[doc = "Multi-Counter Watchdog Counter Control"]
pub mod mcwdt_ctl;
#[doc = "MCWDT_INTR (rw) register accessor: Multi-Counter Watchdog Counter Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcwdt_intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcwdt_intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcwdt_intr`]
module"]
pub type MCWDT_INTR = crate::Reg<mcwdt_intr::MCWDT_INTR_SPEC>;
#[doc = "Multi-Counter Watchdog Counter Interrupt Register"]
pub mod mcwdt_intr;
#[doc = "MCWDT_INTR_SET (rw) register accessor: Multi-Counter Watchdog Counter Interrupt Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcwdt_intr_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcwdt_intr_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcwdt_intr_set`]
module"]
pub type MCWDT_INTR_SET = crate::Reg<mcwdt_intr_set::MCWDT_INTR_SET_SPEC>;
#[doc = "Multi-Counter Watchdog Counter Interrupt Set Register"]
pub mod mcwdt_intr_set;
#[doc = "MCWDT_INTR_MASK (rw) register accessor: Multi-Counter Watchdog Counter Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcwdt_intr_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcwdt_intr_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcwdt_intr_mask`]
module"]
pub type MCWDT_INTR_MASK = crate::Reg<mcwdt_intr_mask::MCWDT_INTR_MASK_SPEC>;
#[doc = "Multi-Counter Watchdog Counter Interrupt Mask Register"]
pub mod mcwdt_intr_mask;
#[doc = "MCWDT_INTR_MASKED (r) register accessor: Multi-Counter Watchdog Counter Interrupt Masked Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcwdt_intr_masked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcwdt_intr_masked`]
module"]
pub type MCWDT_INTR_MASKED = crate::Reg<mcwdt_intr_masked::MCWDT_INTR_MASKED_SPEC>;
#[doc = "Multi-Counter Watchdog Counter Interrupt Masked Register"]
pub mod mcwdt_intr_masked;
#[doc = "MCWDT_LOCK (rw) register accessor: Multi-Counter Watchdog Counter Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcwdt_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcwdt_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcwdt_lock`]
module"]
pub type MCWDT_LOCK = crate::Reg<mcwdt_lock::MCWDT_LOCK_SPEC>;
#[doc = "Multi-Counter Watchdog Counter Lock Register"]
pub mod mcwdt_lock;
