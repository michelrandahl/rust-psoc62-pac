#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctl: CTL,
    _reserved1: [u8; 0x04],
    rtc_rw: RTC_RW,
    cal_ctl: CAL_CTL,
    status: STATUS,
    rtc_time: RTC_TIME,
    rtc_date: RTC_DATE,
    alm1_time: ALM1_TIME,
    alm1_date: ALM1_DATE,
    alm2_time: ALM2_TIME,
    alm2_date: ALM2_DATE,
    intr: INTR,
    intr_set: INTR_SET,
    intr_mask: INTR_MASK,
    intr_masked: INTR_MASKED,
    osccnt: OSCCNT,
    ticks: TICKS,
    pmic_ctl: PMIC_CTL,
    reset: RESET,
    _reserved18: [u8; 0x0fb4],
    breg: [BREG; 64],
    _reserved19: [u8; 0xee00],
    trim: TRIM,
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &CTL {
        &self.ctl
    }
    #[doc = "0x08 - RTC Read Write register"]
    #[inline(always)]
    pub const fn rtc_rw(&self) -> &RTC_RW {
        &self.rtc_rw
    }
    #[doc = "0x0c - Oscillator calibration for absolute frequency"]
    #[inline(always)]
    pub const fn cal_ctl(&self) -> &CAL_CTL {
        &self.cal_ctl
    }
    #[doc = "0x10 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x14 - Calendar Seconds, Minutes, Hours, Day of Week"]
    #[inline(always)]
    pub const fn rtc_time(&self) -> &RTC_TIME {
        &self.rtc_time
    }
    #[doc = "0x18 - Calendar Day of Month, Month, Year"]
    #[inline(always)]
    pub const fn rtc_date(&self) -> &RTC_DATE {
        &self.rtc_date
    }
    #[doc = "0x1c - Alarm 1 Seconds, Minute, Hours, Day of Week"]
    #[inline(always)]
    pub const fn alm1_time(&self) -> &ALM1_TIME {
        &self.alm1_time
    }
    #[doc = "0x20 - Alarm 1 Day of Month, Month"]
    #[inline(always)]
    pub const fn alm1_date(&self) -> &ALM1_DATE {
        &self.alm1_date
    }
    #[doc = "0x24 - Alarm 2 Seconds, Minute, Hours, Day of Week"]
    #[inline(always)]
    pub const fn alm2_time(&self) -> &ALM2_TIME {
        &self.alm2_time
    }
    #[doc = "0x28 - Alarm 2 Day of Month, Month"]
    #[inline(always)]
    pub const fn alm2_date(&self) -> &ALM2_DATE {
        &self.alm2_date
    }
    #[doc = "0x2c - Interrupt request register"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0x30 - Interrupt set request register"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &INTR_SET {
        &self.intr_set
    }
    #[doc = "0x34 - Interrupt mask register"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &INTR_MASK {
        &self.intr_mask
    }
    #[doc = "0x38 - Interrupt masked request register"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &INTR_MASKED {
        &self.intr_masked
    }
    #[doc = "0x3c - 32kHz oscillator counter"]
    #[inline(always)]
    pub const fn osccnt(&self) -> &OSCCNT {
        &self.osccnt
    }
    #[doc = "0x40 - 128Hz tick counter"]
    #[inline(always)]
    pub const fn ticks(&self) -> &TICKS {
        &self.ticks
    }
    #[doc = "0x44 - PMIC control register"]
    #[inline(always)]
    pub const fn pmic_ctl(&self) -> &PMIC_CTL {
        &self.pmic_ctl
    }
    #[doc = "0x48 - Backup reset register"]
    #[inline(always)]
    pub const fn reset(&self) -> &RESET {
        &self.reset
    }
    #[doc = "0x1000..0x1100 - Backup register region"]
    #[inline(always)]
    pub const fn breg(&self, n: usize) -> &BREG {
        &self.breg[n]
    }
    #[doc = "0xff00 - Trim Register"]
    #[inline(always)]
    pub const fn trim(&self) -> &TRIM {
        &self.trim
    }
}
#[doc = "CTL (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "RTC_RW (rw) register accessor: RTC Read Write register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_rw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_rw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_rw`]
module"]
pub type RTC_RW = crate::Reg<rtc_rw::RTC_RW_SPEC>;
#[doc = "RTC Read Write register"]
pub mod rtc_rw;
#[doc = "CAL_CTL (rw) register accessor: Oscillator calibration for absolute frequency\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal_ctl`]
module"]
pub type CAL_CTL = crate::Reg<cal_ctl::CAL_CTL_SPEC>;
#[doc = "Oscillator calibration for absolute frequency"]
pub mod cal_ctl;
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "RTC_TIME (rw) register accessor: Calendar Seconds, Minutes, Hours, Day of Week\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_time`]
module"]
pub type RTC_TIME = crate::Reg<rtc_time::RTC_TIME_SPEC>;
#[doc = "Calendar Seconds, Minutes, Hours, Day of Week"]
pub mod rtc_time;
#[doc = "RTC_DATE (rw) register accessor: Calendar Day of Month, Month, Year\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_date`]
module"]
pub type RTC_DATE = crate::Reg<rtc_date::RTC_DATE_SPEC>;
#[doc = "Calendar Day of Month, Month, Year"]
pub mod rtc_date;
#[doc = "ALM1_TIME (rw) register accessor: Alarm 1 Seconds, Minute, Hours, Day of Week\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alm1_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alm1_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alm1_time`]
module"]
pub type ALM1_TIME = crate::Reg<alm1_time::ALM1_TIME_SPEC>;
#[doc = "Alarm 1 Seconds, Minute, Hours, Day of Week"]
pub mod alm1_time;
#[doc = "ALM1_DATE (rw) register accessor: Alarm 1 Day of Month, Month\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alm1_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alm1_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alm1_date`]
module"]
pub type ALM1_DATE = crate::Reg<alm1_date::ALM1_DATE_SPEC>;
#[doc = "Alarm 1 Day of Month, Month"]
pub mod alm1_date;
#[doc = "ALM2_TIME (rw) register accessor: Alarm 2 Seconds, Minute, Hours, Day of Week\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alm2_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alm2_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alm2_time`]
module"]
pub type ALM2_TIME = crate::Reg<alm2_time::ALM2_TIME_SPEC>;
#[doc = "Alarm 2 Seconds, Minute, Hours, Day of Week"]
pub mod alm2_time;
#[doc = "ALM2_DATE (rw) register accessor: Alarm 2 Day of Month, Month\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alm2_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alm2_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alm2_date`]
module"]
pub type ALM2_DATE = crate::Reg<alm2_date::ALM2_DATE_SPEC>;
#[doc = "Alarm 2 Day of Month, Month"]
pub mod alm2_date;
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
#[doc = "OSCCNT (r) register accessor: 32kHz oscillator counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osccnt`]
module"]
pub type OSCCNT = crate::Reg<osccnt::OSCCNT_SPEC>;
#[doc = "32kHz oscillator counter"]
pub mod osccnt;
#[doc = "TICKS (r) register accessor: 128Hz tick counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ticks::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ticks`]
module"]
pub type TICKS = crate::Reg<ticks::TICKS_SPEC>;
#[doc = "128Hz tick counter"]
pub mod ticks;
#[doc = "PMIC_CTL (rw) register accessor: PMIC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmic_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmic_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmic_ctl`]
module"]
pub type PMIC_CTL = crate::Reg<pmic_ctl::PMIC_CTL_SPEC>;
#[doc = "PMIC control register"]
pub mod pmic_ctl;
#[doc = "RESET (rw) register accessor: Backup reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset`]
module"]
pub type RESET = crate::Reg<reset::RESET_SPEC>;
#[doc = "Backup reset register"]
pub mod reset;
#[doc = "BREG (rw) register accessor: Backup register region\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`breg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`breg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@breg`]
module"]
pub type BREG = crate::Reg<breg::BREG_SPEC>;
#[doc = "Backup register region"]
pub mod breg;
#[doc = "TRIM (rw) register accessor: Trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trim`]
module"]
pub type TRIM = crate::Reg<trim::TRIM_SPEC>;
#[doc = "Trim Register"]
pub mod trim;
