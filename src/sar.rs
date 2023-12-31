#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    sample_ctrl: SAMPLE_CTRL,
    _reserved2: [u8; 0x08],
    sample_time01: SAMPLE_TIME01,
    sample_time23: SAMPLE_TIME23,
    range_thres: RANGE_THRES,
    range_cond: RANGE_COND,
    chan_en: CHAN_EN,
    start_ctrl: START_CTRL,
    _reserved8: [u8; 0x58],
    chan_config: [CHAN_CONFIG; 16],
    _reserved9: [u8; 0x40],
    chan_work: [CHAN_WORK; 16],
    _reserved10: [u8; 0x40],
    chan_result: [CHAN_RESULT; 16],
    _reserved11: [u8; 0x40],
    chan_work_updated: CHAN_WORK_UPDATED,
    chan_result_updated: CHAN_RESULT_UPDATED,
    chan_work_newvalue: CHAN_WORK_NEWVALUE,
    chan_result_newvalue: CHAN_RESULT_NEWVALUE,
    intr: INTR,
    intr_set: INTR_SET,
    intr_mask: INTR_MASK,
    intr_masked: INTR_MASKED,
    saturate_intr: SATURATE_INTR,
    saturate_intr_set: SATURATE_INTR_SET,
    saturate_intr_mask: SATURATE_INTR_MASK,
    saturate_intr_masked: SATURATE_INTR_MASKED,
    range_intr: RANGE_INTR,
    range_intr_set: RANGE_INTR_SET,
    range_intr_mask: RANGE_INTR_MASK,
    range_intr_masked: RANGE_INTR_MASKED,
    intr_cause: INTR_CAUSE,
    _reserved28: [u8; 0x3c],
    inj_chan_config: INJ_CHAN_CONFIG,
    _reserved29: [u8; 0x0c],
    inj_result: INJ_RESULT,
    _reserved30: [u8; 0x0c],
    status: STATUS,
    avg_stat: AVG_STAT,
    _reserved32: [u8; 0x58],
    mux_switch0: MUX_SWITCH0,
    mux_switch_clear0: MUX_SWITCH_CLEAR0,
    _reserved34: [u8; 0x38],
    mux_switch_ds_ctrl: MUX_SWITCH_DS_CTRL,
    mux_switch_sq_ctrl: MUX_SWITCH_SQ_CTRL,
    mux_switch_status: MUX_SWITCH_STATUS,
    _reserved37: [u8; 0x0bb4],
    ana_trim0: ANA_TRIM0,
    ana_trim1: ANA_TRIM1,
}
impl RegisterBlock {
    #[doc = "0x00 - Analog control register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Sample control register."]
    #[inline(always)]
    pub const fn sample_ctrl(&self) -> &SAMPLE_CTRL {
        &self.sample_ctrl
    }
    #[doc = "0x10 - Sample time specification ST0 and ST1"]
    #[inline(always)]
    pub const fn sample_time01(&self) -> &SAMPLE_TIME01 {
        &self.sample_time01
    }
    #[doc = "0x14 - Sample time specification ST2 and ST3"]
    #[inline(always)]
    pub const fn sample_time23(&self) -> &SAMPLE_TIME23 {
        &self.sample_time23
    }
    #[doc = "0x18 - Global range detect threshold register."]
    #[inline(always)]
    pub const fn range_thres(&self) -> &RANGE_THRES {
        &self.range_thres
    }
    #[doc = "0x1c - Global range detect mode register."]
    #[inline(always)]
    pub const fn range_cond(&self) -> &RANGE_COND {
        &self.range_cond
    }
    #[doc = "0x20 - Enable bits for the channels"]
    #[inline(always)]
    pub const fn chan_en(&self) -> &CHAN_EN {
        &self.chan_en
    }
    #[doc = "0x24 - Start control register (firmware trigger)."]
    #[inline(always)]
    pub const fn start_ctrl(&self) -> &START_CTRL {
        &self.start_ctrl
    }
    #[doc = "0x80..0xc0 - Channel configuration register."]
    #[inline(always)]
    pub const fn chan_config(&self, n: usize) -> &CHAN_CONFIG {
        &self.chan_config[n]
    }
    #[doc = "0x100..0x140 - Channel working data register"]
    #[inline(always)]
    pub const fn chan_work(&self, n: usize) -> &CHAN_WORK {
        &self.chan_work[n]
    }
    #[doc = "0x180..0x1c0 - Channel result data register"]
    #[inline(always)]
    pub const fn chan_result(&self, n: usize) -> &CHAN_RESULT {
        &self.chan_result[n]
    }
    #[doc = "0x200 - Channel working data register 'updated' bits"]
    #[inline(always)]
    pub const fn chan_work_updated(&self) -> &CHAN_WORK_UPDATED {
        &self.chan_work_updated
    }
    #[doc = "0x204 - Channel result data register 'updated' bits"]
    #[inline(always)]
    pub const fn chan_result_updated(&self) -> &CHAN_RESULT_UPDATED {
        &self.chan_result_updated
    }
    #[doc = "0x208 - Channel working data register 'new value' bits"]
    #[inline(always)]
    pub const fn chan_work_newvalue(&self) -> &CHAN_WORK_NEWVALUE {
        &self.chan_work_newvalue
    }
    #[doc = "0x20c - Channel result data register 'new value' bits"]
    #[inline(always)]
    pub const fn chan_result_newvalue(&self) -> &CHAN_RESULT_NEWVALUE {
        &self.chan_result_newvalue
    }
    #[doc = "0x210 - Interrupt request register."]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0x214 - Interrupt set request register"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &INTR_SET {
        &self.intr_set
    }
    #[doc = "0x218 - Interrupt mask register."]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &INTR_MASK {
        &self.intr_mask
    }
    #[doc = "0x21c - Interrupt masked request register"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &INTR_MASKED {
        &self.intr_masked
    }
    #[doc = "0x220 - Saturate interrupt request register."]
    #[inline(always)]
    pub const fn saturate_intr(&self) -> &SATURATE_INTR {
        &self.saturate_intr
    }
    #[doc = "0x224 - Saturate interrupt set request register"]
    #[inline(always)]
    pub const fn saturate_intr_set(&self) -> &SATURATE_INTR_SET {
        &self.saturate_intr_set
    }
    #[doc = "0x228 - Saturate interrupt mask register."]
    #[inline(always)]
    pub const fn saturate_intr_mask(&self) -> &SATURATE_INTR_MASK {
        &self.saturate_intr_mask
    }
    #[doc = "0x22c - Saturate interrupt masked request register"]
    #[inline(always)]
    pub const fn saturate_intr_masked(&self) -> &SATURATE_INTR_MASKED {
        &self.saturate_intr_masked
    }
    #[doc = "0x230 - Range detect interrupt request register."]
    #[inline(always)]
    pub const fn range_intr(&self) -> &RANGE_INTR {
        &self.range_intr
    }
    #[doc = "0x234 - Range detect interrupt set request register"]
    #[inline(always)]
    pub const fn range_intr_set(&self) -> &RANGE_INTR_SET {
        &self.range_intr_set
    }
    #[doc = "0x238 - Range detect interrupt mask register."]
    #[inline(always)]
    pub const fn range_intr_mask(&self) -> &RANGE_INTR_MASK {
        &self.range_intr_mask
    }
    #[doc = "0x23c - Range interrupt masked request register"]
    #[inline(always)]
    pub const fn range_intr_masked(&self) -> &RANGE_INTR_MASKED {
        &self.range_intr_masked
    }
    #[doc = "0x240 - Interrupt cause register"]
    #[inline(always)]
    pub const fn intr_cause(&self) -> &INTR_CAUSE {
        &self.intr_cause
    }
    #[doc = "0x280 - Injection channel configuration register."]
    #[inline(always)]
    pub const fn inj_chan_config(&self) -> &INJ_CHAN_CONFIG {
        &self.inj_chan_config
    }
    #[doc = "0x290 - Injection channel result register"]
    #[inline(always)]
    pub const fn inj_result(&self) -> &INJ_RESULT {
        &self.inj_result
    }
    #[doc = "0x2a0 - Current status of internal SAR registers (mostly for debug)"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x2a4 - Current averaging status (for debug)"]
    #[inline(always)]
    pub const fn avg_stat(&self) -> &AVG_STAT {
        &self.avg_stat
    }
    #[doc = "0x300 - SARMUX Firmware switch controls"]
    #[inline(always)]
    pub const fn mux_switch0(&self) -> &MUX_SWITCH0 {
        &self.mux_switch0
    }
    #[doc = "0x304 - SARMUX Firmware switch control clear"]
    #[inline(always)]
    pub const fn mux_switch_clear0(&self) -> &MUX_SWITCH_CLEAR0 {
        &self.mux_switch_clear0
    }
    #[doc = "0x340 - SARMUX switch DSI control"]
    #[inline(always)]
    pub const fn mux_switch_ds_ctrl(&self) -> &MUX_SWITCH_DS_CTRL {
        &self.mux_switch_ds_ctrl
    }
    #[doc = "0x344 - SARMUX switch Sar Sequencer control"]
    #[inline(always)]
    pub const fn mux_switch_sq_ctrl(&self) -> &MUX_SWITCH_SQ_CTRL {
        &self.mux_switch_sq_ctrl
    }
    #[doc = "0x348 - SARMUX switch status"]
    #[inline(always)]
    pub const fn mux_switch_status(&self) -> &MUX_SWITCH_STATUS {
        &self.mux_switch_status
    }
    #[doc = "0xf00 - Analog trim register."]
    #[inline(always)]
    pub const fn ana_trim0(&self) -> &ANA_TRIM0 {
        &self.ana_trim0
    }
    #[doc = "0xf04 - Analog trim register."]
    #[inline(always)]
    pub const fn ana_trim1(&self) -> &ANA_TRIM1 {
        &self.ana_trim1
    }
}
#[doc = "CTRL (rw) register accessor: Analog control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Analog control register."]
pub mod ctrl;
#[doc = "SAMPLE_CTRL (rw) register accessor: Sample control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sample_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sample_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sample_ctrl`]
module"]
pub type SAMPLE_CTRL = crate::Reg<sample_ctrl::SAMPLE_CTRL_SPEC>;
#[doc = "Sample control register."]
pub mod sample_ctrl;
#[doc = "SAMPLE_TIME01 (rw) register accessor: Sample time specification ST0 and ST1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sample_time01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sample_time01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sample_time01`]
module"]
pub type SAMPLE_TIME01 = crate::Reg<sample_time01::SAMPLE_TIME01_SPEC>;
#[doc = "Sample time specification ST0 and ST1"]
pub mod sample_time01;
#[doc = "SAMPLE_TIME23 (rw) register accessor: Sample time specification ST2 and ST3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sample_time23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sample_time23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sample_time23`]
module"]
pub type SAMPLE_TIME23 = crate::Reg<sample_time23::SAMPLE_TIME23_SPEC>;
#[doc = "Sample time specification ST2 and ST3"]
pub mod sample_time23;
#[doc = "RANGE_THRES (rw) register accessor: Global range detect threshold register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`range_thres::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`range_thres::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@range_thres`]
module"]
pub type RANGE_THRES = crate::Reg<range_thres::RANGE_THRES_SPEC>;
#[doc = "Global range detect threshold register."]
pub mod range_thres;
#[doc = "RANGE_COND (rw) register accessor: Global range detect mode register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`range_cond::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`range_cond::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@range_cond`]
module"]
pub type RANGE_COND = crate::Reg<range_cond::RANGE_COND_SPEC>;
#[doc = "Global range detect mode register."]
pub mod range_cond;
#[doc = "CHAN_EN (rw) register accessor: Enable bits for the channels\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chan_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chan_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chan_en`]
module"]
pub type CHAN_EN = crate::Reg<chan_en::CHAN_EN_SPEC>;
#[doc = "Enable bits for the channels"]
pub mod chan_en;
#[doc = "START_CTRL (rw) register accessor: Start control register (firmware trigger).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`start_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`start_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start_ctrl`]
module"]
pub type START_CTRL = crate::Reg<start_ctrl::START_CTRL_SPEC>;
#[doc = "Start control register (firmware trigger)."]
pub mod start_ctrl;
#[doc = "CHAN_CONFIG (rw) register accessor: Channel configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chan_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chan_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chan_config`]
module"]
pub type CHAN_CONFIG = crate::Reg<chan_config::CHAN_CONFIG_SPEC>;
#[doc = "Channel configuration register."]
pub mod chan_config;
#[doc = "CHAN_WORK (r) register accessor: Channel working data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chan_work::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chan_work`]
module"]
pub type CHAN_WORK = crate::Reg<chan_work::CHAN_WORK_SPEC>;
#[doc = "Channel working data register"]
pub mod chan_work;
#[doc = "CHAN_RESULT (r) register accessor: Channel result data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chan_result::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chan_result`]
module"]
pub type CHAN_RESULT = crate::Reg<chan_result::CHAN_RESULT_SPEC>;
#[doc = "Channel result data register"]
pub mod chan_result;
#[doc = "CHAN_WORK_UPDATED (r) register accessor: Channel working data register 'updated' bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chan_work_updated::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chan_work_updated`]
module"]
pub type CHAN_WORK_UPDATED = crate::Reg<chan_work_updated::CHAN_WORK_UPDATED_SPEC>;
#[doc = "Channel working data register 'updated' bits"]
pub mod chan_work_updated;
#[doc = "CHAN_RESULT_UPDATED (r) register accessor: Channel result data register 'updated' bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chan_result_updated::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chan_result_updated`]
module"]
pub type CHAN_RESULT_UPDATED = crate::Reg<chan_result_updated::CHAN_RESULT_UPDATED_SPEC>;
#[doc = "Channel result data register 'updated' bits"]
pub mod chan_result_updated;
#[doc = "CHAN_WORK_NEWVALUE (r) register accessor: Channel working data register 'new value' bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chan_work_newvalue::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chan_work_newvalue`]
module"]
pub type CHAN_WORK_NEWVALUE = crate::Reg<chan_work_newvalue::CHAN_WORK_NEWVALUE_SPEC>;
#[doc = "Channel working data register 'new value' bits"]
pub mod chan_work_newvalue;
#[doc = "CHAN_RESULT_NEWVALUE (r) register accessor: Channel result data register 'new value' bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chan_result_newvalue::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chan_result_newvalue`]
module"]
pub type CHAN_RESULT_NEWVALUE = crate::Reg<chan_result_newvalue::CHAN_RESULT_NEWVALUE_SPEC>;
#[doc = "Channel result data register 'new value' bits"]
pub mod chan_result_newvalue;
#[doc = "INTR (rw) register accessor: Interrupt request register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt request register."]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: Interrupt set request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_set`]
module"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt set request register"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: Interrupt mask register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mask`]
module"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt mask register."]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: Interrupt masked request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_masked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_masked`]
module"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt masked request register"]
pub mod intr_masked;
#[doc = "SATURATE_INTR (rw) register accessor: Saturate interrupt request register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saturate_intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saturate_intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saturate_intr`]
module"]
pub type SATURATE_INTR = crate::Reg<saturate_intr::SATURATE_INTR_SPEC>;
#[doc = "Saturate interrupt request register."]
pub mod saturate_intr;
#[doc = "SATURATE_INTR_SET (rw) register accessor: Saturate interrupt set request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saturate_intr_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saturate_intr_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saturate_intr_set`]
module"]
pub type SATURATE_INTR_SET = crate::Reg<saturate_intr_set::SATURATE_INTR_SET_SPEC>;
#[doc = "Saturate interrupt set request register"]
pub mod saturate_intr_set;
#[doc = "SATURATE_INTR_MASK (rw) register accessor: Saturate interrupt mask register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saturate_intr_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saturate_intr_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saturate_intr_mask`]
module"]
pub type SATURATE_INTR_MASK = crate::Reg<saturate_intr_mask::SATURATE_INTR_MASK_SPEC>;
#[doc = "Saturate interrupt mask register."]
pub mod saturate_intr_mask;
#[doc = "SATURATE_INTR_MASKED (r) register accessor: Saturate interrupt masked request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saturate_intr_masked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saturate_intr_masked`]
module"]
pub type SATURATE_INTR_MASKED = crate::Reg<saturate_intr_masked::SATURATE_INTR_MASKED_SPEC>;
#[doc = "Saturate interrupt masked request register"]
pub mod saturate_intr_masked;
#[doc = "RANGE_INTR (rw) register accessor: Range detect interrupt request register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`range_intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`range_intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@range_intr`]
module"]
pub type RANGE_INTR = crate::Reg<range_intr::RANGE_INTR_SPEC>;
#[doc = "Range detect interrupt request register."]
pub mod range_intr;
#[doc = "RANGE_INTR_SET (rw) register accessor: Range detect interrupt set request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`range_intr_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`range_intr_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@range_intr_set`]
module"]
pub type RANGE_INTR_SET = crate::Reg<range_intr_set::RANGE_INTR_SET_SPEC>;
#[doc = "Range detect interrupt set request register"]
pub mod range_intr_set;
#[doc = "RANGE_INTR_MASK (rw) register accessor: Range detect interrupt mask register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`range_intr_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`range_intr_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@range_intr_mask`]
module"]
pub type RANGE_INTR_MASK = crate::Reg<range_intr_mask::RANGE_INTR_MASK_SPEC>;
#[doc = "Range detect interrupt mask register."]
pub mod range_intr_mask;
#[doc = "RANGE_INTR_MASKED (r) register accessor: Range interrupt masked request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`range_intr_masked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@range_intr_masked`]
module"]
pub type RANGE_INTR_MASKED = crate::Reg<range_intr_masked::RANGE_INTR_MASKED_SPEC>;
#[doc = "Range interrupt masked request register"]
pub mod range_intr_masked;
#[doc = "INTR_CAUSE (r) register accessor: Interrupt cause register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_cause::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cause`]
module"]
pub type INTR_CAUSE = crate::Reg<intr_cause::INTR_CAUSE_SPEC>;
#[doc = "Interrupt cause register"]
pub mod intr_cause;
#[doc = "INJ_CHAN_CONFIG (rw) register accessor: Injection channel configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inj_chan_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inj_chan_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inj_chan_config`]
module"]
pub type INJ_CHAN_CONFIG = crate::Reg<inj_chan_config::INJ_CHAN_CONFIG_SPEC>;
#[doc = "Injection channel configuration register."]
pub mod inj_chan_config;
#[doc = "INJ_RESULT (r) register accessor: Injection channel result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inj_result::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inj_result`]
module"]
pub type INJ_RESULT = crate::Reg<inj_result::INJ_RESULT_SPEC>;
#[doc = "Injection channel result register"]
pub mod inj_result;
#[doc = "STATUS (r) register accessor: Current status of internal SAR registers (mostly for debug)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Current status of internal SAR registers (mostly for debug)"]
pub mod status;
#[doc = "AVG_STAT (r) register accessor: Current averaging status (for debug)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`avg_stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@avg_stat`]
module"]
pub type AVG_STAT = crate::Reg<avg_stat::AVG_STAT_SPEC>;
#[doc = "Current averaging status (for debug)"]
pub mod avg_stat;
#[doc = "MUX_SWITCH0 (rw) register accessor: SARMUX Firmware switch controls\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mux_switch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mux_switch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux_switch0`]
module"]
pub type MUX_SWITCH0 = crate::Reg<mux_switch0::MUX_SWITCH0_SPEC>;
#[doc = "SARMUX Firmware switch controls"]
pub mod mux_switch0;
#[doc = "MUX_SWITCH_CLEAR0 (rw) register accessor: SARMUX Firmware switch control clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mux_switch_clear0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mux_switch_clear0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux_switch_clear0`]
module"]
pub type MUX_SWITCH_CLEAR0 = crate::Reg<mux_switch_clear0::MUX_SWITCH_CLEAR0_SPEC>;
#[doc = "SARMUX Firmware switch control clear"]
pub mod mux_switch_clear0;
#[doc = "MUX_SWITCH_DS_CTRL (rw) register accessor: SARMUX switch DSI control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mux_switch_ds_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mux_switch_ds_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux_switch_ds_ctrl`]
module"]
pub type MUX_SWITCH_DS_CTRL = crate::Reg<mux_switch_ds_ctrl::MUX_SWITCH_DS_CTRL_SPEC>;
#[doc = "SARMUX switch DSI control"]
pub mod mux_switch_ds_ctrl;
#[doc = "MUX_SWITCH_SQ_CTRL (rw) register accessor: SARMUX switch Sar Sequencer control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mux_switch_sq_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mux_switch_sq_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux_switch_sq_ctrl`]
module"]
pub type MUX_SWITCH_SQ_CTRL = crate::Reg<mux_switch_sq_ctrl::MUX_SWITCH_SQ_CTRL_SPEC>;
#[doc = "SARMUX switch Sar Sequencer control"]
pub mod mux_switch_sq_ctrl;
#[doc = "MUX_SWITCH_STATUS (r) register accessor: SARMUX switch status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mux_switch_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux_switch_status`]
module"]
pub type MUX_SWITCH_STATUS = crate::Reg<mux_switch_status::MUX_SWITCH_STATUS_SPEC>;
#[doc = "SARMUX switch status"]
pub mod mux_switch_status;
#[doc = "ANA_TRIM0 (rw) register accessor: Analog trim register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_trim0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_trim0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_trim0`]
module"]
pub type ANA_TRIM0 = crate::Reg<ana_trim0::ANA_TRIM0_SPEC>;
#[doc = "Analog trim register."]
pub mod ana_trim0;
#[doc = "ANA_TRIM1 (rw) register accessor: Analog trim register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_trim1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_trim1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_trim1`]
module"]
pub type ANA_TRIM1 = crate::Reg<ana_trim1::ANA_TRIM1_SPEC>;
#[doc = "Analog trim register."]
pub mod ana_trim1;
