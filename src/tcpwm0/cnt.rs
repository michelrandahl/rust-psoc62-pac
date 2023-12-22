#[doc = r"Register block"]
#[repr(C)]
pub struct CNT {
    ctrl: CTRL,
    status: STATUS,
    counter: COUNTER,
    cc: CC,
    cc_buff: CC_BUFF,
    period: PERIOD,
    period_buff: PERIOD_BUFF,
    _reserved7: [u8; 0x04],
    tr_ctrl0: TR_CTRL0,
    tr_ctrl1: TR_CTRL1,
    tr_ctrl2: TR_CTRL2,
    _reserved10: [u8; 0x04],
    intr: INTR,
    intr_set: INTR_SET,
    intr_mask: INTR_MASK,
    intr_masked: INTR_MASKED,
}
impl CNT {
    #[doc = "0x00 - Counter control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Counter status register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x08 - Counter count register"]
    #[inline(always)]
    pub const fn counter(&self) -> &COUNTER {
        &self.counter
    }
    #[doc = "0x0c - Counter compare/capture register"]
    #[inline(always)]
    pub const fn cc(&self) -> &CC {
        &self.cc
    }
    #[doc = "0x10 - Counter buffered compare/capture register"]
    #[inline(always)]
    pub const fn cc_buff(&self) -> &CC_BUFF {
        &self.cc_buff
    }
    #[doc = "0x14 - Counter period register"]
    #[inline(always)]
    pub const fn period(&self) -> &PERIOD {
        &self.period
    }
    #[doc = "0x18 - Counter buffered period register"]
    #[inline(always)]
    pub const fn period_buff(&self) -> &PERIOD_BUFF {
        &self.period_buff
    }
    #[doc = "0x20 - Counter trigger control register 0"]
    #[inline(always)]
    pub const fn tr_ctrl0(&self) -> &TR_CTRL0 {
        &self.tr_ctrl0
    }
    #[doc = "0x24 - Counter trigger control register 1"]
    #[inline(always)]
    pub const fn tr_ctrl1(&self) -> &TR_CTRL1 {
        &self.tr_ctrl1
    }
    #[doc = "0x28 - Counter trigger control register 2"]
    #[inline(always)]
    pub const fn tr_ctrl2(&self) -> &TR_CTRL2 {
        &self.tr_ctrl2
    }
    #[doc = "0x30 - Interrupt request register"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0x34 - Interrupt set request register"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &INTR_SET {
        &self.intr_set
    }
    #[doc = "0x38 - Interrupt mask register"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &INTR_MASK {
        &self.intr_mask
    }
    #[doc = "0x3c - Interrupt masked request register"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &INTR_MASKED {
        &self.intr_masked
    }
}
#[doc = "CTRL (rw) register accessor: Counter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Counter control register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: Counter status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Counter status register"]
pub mod status;
#[doc = "COUNTER (rw) register accessor: Counter count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`counter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`counter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@counter`]
module"]
pub type COUNTER = crate::Reg<counter::COUNTER_SPEC>;
#[doc = "Counter count register"]
pub mod counter;
#[doc = "CC (rw) register accessor: Counter compare/capture register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc`]
module"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "Counter compare/capture register"]
pub mod cc;
#[doc = "CC_BUFF (rw) register accessor: Counter buffered compare/capture register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_buff::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_buff::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_buff`]
module"]
pub type CC_BUFF = crate::Reg<cc_buff::CC_BUFF_SPEC>;
#[doc = "Counter buffered compare/capture register"]
pub mod cc_buff;
#[doc = "PERIOD (rw) register accessor: Counter period register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@period`]
module"]
pub type PERIOD = crate::Reg<period::PERIOD_SPEC>;
#[doc = "Counter period register"]
pub mod period;
#[doc = "PERIOD_BUFF (rw) register accessor: Counter buffered period register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`period_buff::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`period_buff::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@period_buff`]
module"]
pub type PERIOD_BUFF = crate::Reg<period_buff::PERIOD_BUFF_SPEC>;
#[doc = "Counter buffered period register"]
pub mod period_buff;
#[doc = "TR_CTRL0 (rw) register accessor: Counter trigger control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_ctrl0`]
module"]
pub type TR_CTRL0 = crate::Reg<tr_ctrl0::TR_CTRL0_SPEC>;
#[doc = "Counter trigger control register 0"]
pub mod tr_ctrl0;
#[doc = "TR_CTRL1 (rw) register accessor: Counter trigger control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_ctrl1`]
module"]
pub type TR_CTRL1 = crate::Reg<tr_ctrl1::TR_CTRL1_SPEC>;
#[doc = "Counter trigger control register 1"]
pub mod tr_ctrl1;
#[doc = "TR_CTRL2 (rw) register accessor: Counter trigger control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_ctrl2`]
module"]
pub type TR_CTRL2 = crate::Reg<tr_ctrl2::TR_CTRL2_SPEC>;
#[doc = "Counter trigger control register 2"]
pub mod tr_ctrl2;
#[doc = "INTR (rw) register accessor: Interrupt request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt request register"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: Interrupt set request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_set`]
module"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt set request register"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mask`]
module"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt mask register"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: Interrupt masked request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_masked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_masked`]
module"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt masked request register"]
pub mod intr_masked;
