#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    config: CONFIG,
    spare: SPARE,
    _reserved2: [u8; 0x78],
    status: STATUS,
    stat_seq: STAT_SEQ,
    stat_cnts: STAT_CNTS,
    stat_hcnt: STAT_HCNT,
    _reserved6: [u8; 0x40],
    result_val1: RESULT_VAL1,
    result_val2: RESULT_VAL2,
    _reserved8: [u8; 0x08],
    adc_res: ADC_RES,
    _reserved9: [u8; 0x0c],
    intr: INTR,
    intr_set: INTR_SET,
    intr_mask: INTR_MASK,
    intr_masked: INTR_MASKED,
    _reserved13: [u8; 0x80],
    hscmp: HSCMP,
    ambuf: AMBUF,
    refgen: REFGEN,
    csdcmp: CSDCMP,
    _reserved17: [u8; 0x60],
    sw_res: SW_RES,
    _reserved18: [u8; 0x0c],
    sense_period: SENSE_PERIOD,
    sense_duty: SENSE_DUTY,
    _reserved20: [u8; 0x78],
    sw_hs_p_sel: SW_HS_P_SEL,
    sw_hs_n_sel: SW_HS_N_SEL,
    sw_shield_sel: SW_SHIELD_SEL,
    _reserved23: [u8; 0x04],
    sw_amuxbuf_sel: SW_AMUXBUF_SEL,
    sw_byp_sel: SW_BYP_SEL,
    _reserved25: [u8; 0x08],
    sw_cmp_p_sel: SW_CMP_P_SEL,
    sw_cmp_n_sel: SW_CMP_N_SEL,
    sw_refgen_sel: SW_REFGEN_SEL,
    _reserved28: [u8; 0x04],
    sw_fw_mod_sel: SW_FW_MOD_SEL,
    sw_fw_tank_sel: SW_FW_TANK_SEL,
    _reserved30: [u8; 0x08],
    sw_dsi_sel: SW_DSI_SEL,
    _reserved31: [u8; 0x0c],
    io_sel: IO_SEL,
    _reserved32: [u8; 0x2c],
    seq_time: SEQ_TIME,
    _reserved33: [u8; 0x0c],
    seq_init_cnt: SEQ_INIT_CNT,
    seq_norm_cnt: SEQ_NORM_CNT,
    _reserved35: [u8; 0x08],
    adc_ctl: ADC_CTL,
    _reserved36: [u8; 0x1c],
    seq_start: SEQ_START,
    _reserved37: [u8; 0xbc],
    idaca: IDACA,
    _reserved38: [u8; 0xfc],
    idacb: IDACB,
}
impl RegisterBlock {
    #[doc = "0x00 - Configuration and Control"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    #[doc = "0x04 - Spare MMIO"]
    #[inline(always)]
    pub const fn spare(&self) -> &SPARE {
        &self.spare
    }
    #[doc = "0x80 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x84 - Current Sequencer status"]
    #[inline(always)]
    pub const fn stat_seq(&self) -> &STAT_SEQ {
        &self.stat_seq
    }
    #[doc = "0x88 - Current status counts"]
    #[inline(always)]
    pub const fn stat_cnts(&self) -> &STAT_CNTS {
        &self.stat_cnts
    }
    #[doc = "0x8c - Current count of the HSCMP counter"]
    #[inline(always)]
    pub const fn stat_hcnt(&self) -> &STAT_HCNT {
        &self.stat_hcnt
    }
    #[doc = "0xd0 - Result CSD/CSX accumulation counter value 1"]
    #[inline(always)]
    pub const fn result_val1(&self) -> &RESULT_VAL1 {
        &self.result_val1
    }
    #[doc = "0xd4 - Result CSX accumulation counter value 2"]
    #[inline(always)]
    pub const fn result_val2(&self) -> &RESULT_VAL2 {
        &self.result_val2
    }
    #[doc = "0xe0 - ADC measurement"]
    #[inline(always)]
    pub const fn adc_res(&self) -> &ADC_RES {
        &self.adc_res
    }
    #[doc = "0xf0 - CSD Interrupt Request Register"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0xf4 - CSD Interrupt set register"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &INTR_SET {
        &self.intr_set
    }
    #[doc = "0xf8 - CSD Interrupt mask register"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &INTR_MASK {
        &self.intr_mask
    }
    #[doc = "0xfc - CSD Interrupt masked register"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &INTR_MASKED {
        &self.intr_masked
    }
    #[doc = "0x180 - High Speed Comparator configuration"]
    #[inline(always)]
    pub const fn hscmp(&self) -> &HSCMP {
        &self.hscmp
    }
    #[doc = "0x184 - Reference Generator configuration"]
    #[inline(always)]
    pub const fn ambuf(&self) -> &AMBUF {
        &self.ambuf
    }
    #[doc = "0x188 - Reference Generator configuration"]
    #[inline(always)]
    pub const fn refgen(&self) -> &REFGEN {
        &self.refgen
    }
    #[doc = "0x18c - CSD Comparator configuration"]
    #[inline(always)]
    pub const fn csdcmp(&self) -> &CSDCMP {
        &self.csdcmp
    }
    #[doc = "0x1f0 - Switch Resistance configuration"]
    #[inline(always)]
    pub const fn sw_res(&self) -> &SW_RES {
        &self.sw_res
    }
    #[doc = "0x200 - Sense clock period"]
    #[inline(always)]
    pub const fn sense_period(&self) -> &SENSE_PERIOD {
        &self.sense_period
    }
    #[doc = "0x204 - Sense clock duty cycle"]
    #[inline(always)]
    pub const fn sense_duty(&self) -> &SENSE_DUTY {
        &self.sense_duty
    }
    #[doc = "0x280 - HSCMP Pos input switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_hs_p_sel(&self) -> &SW_HS_P_SEL {
        &self.sw_hs_p_sel
    }
    #[doc = "0x284 - HSCMP Neg input switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_hs_n_sel(&self) -> &SW_HS_N_SEL {
        &self.sw_hs_n_sel
    }
    #[doc = "0x288 - Shielding switches Waveform selection"]
    #[inline(always)]
    pub const fn sw_shield_sel(&self) -> &SW_SHIELD_SEL {
        &self.sw_shield_sel
    }
    #[doc = "0x290 - Amuxbuffer switches Waveform selection"]
    #[inline(always)]
    pub const fn sw_amuxbuf_sel(&self) -> &SW_AMUXBUF_SEL {
        &self.sw_amuxbuf_sel
    }
    #[doc = "0x294 - AMUXBUS bypass switches Waveform selection"]
    #[inline(always)]
    pub const fn sw_byp_sel(&self) -> &SW_BYP_SEL {
        &self.sw_byp_sel
    }
    #[doc = "0x2a0 - CSDCMP Pos Switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_cmp_p_sel(&self) -> &SW_CMP_P_SEL {
        &self.sw_cmp_p_sel
    }
    #[doc = "0x2a4 - CSDCMP Neg Switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_cmp_n_sel(&self) -> &SW_CMP_N_SEL {
        &self.sw_cmp_n_sel
    }
    #[doc = "0x2a8 - Reference Generator Switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_refgen_sel(&self) -> &SW_REFGEN_SEL {
        &self.sw_refgen_sel
    }
    #[doc = "0x2b0 - Full Wave Cmod Switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_fw_mod_sel(&self) -> &SW_FW_MOD_SEL {
        &self.sw_fw_mod_sel
    }
    #[doc = "0x2b4 - Full Wave Csh_tank Switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_fw_tank_sel(&self) -> &SW_FW_TANK_SEL {
        &self.sw_fw_tank_sel
    }
    #[doc = "0x2c0 - DSI output switch control Waveform selection"]
    #[inline(always)]
    pub const fn sw_dsi_sel(&self) -> &SW_DSI_SEL {
        &self.sw_dsi_sel
    }
    #[doc = "0x2d0 - IO output control Waveform selection"]
    #[inline(always)]
    pub const fn io_sel(&self) -> &IO_SEL {
        &self.io_sel
    }
    #[doc = "0x300 - Sequencer Timing"]
    #[inline(always)]
    pub const fn seq_time(&self) -> &SEQ_TIME {
        &self.seq_time
    }
    #[doc = "0x310 - Sequencer Initial conversion and sample counts"]
    #[inline(always)]
    pub const fn seq_init_cnt(&self) -> &SEQ_INIT_CNT {
        &self.seq_init_cnt
    }
    #[doc = "0x314 - Sequencer Normal conversion and sample counts"]
    #[inline(always)]
    pub const fn seq_norm_cnt(&self) -> &SEQ_NORM_CNT {
        &self.seq_norm_cnt
    }
    #[doc = "0x320 - ADC Control"]
    #[inline(always)]
    pub const fn adc_ctl(&self) -> &ADC_CTL {
        &self.adc_ctl
    }
    #[doc = "0x340 - Sequencer start"]
    #[inline(always)]
    pub const fn seq_start(&self) -> &SEQ_START {
        &self.seq_start
    }
    #[doc = "0x400 - IDACA Configuration"]
    #[inline(always)]
    pub const fn idaca(&self) -> &IDACA {
        &self.idaca
    }
    #[doc = "0x500 - IDACB Configuration"]
    #[inline(always)]
    pub const fn idacb(&self) -> &IDACB {
        &self.idacb
    }
}
#[doc = "CONFIG (rw) register accessor: Configuration and Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration and Control"]
pub mod config;
#[doc = "SPARE (rw) register accessor: Spare MMIO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spare::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spare::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spare`]
module"]
pub type SPARE = crate::Reg<spare::SPARE_SPEC>;
#[doc = "Spare MMIO"]
pub mod spare;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "STAT_SEQ (r) register accessor: Current Sequencer status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat_seq::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat_seq`]
module"]
pub type STAT_SEQ = crate::Reg<stat_seq::STAT_SEQ_SPEC>;
#[doc = "Current Sequencer status"]
pub mod stat_seq;
#[doc = "STAT_CNTS (r) register accessor: Current status counts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat_cnts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat_cnts`]
module"]
pub type STAT_CNTS = crate::Reg<stat_cnts::STAT_CNTS_SPEC>;
#[doc = "Current status counts"]
pub mod stat_cnts;
#[doc = "STAT_HCNT (r) register accessor: Current count of the HSCMP counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat_hcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat_hcnt`]
module"]
pub type STAT_HCNT = crate::Reg<stat_hcnt::STAT_HCNT_SPEC>;
#[doc = "Current count of the HSCMP counter"]
pub mod stat_hcnt;
#[doc = "RESULT_VAL1 (r) register accessor: Result CSD/CSX accumulation counter value 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`result_val1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result_val1`]
module"]
pub type RESULT_VAL1 = crate::Reg<result_val1::RESULT_VAL1_SPEC>;
#[doc = "Result CSD/CSX accumulation counter value 1"]
pub mod result_val1;
#[doc = "RESULT_VAL2 (r) register accessor: Result CSX accumulation counter value 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`result_val2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result_val2`]
module"]
pub type RESULT_VAL2 = crate::Reg<result_val2::RESULT_VAL2_SPEC>;
#[doc = "Result CSX accumulation counter value 2"]
pub mod result_val2;
#[doc = "ADC_RES (r) register accessor: ADC measurement\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_res::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_res`]
module"]
pub type ADC_RES = crate::Reg<adc_res::ADC_RES_SPEC>;
#[doc = "ADC measurement"]
pub mod adc_res;
#[doc = "INTR (rw) register accessor: CSD Interrupt Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "CSD Interrupt Request Register"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: CSD Interrupt set register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_set`]
module"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "CSD Interrupt set register"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: CSD Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mask`]
module"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "CSD Interrupt mask register"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: CSD Interrupt masked register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_masked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_masked`]
module"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "CSD Interrupt masked register"]
pub mod intr_masked;
#[doc = "HSCMP (rw) register accessor: High Speed Comparator configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hscmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hscmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hscmp`]
module"]
pub type HSCMP = crate::Reg<hscmp::HSCMP_SPEC>;
#[doc = "High Speed Comparator configuration"]
pub mod hscmp;
#[doc = "AMBUF (rw) register accessor: Reference Generator configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ambuf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ambuf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ambuf`]
module"]
pub type AMBUF = crate::Reg<ambuf::AMBUF_SPEC>;
#[doc = "Reference Generator configuration"]
pub mod ambuf;
#[doc = "REFGEN (rw) register accessor: Reference Generator configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`refgen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`refgen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@refgen`]
module"]
pub type REFGEN = crate::Reg<refgen::REFGEN_SPEC>;
#[doc = "Reference Generator configuration"]
pub mod refgen;
#[doc = "CSDCMP (rw) register accessor: CSD Comparator configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csdcmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csdcmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csdcmp`]
module"]
pub type CSDCMP = crate::Reg<csdcmp::CSDCMP_SPEC>;
#[doc = "CSD Comparator configuration"]
pub mod csdcmp;
#[doc = "SW_RES (rw) register accessor: Switch Resistance configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_res::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_res::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_res`]
module"]
pub type SW_RES = crate::Reg<sw_res::SW_RES_SPEC>;
#[doc = "Switch Resistance configuration"]
pub mod sw_res;
#[doc = "SENSE_PERIOD (rw) register accessor: Sense clock period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sense_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sense_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sense_period`]
module"]
pub type SENSE_PERIOD = crate::Reg<sense_period::SENSE_PERIOD_SPEC>;
#[doc = "Sense clock period"]
pub mod sense_period;
#[doc = "SENSE_DUTY (rw) register accessor: Sense clock duty cycle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sense_duty::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sense_duty::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sense_duty`]
module"]
pub type SENSE_DUTY = crate::Reg<sense_duty::SENSE_DUTY_SPEC>;
#[doc = "Sense clock duty cycle"]
pub mod sense_duty;
#[doc = "SW_HS_P_SEL (rw) register accessor: HSCMP Pos input switch Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_hs_p_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_hs_p_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_hs_p_sel`]
module"]
pub type SW_HS_P_SEL = crate::Reg<sw_hs_p_sel::SW_HS_P_SEL_SPEC>;
#[doc = "HSCMP Pos input switch Waveform selection"]
pub mod sw_hs_p_sel;
#[doc = "SW_HS_N_SEL (rw) register accessor: HSCMP Neg input switch Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_hs_n_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_hs_n_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_hs_n_sel`]
module"]
pub type SW_HS_N_SEL = crate::Reg<sw_hs_n_sel::SW_HS_N_SEL_SPEC>;
#[doc = "HSCMP Neg input switch Waveform selection"]
pub mod sw_hs_n_sel;
#[doc = "SW_SHIELD_SEL (rw) register accessor: Shielding switches Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_shield_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_shield_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_shield_sel`]
module"]
pub type SW_SHIELD_SEL = crate::Reg<sw_shield_sel::SW_SHIELD_SEL_SPEC>;
#[doc = "Shielding switches Waveform selection"]
pub mod sw_shield_sel;
#[doc = "SW_AMUXBUF_SEL (rw) register accessor: Amuxbuffer switches Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_amuxbuf_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_amuxbuf_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_amuxbuf_sel`]
module"]
pub type SW_AMUXBUF_SEL = crate::Reg<sw_amuxbuf_sel::SW_AMUXBUF_SEL_SPEC>;
#[doc = "Amuxbuffer switches Waveform selection"]
pub mod sw_amuxbuf_sel;
#[doc = "SW_BYP_SEL (rw) register accessor: AMUXBUS bypass switches Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_byp_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_byp_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_byp_sel`]
module"]
pub type SW_BYP_SEL = crate::Reg<sw_byp_sel::SW_BYP_SEL_SPEC>;
#[doc = "AMUXBUS bypass switches Waveform selection"]
pub mod sw_byp_sel;
#[doc = "SW_CMP_P_SEL (rw) register accessor: CSDCMP Pos Switch Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_cmp_p_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_cmp_p_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_cmp_p_sel`]
module"]
pub type SW_CMP_P_SEL = crate::Reg<sw_cmp_p_sel::SW_CMP_P_SEL_SPEC>;
#[doc = "CSDCMP Pos Switch Waveform selection"]
pub mod sw_cmp_p_sel;
#[doc = "SW_CMP_N_SEL (rw) register accessor: CSDCMP Neg Switch Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_cmp_n_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_cmp_n_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_cmp_n_sel`]
module"]
pub type SW_CMP_N_SEL = crate::Reg<sw_cmp_n_sel::SW_CMP_N_SEL_SPEC>;
#[doc = "CSDCMP Neg Switch Waveform selection"]
pub mod sw_cmp_n_sel;
#[doc = "SW_REFGEN_SEL (rw) register accessor: Reference Generator Switch Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_refgen_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_refgen_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_refgen_sel`]
module"]
pub type SW_REFGEN_SEL = crate::Reg<sw_refgen_sel::SW_REFGEN_SEL_SPEC>;
#[doc = "Reference Generator Switch Waveform selection"]
pub mod sw_refgen_sel;
#[doc = "SW_FW_MOD_SEL (rw) register accessor: Full Wave Cmod Switch Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_fw_mod_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_fw_mod_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_fw_mod_sel`]
module"]
pub type SW_FW_MOD_SEL = crate::Reg<sw_fw_mod_sel::SW_FW_MOD_SEL_SPEC>;
#[doc = "Full Wave Cmod Switch Waveform selection"]
pub mod sw_fw_mod_sel;
#[doc = "SW_FW_TANK_SEL (rw) register accessor: Full Wave Csh_tank Switch Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_fw_tank_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_fw_tank_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_fw_tank_sel`]
module"]
pub type SW_FW_TANK_SEL = crate::Reg<sw_fw_tank_sel::SW_FW_TANK_SEL_SPEC>;
#[doc = "Full Wave Csh_tank Switch Waveform selection"]
pub mod sw_fw_tank_sel;
#[doc = "SW_DSI_SEL (rw) register accessor: DSI output switch control Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_dsi_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_dsi_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_dsi_sel`]
module"]
pub type SW_DSI_SEL = crate::Reg<sw_dsi_sel::SW_DSI_SEL_SPEC>;
#[doc = "DSI output switch control Waveform selection"]
pub mod sw_dsi_sel;
#[doc = "IO_SEL (rw) register accessor: IO output control Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_sel`]
module"]
pub type IO_SEL = crate::Reg<io_sel::IO_SEL_SPEC>;
#[doc = "IO output control Waveform selection"]
pub mod io_sel;
#[doc = "SEQ_TIME (rw) register accessor: Sequencer Timing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_time`]
module"]
pub type SEQ_TIME = crate::Reg<seq_time::SEQ_TIME_SPEC>;
#[doc = "Sequencer Timing"]
pub mod seq_time;
#[doc = "SEQ_INIT_CNT (rw) register accessor: Sequencer Initial conversion and sample counts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_init_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_init_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_init_cnt`]
module"]
pub type SEQ_INIT_CNT = crate::Reg<seq_init_cnt::SEQ_INIT_CNT_SPEC>;
#[doc = "Sequencer Initial conversion and sample counts"]
pub mod seq_init_cnt;
#[doc = "SEQ_NORM_CNT (rw) register accessor: Sequencer Normal conversion and sample counts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_norm_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_norm_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_norm_cnt`]
module"]
pub type SEQ_NORM_CNT = crate::Reg<seq_norm_cnt::SEQ_NORM_CNT_SPEC>;
#[doc = "Sequencer Normal conversion and sample counts"]
pub mod seq_norm_cnt;
#[doc = "ADC_CTL (rw) register accessor: ADC Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_ctl`]
module"]
pub type ADC_CTL = crate::Reg<adc_ctl::ADC_CTL_SPEC>;
#[doc = "ADC Control"]
pub mod adc_ctl;
#[doc = "SEQ_START (rw) register accessor: Sequencer start\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_start`]
module"]
pub type SEQ_START = crate::Reg<seq_start::SEQ_START_SPEC>;
#[doc = "Sequencer start"]
pub mod seq_start;
#[doc = "IDACA (rw) register accessor: IDACA Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idaca::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idaca::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idaca`]
module"]
pub type IDACA = crate::Reg<idaca::IDACA_SPEC>;
#[doc = "IDACA Configuration"]
pub mod idaca;
#[doc = "IDACB (rw) register accessor: IDACB Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idacb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idacb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idacb`]
module"]
pub type IDACB = crate::Reg<idacb::IDACB_SPEC>;
#[doc = "IDACB Configuration"]
pub mod idacb;
