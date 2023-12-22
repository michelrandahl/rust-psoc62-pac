#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pwr_ctl: PWR_CTL,
    pwr_hibernate: PWR_HIBERNATE,
    pwr_lvd_ctl: PWR_LVD_CTL,
    _reserved3: [u8; 0x08],
    pwr_buck_ctl: PWR_BUCK_CTL,
    pwr_buck_ctl2: PWR_BUCK_CTL2,
    pwr_lvd_status: PWR_LVD_STATUS,
    _reserved6: [u8; 0x60],
    pwr_hib_data: [PWR_HIB_DATA; 16],
    _reserved7: [u8; 0xc0],
    wdt_ctl: WDT_CTL,
    wdt_cnt: WDT_CNT,
    wdt_match: WDT_MATCH,
    _reserved10: [u8; 0x74],
    mcwdt_struct: (),
    _reserved11: [u8; 0x0100],
    clk_dsi_select: [CLK_DSI_SELECT; 16],
    clk_path_select: [CLK_PATH_SELECT; 16],
    clk_root_select: [CLK_ROOT_SELECT; 16],
    _reserved14: [u8; 0x0140],
    clk_select: CLK_SELECT,
    clk_timer_ctl: CLK_TIMER_CTL,
    _reserved16: [u8; 0x04],
    clk_ilo_config: CLK_ILO_CONFIG,
    clk_imo_config: CLK_IMO_CONFIG,
    clk_output_fast: CLK_OUTPUT_FAST,
    clk_output_slow: CLK_OUTPUT_SLOW,
    clk_cal_cnt1: CLK_CAL_CNT1,
    clk_cal_cnt2: CLK_CAL_CNT2,
    _reserved22: [u8; 0x08],
    clk_eco_config: CLK_ECO_CONFIG,
    clk_eco_status: CLK_ECO_STATUS,
    _reserved24: [u8; 0x08],
    clk_pilo_config: CLK_PILO_CONFIG,
    _reserved25: [u8; 0x40],
    clk_fll_config: CLK_FLL_CONFIG,
    clk_fll_config2: CLK_FLL_CONFIG2,
    clk_fll_config3: CLK_FLL_CONFIG3,
    clk_fll_config4: CLK_FLL_CONFIG4,
    clk_fll_status: CLK_FLL_STATUS,
    _reserved30: [u8; 0x6c],
    clk_pll_config: [CLK_PLL_CONFIG; 15],
    _reserved31: [u8; 0x04],
    clk_pll_status: [CLK_PLL_STATUS; 15],
    _reserved32: [u8; 0x84],
    srss_intr: SRSS_INTR,
    srss_intr_set: SRSS_INTR_SET,
    srss_intr_mask: SRSS_INTR_MASK,
    srss_intr_masked: SRSS_INTR_MASKED,
    srss_intr_cfg: SRSS_INTR_CFG,
    _reserved37: [u8; 0xec],
    res_cause: RES_CAUSE,
    res_cause2: RES_CAUSE2,
    _reserved39: [u8; 0x76f8],
    pwr_trim_ref_ctl: PWR_TRIM_REF_CTL,
    pwr_trim_bodovp_ctl: PWR_TRIM_BODOVP_CTL,
    clk_trim_cco_ctl: CLK_TRIM_CCO_CTL,
    clk_trim_cco_ctl2: CLK_TRIM_CCO_CTL2,
    _reserved43: [u8; 0x20],
    pwr_trim_wake_ctl: PWR_TRIM_WAKE_CTL,
    _reserved44: [u8; 0x7fdc],
    pwr_trim_lvd_ctl: PWR_TRIM_LVD_CTL,
    _reserved45: [u8; 0x04],
    clk_trim_ilo_ctl: CLK_TRIM_ILO_CTL,
    pwr_trim_pwrsys_ctl: PWR_TRIM_PWRSYS_CTL,
    clk_trim_eco_ctl: CLK_TRIM_ECO_CTL,
    clk_trim_pilo_ctl: CLK_TRIM_PILO_CTL,
    clk_trim_pilo_ctl2: CLK_TRIM_PILO_CTL2,
    clk_trim_pilo_ctl3: CLK_TRIM_PILO_CTL3,
}
impl RegisterBlock {
    #[doc = "0x00 - Power Mode Control"]
    #[inline(always)]
    pub const fn pwr_ctl(&self) -> &PWR_CTL {
        &self.pwr_ctl
    }
    #[doc = "0x04 - HIBERNATE Mode Register"]
    #[inline(always)]
    pub const fn pwr_hibernate(&self) -> &PWR_HIBERNATE {
        &self.pwr_hibernate
    }
    #[doc = "0x08 - Low Voltage Detector (LVD) Configuration Register"]
    #[inline(always)]
    pub const fn pwr_lvd_ctl(&self) -> &PWR_LVD_CTL {
        &self.pwr_lvd_ctl
    }
    #[doc = "0x14 - Buck Control Register"]
    #[inline(always)]
    pub const fn pwr_buck_ctl(&self) -> &PWR_BUCK_CTL {
        &self.pwr_buck_ctl
    }
    #[doc = "0x18 - Buck Control Register 2"]
    #[inline(always)]
    pub const fn pwr_buck_ctl2(&self) -> &PWR_BUCK_CTL2 {
        &self.pwr_buck_ctl2
    }
    #[doc = "0x1c - Low Voltage Detector (LVD) Status Register"]
    #[inline(always)]
    pub const fn pwr_lvd_status(&self) -> &PWR_LVD_STATUS {
        &self.pwr_lvd_status
    }
    #[doc = "0x80..0xc0 - HIBERNATE Data Register"]
    #[inline(always)]
    pub const fn pwr_hib_data(&self, n: usize) -> &PWR_HIB_DATA {
        &self.pwr_hib_data[n]
    }
    #[doc = "0x180 - Watchdog Counter Control Register"]
    #[inline(always)]
    pub const fn wdt_ctl(&self) -> &WDT_CTL {
        &self.wdt_ctl
    }
    #[doc = "0x184 - Watchdog Counter Count Register"]
    #[inline(always)]
    pub const fn wdt_cnt(&self) -> &WDT_CNT {
        &self.wdt_cnt
    }
    #[doc = "0x188 - Watchdog Counter Match Register"]
    #[inline(always)]
    pub const fn wdt_match(&self) -> &WDT_MATCH {
        &self.wdt_match
    }
    #[doc = "0x200..0x258 - Multi-Counter Watchdog Timer"]
    #[inline(always)]
    pub const fn mcwdt_struct(&self, n: usize) -> &MCWDT_STRUCT {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(512)
                .add(64 * n)
                .cast()
        }
    }
    #[doc = "0x300..0x340 - Clock DSI Select Register"]
    #[inline(always)]
    pub const fn clk_dsi_select(&self, n: usize) -> &CLK_DSI_SELECT {
        &self.clk_dsi_select[n]
    }
    #[doc = "0x340..0x380 - Clock Path Select Register"]
    #[inline(always)]
    pub const fn clk_path_select(&self, n: usize) -> &CLK_PATH_SELECT {
        &self.clk_path_select[n]
    }
    #[doc = "0x380..0x3c0 - Clock Root Select Register"]
    #[inline(always)]
    pub const fn clk_root_select(&self, n: usize) -> &CLK_ROOT_SELECT {
        &self.clk_root_select[n]
    }
    #[doc = "0x500 - Clock selection register"]
    #[inline(always)]
    pub const fn clk_select(&self) -> &CLK_SELECT {
        &self.clk_select
    }
    #[doc = "0x504 - Timer Clock Control Register"]
    #[inline(always)]
    pub const fn clk_timer_ctl(&self) -> &CLK_TIMER_CTL {
        &self.clk_timer_ctl
    }
    #[doc = "0x50c - ILO Configuration"]
    #[inline(always)]
    pub const fn clk_ilo_config(&self) -> &CLK_ILO_CONFIG {
        &self.clk_ilo_config
    }
    #[doc = "0x510 - IMO Configuration"]
    #[inline(always)]
    pub const fn clk_imo_config(&self) -> &CLK_IMO_CONFIG {
        &self.clk_imo_config
    }
    #[doc = "0x514 - Fast Clock Output Select Register"]
    #[inline(always)]
    pub const fn clk_output_fast(&self) -> &CLK_OUTPUT_FAST {
        &self.clk_output_fast
    }
    #[doc = "0x518 - Slow Clock Output Select Register"]
    #[inline(always)]
    pub const fn clk_output_slow(&self) -> &CLK_OUTPUT_SLOW {
        &self.clk_output_slow
    }
    #[doc = "0x51c - Clock Calibration Counter 1"]
    #[inline(always)]
    pub const fn clk_cal_cnt1(&self) -> &CLK_CAL_CNT1 {
        &self.clk_cal_cnt1
    }
    #[doc = "0x520 - Clock Calibration Counter 2"]
    #[inline(always)]
    pub const fn clk_cal_cnt2(&self) -> &CLK_CAL_CNT2 {
        &self.clk_cal_cnt2
    }
    #[doc = "0x52c - ECO Configuration Register"]
    #[inline(always)]
    pub const fn clk_eco_config(&self) -> &CLK_ECO_CONFIG {
        &self.clk_eco_config
    }
    #[doc = "0x530 - ECO Status Register"]
    #[inline(always)]
    pub const fn clk_eco_status(&self) -> &CLK_ECO_STATUS {
        &self.clk_eco_status
    }
    #[doc = "0x53c - Precision ILO Configuration Register"]
    #[inline(always)]
    pub const fn clk_pilo_config(&self) -> &CLK_PILO_CONFIG {
        &self.clk_pilo_config
    }
    #[doc = "0x580 - FLL Configuration Register"]
    #[inline(always)]
    pub const fn clk_fll_config(&self) -> &CLK_FLL_CONFIG {
        &self.clk_fll_config
    }
    #[doc = "0x584 - FLL Configuration Register 2"]
    #[inline(always)]
    pub const fn clk_fll_config2(&self) -> &CLK_FLL_CONFIG2 {
        &self.clk_fll_config2
    }
    #[doc = "0x588 - FLL Configuration Register 3"]
    #[inline(always)]
    pub const fn clk_fll_config3(&self) -> &CLK_FLL_CONFIG3 {
        &self.clk_fll_config3
    }
    #[doc = "0x58c - FLL Configuration Register 4"]
    #[inline(always)]
    pub const fn clk_fll_config4(&self) -> &CLK_FLL_CONFIG4 {
        &self.clk_fll_config4
    }
    #[doc = "0x590 - FLL Status Register"]
    #[inline(always)]
    pub const fn clk_fll_status(&self) -> &CLK_FLL_STATUS {
        &self.clk_fll_status
    }
    #[doc = "0x600..0x63c - PLL Configuration Register"]
    #[inline(always)]
    pub const fn clk_pll_config(&self, n: usize) -> &CLK_PLL_CONFIG {
        &self.clk_pll_config[n]
    }
    #[doc = "0x640..0x67c - PLL Status Register"]
    #[inline(always)]
    pub const fn clk_pll_status(&self, n: usize) -> &CLK_PLL_STATUS {
        &self.clk_pll_status[n]
    }
    #[doc = "0x700 - SRSS Interrupt Register"]
    #[inline(always)]
    pub const fn srss_intr(&self) -> &SRSS_INTR {
        &self.srss_intr
    }
    #[doc = "0x704 - SRSS Interrupt Set Register"]
    #[inline(always)]
    pub const fn srss_intr_set(&self) -> &SRSS_INTR_SET {
        &self.srss_intr_set
    }
    #[doc = "0x708 - SRSS Interrupt Mask Register"]
    #[inline(always)]
    pub const fn srss_intr_mask(&self) -> &SRSS_INTR_MASK {
        &self.srss_intr_mask
    }
    #[doc = "0x70c - SRSS Interrupt Masked Register"]
    #[inline(always)]
    pub const fn srss_intr_masked(&self) -> &SRSS_INTR_MASKED {
        &self.srss_intr_masked
    }
    #[doc = "0x710 - SRSS Interrupt Configuration Register"]
    #[inline(always)]
    pub const fn srss_intr_cfg(&self) -> &SRSS_INTR_CFG {
        &self.srss_intr_cfg
    }
    #[doc = "0x800 - Reset Cause Observation Register"]
    #[inline(always)]
    pub const fn res_cause(&self) -> &RES_CAUSE {
        &self.res_cause
    }
    #[doc = "0x804 - Reset Cause Observation Register 2"]
    #[inline(always)]
    pub const fn res_cause2(&self) -> &RES_CAUSE2 {
        &self.res_cause2
    }
    #[doc = "0x7f00 - Reference Trim Register"]
    #[inline(always)]
    pub const fn pwr_trim_ref_ctl(&self) -> &PWR_TRIM_REF_CTL {
        &self.pwr_trim_ref_ctl
    }
    #[doc = "0x7f04 - BOD/OVP Trim Register"]
    #[inline(always)]
    pub const fn pwr_trim_bodovp_ctl(&self) -> &PWR_TRIM_BODOVP_CTL {
        &self.pwr_trim_bodovp_ctl
    }
    #[doc = "0x7f08 - CCO Trim Register"]
    #[inline(always)]
    pub const fn clk_trim_cco_ctl(&self) -> &CLK_TRIM_CCO_CTL {
        &self.clk_trim_cco_ctl
    }
    #[doc = "0x7f0c - CCO Trim Register 2"]
    #[inline(always)]
    pub const fn clk_trim_cco_ctl2(&self) -> &CLK_TRIM_CCO_CTL2 {
        &self.clk_trim_cco_ctl2
    }
    #[doc = "0x7f30 - Wakeup Trim Register"]
    #[inline(always)]
    pub const fn pwr_trim_wake_ctl(&self) -> &PWR_TRIM_WAKE_CTL {
        &self.pwr_trim_wake_ctl
    }
    #[doc = "0xff10 - LVD Trim Register"]
    #[inline(always)]
    pub const fn pwr_trim_lvd_ctl(&self) -> &PWR_TRIM_LVD_CTL {
        &self.pwr_trim_lvd_ctl
    }
    #[doc = "0xff18 - ILO Trim Register"]
    #[inline(always)]
    pub const fn clk_trim_ilo_ctl(&self) -> &CLK_TRIM_ILO_CTL {
        &self.clk_trim_ilo_ctl
    }
    #[doc = "0xff1c - Power System Trim Register"]
    #[inline(always)]
    pub const fn pwr_trim_pwrsys_ctl(&self) -> &PWR_TRIM_PWRSYS_CTL {
        &self.pwr_trim_pwrsys_ctl
    }
    #[doc = "0xff20 - ECO Trim Register"]
    #[inline(always)]
    pub const fn clk_trim_eco_ctl(&self) -> &CLK_TRIM_ECO_CTL {
        &self.clk_trim_eco_ctl
    }
    #[doc = "0xff24 - PILO Trim Register"]
    #[inline(always)]
    pub const fn clk_trim_pilo_ctl(&self) -> &CLK_TRIM_PILO_CTL {
        &self.clk_trim_pilo_ctl
    }
    #[doc = "0xff28 - PILO Trim Register 2"]
    #[inline(always)]
    pub const fn clk_trim_pilo_ctl2(&self) -> &CLK_TRIM_PILO_CTL2 {
        &self.clk_trim_pilo_ctl2
    }
    #[doc = "0xff2c - PILO Trim Register 3"]
    #[inline(always)]
    pub const fn clk_trim_pilo_ctl3(&self) -> &CLK_TRIM_PILO_CTL3 {
        &self.clk_trim_pilo_ctl3
    }
}
#[doc = "PWR_CTL (rw) register accessor: Power Mode Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_ctl`]
module"]
pub type PWR_CTL = crate::Reg<pwr_ctl::PWR_CTL_SPEC>;
#[doc = "Power Mode Control"]
pub mod pwr_ctl;
#[doc = "PWR_HIBERNATE (rw) register accessor: HIBERNATE Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_hibernate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_hibernate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_hibernate`]
module"]
pub type PWR_HIBERNATE = crate::Reg<pwr_hibernate::PWR_HIBERNATE_SPEC>;
#[doc = "HIBERNATE Mode Register"]
pub mod pwr_hibernate;
#[doc = "PWR_LVD_CTL (rw) register accessor: Low Voltage Detector (LVD) Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_lvd_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_lvd_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_lvd_ctl`]
module"]
pub type PWR_LVD_CTL = crate::Reg<pwr_lvd_ctl::PWR_LVD_CTL_SPEC>;
#[doc = "Low Voltage Detector (LVD) Configuration Register"]
pub mod pwr_lvd_ctl;
#[doc = "PWR_BUCK_CTL (rw) register accessor: Buck Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_buck_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_buck_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_buck_ctl`]
module"]
pub type PWR_BUCK_CTL = crate::Reg<pwr_buck_ctl::PWR_BUCK_CTL_SPEC>;
#[doc = "Buck Control Register"]
pub mod pwr_buck_ctl;
#[doc = "PWR_BUCK_CTL2 (rw) register accessor: Buck Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_buck_ctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_buck_ctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_buck_ctl2`]
module"]
pub type PWR_BUCK_CTL2 = crate::Reg<pwr_buck_ctl2::PWR_BUCK_CTL2_SPEC>;
#[doc = "Buck Control Register 2"]
pub mod pwr_buck_ctl2;
#[doc = "PWR_LVD_STATUS (r) register accessor: Low Voltage Detector (LVD) Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_lvd_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_lvd_status`]
module"]
pub type PWR_LVD_STATUS = crate::Reg<pwr_lvd_status::PWR_LVD_STATUS_SPEC>;
#[doc = "Low Voltage Detector (LVD) Status Register"]
pub mod pwr_lvd_status;
#[doc = "PWR_HIB_DATA (rw) register accessor: HIBERNATE Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_hib_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_hib_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_hib_data`]
module"]
pub type PWR_HIB_DATA = crate::Reg<pwr_hib_data::PWR_HIB_DATA_SPEC>;
#[doc = "HIBERNATE Data Register"]
pub mod pwr_hib_data;
#[doc = "WDT_CTL (rw) register accessor: Watchdog Counter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdt_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt_ctl`]
module"]
pub type WDT_CTL = crate::Reg<wdt_ctl::WDT_CTL_SPEC>;
#[doc = "Watchdog Counter Control Register"]
pub mod wdt_ctl;
#[doc = "WDT_CNT (rw) register accessor: Watchdog Counter Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdt_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt_cnt`]
module"]
pub type WDT_CNT = crate::Reg<wdt_cnt::WDT_CNT_SPEC>;
#[doc = "Watchdog Counter Count Register"]
pub mod wdt_cnt;
#[doc = "WDT_MATCH (rw) register accessor: Watchdog Counter Match Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt_match::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdt_match::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt_match`]
module"]
pub type WDT_MATCH = crate::Reg<wdt_match::WDT_MATCH_SPEC>;
#[doc = "Watchdog Counter Match Register"]
pub mod wdt_match;
#[doc = "Multi-Counter Watchdog Timer"]
pub use self::mcwdt_struct::MCWDT_STRUCT;
#[doc = r"Cluster"]
#[doc = "Multi-Counter Watchdog Timer"]
pub mod mcwdt_struct;
#[doc = "CLK_DSI_SELECT (rw) register accessor: Clock DSI Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_dsi_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_dsi_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_dsi_select`]
module"]
pub type CLK_DSI_SELECT = crate::Reg<clk_dsi_select::CLK_DSI_SELECT_SPEC>;
#[doc = "Clock DSI Select Register"]
pub mod clk_dsi_select;
#[doc = "CLK_PATH_SELECT (rw) register accessor: Clock Path Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_path_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_path_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_path_select`]
module"]
pub type CLK_PATH_SELECT = crate::Reg<clk_path_select::CLK_PATH_SELECT_SPEC>;
#[doc = "Clock Path Select Register"]
pub mod clk_path_select;
#[doc = "CLK_ROOT_SELECT (rw) register accessor: Clock Root Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_root_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_root_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_root_select`]
module"]
pub type CLK_ROOT_SELECT = crate::Reg<clk_root_select::CLK_ROOT_SELECT_SPEC>;
#[doc = "Clock Root Select Register"]
pub mod clk_root_select;
#[doc = "CLK_SELECT (rw) register accessor: Clock selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_select`]
module"]
pub type CLK_SELECT = crate::Reg<clk_select::CLK_SELECT_SPEC>;
#[doc = "Clock selection register"]
pub mod clk_select;
#[doc = "CLK_TIMER_CTL (rw) register accessor: Timer Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_timer_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_timer_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_timer_ctl`]
module"]
pub type CLK_TIMER_CTL = crate::Reg<clk_timer_ctl::CLK_TIMER_CTL_SPEC>;
#[doc = "Timer Clock Control Register"]
pub mod clk_timer_ctl;
#[doc = "CLK_ILO_CONFIG (rw) register accessor: ILO Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_ilo_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ilo_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ilo_config`]
module"]
pub type CLK_ILO_CONFIG = crate::Reg<clk_ilo_config::CLK_ILO_CONFIG_SPEC>;
#[doc = "ILO Configuration"]
pub mod clk_ilo_config;
#[doc = "CLK_IMO_CONFIG (rw) register accessor: IMO Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_imo_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_imo_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_imo_config`]
module"]
pub type CLK_IMO_CONFIG = crate::Reg<clk_imo_config::CLK_IMO_CONFIG_SPEC>;
#[doc = "IMO Configuration"]
pub mod clk_imo_config;
#[doc = "CLK_OUTPUT_FAST (rw) register accessor: Fast Clock Output Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_output_fast::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_output_fast::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_output_fast`]
module"]
pub type CLK_OUTPUT_FAST = crate::Reg<clk_output_fast::CLK_OUTPUT_FAST_SPEC>;
#[doc = "Fast Clock Output Select Register"]
pub mod clk_output_fast;
#[doc = "CLK_OUTPUT_SLOW (rw) register accessor: Slow Clock Output Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_output_slow::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_output_slow::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_output_slow`]
module"]
pub type CLK_OUTPUT_SLOW = crate::Reg<clk_output_slow::CLK_OUTPUT_SLOW_SPEC>;
#[doc = "Slow Clock Output Select Register"]
pub mod clk_output_slow;
#[doc = "CLK_CAL_CNT1 (rw) register accessor: Clock Calibration Counter 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_cal_cnt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_cal_cnt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_cal_cnt1`]
module"]
pub type CLK_CAL_CNT1 = crate::Reg<clk_cal_cnt1::CLK_CAL_CNT1_SPEC>;
#[doc = "Clock Calibration Counter 1"]
pub mod clk_cal_cnt1;
#[doc = "CLK_CAL_CNT2 (r) register accessor: Clock Calibration Counter 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_cal_cnt2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_cal_cnt2`]
module"]
pub type CLK_CAL_CNT2 = crate::Reg<clk_cal_cnt2::CLK_CAL_CNT2_SPEC>;
#[doc = "Clock Calibration Counter 2"]
pub mod clk_cal_cnt2;
#[doc = "CLK_ECO_CONFIG (rw) register accessor: ECO Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_eco_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_eco_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_eco_config`]
module"]
pub type CLK_ECO_CONFIG = crate::Reg<clk_eco_config::CLK_ECO_CONFIG_SPEC>;
#[doc = "ECO Configuration Register"]
pub mod clk_eco_config;
#[doc = "CLK_ECO_STATUS (r) register accessor: ECO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_eco_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_eco_status`]
module"]
pub type CLK_ECO_STATUS = crate::Reg<clk_eco_status::CLK_ECO_STATUS_SPEC>;
#[doc = "ECO Status Register"]
pub mod clk_eco_status;
#[doc = "CLK_PILO_CONFIG (rw) register accessor: Precision ILO Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pilo_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pilo_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pilo_config`]
module"]
pub type CLK_PILO_CONFIG = crate::Reg<clk_pilo_config::CLK_PILO_CONFIG_SPEC>;
#[doc = "Precision ILO Configuration Register"]
pub mod clk_pilo_config;
#[doc = "CLK_FLL_CONFIG (rw) register accessor: FLL Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_fll_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_fll_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_fll_config`]
module"]
pub type CLK_FLL_CONFIG = crate::Reg<clk_fll_config::CLK_FLL_CONFIG_SPEC>;
#[doc = "FLL Configuration Register"]
pub mod clk_fll_config;
#[doc = "CLK_FLL_CONFIG2 (rw) register accessor: FLL Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_fll_config2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_fll_config2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_fll_config2`]
module"]
pub type CLK_FLL_CONFIG2 = crate::Reg<clk_fll_config2::CLK_FLL_CONFIG2_SPEC>;
#[doc = "FLL Configuration Register 2"]
pub mod clk_fll_config2;
#[doc = "CLK_FLL_CONFIG3 (rw) register accessor: FLL Configuration Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_fll_config3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_fll_config3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_fll_config3`]
module"]
pub type CLK_FLL_CONFIG3 = crate::Reg<clk_fll_config3::CLK_FLL_CONFIG3_SPEC>;
#[doc = "FLL Configuration Register 3"]
pub mod clk_fll_config3;
#[doc = "CLK_FLL_CONFIG4 (rw) register accessor: FLL Configuration Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_fll_config4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_fll_config4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_fll_config4`]
module"]
pub type CLK_FLL_CONFIG4 = crate::Reg<clk_fll_config4::CLK_FLL_CONFIG4_SPEC>;
#[doc = "FLL Configuration Register 4"]
pub mod clk_fll_config4;
#[doc = "CLK_FLL_STATUS (rw) register accessor: FLL Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_fll_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_fll_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_fll_status`]
module"]
pub type CLK_FLL_STATUS = crate::Reg<clk_fll_status::CLK_FLL_STATUS_SPEC>;
#[doc = "FLL Status Register"]
pub mod clk_fll_status;
#[doc = "CLK_PLL_CONFIG (rw) register accessor: PLL Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pll_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pll_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pll_config`]
module"]
pub type CLK_PLL_CONFIG = crate::Reg<clk_pll_config::CLK_PLL_CONFIG_SPEC>;
#[doc = "PLL Configuration Register"]
pub mod clk_pll_config;
#[doc = "CLK_PLL_STATUS (rw) register accessor: PLL Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pll_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pll_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pll_status`]
module"]
pub type CLK_PLL_STATUS = crate::Reg<clk_pll_status::CLK_PLL_STATUS_SPEC>;
#[doc = "PLL Status Register"]
pub mod clk_pll_status;
#[doc = "SRSS_INTR (rw) register accessor: SRSS Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srss_intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srss_intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srss_intr`]
module"]
pub type SRSS_INTR = crate::Reg<srss_intr::SRSS_INTR_SPEC>;
#[doc = "SRSS Interrupt Register"]
pub mod srss_intr;
#[doc = "SRSS_INTR_SET (rw) register accessor: SRSS Interrupt Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srss_intr_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srss_intr_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srss_intr_set`]
module"]
pub type SRSS_INTR_SET = crate::Reg<srss_intr_set::SRSS_INTR_SET_SPEC>;
#[doc = "SRSS Interrupt Set Register"]
pub mod srss_intr_set;
#[doc = "SRSS_INTR_MASK (rw) register accessor: SRSS Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srss_intr_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srss_intr_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srss_intr_mask`]
module"]
pub type SRSS_INTR_MASK = crate::Reg<srss_intr_mask::SRSS_INTR_MASK_SPEC>;
#[doc = "SRSS Interrupt Mask Register"]
pub mod srss_intr_mask;
#[doc = "SRSS_INTR_MASKED (r) register accessor: SRSS Interrupt Masked Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srss_intr_masked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srss_intr_masked`]
module"]
pub type SRSS_INTR_MASKED = crate::Reg<srss_intr_masked::SRSS_INTR_MASKED_SPEC>;
#[doc = "SRSS Interrupt Masked Register"]
pub mod srss_intr_masked;
#[doc = "SRSS_INTR_CFG (rw) register accessor: SRSS Interrupt Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srss_intr_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srss_intr_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srss_intr_cfg`]
module"]
pub type SRSS_INTR_CFG = crate::Reg<srss_intr_cfg::SRSS_INTR_CFG_SPEC>;
#[doc = "SRSS Interrupt Configuration Register"]
pub mod srss_intr_cfg;
#[doc = "RES_CAUSE (rw) register accessor: Reset Cause Observation Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`res_cause::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`res_cause::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res_cause`]
module"]
pub type RES_CAUSE = crate::Reg<res_cause::RES_CAUSE_SPEC>;
#[doc = "Reset Cause Observation Register"]
pub mod res_cause;
#[doc = "RES_CAUSE2 (rw) register accessor: Reset Cause Observation Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`res_cause2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`res_cause2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res_cause2`]
module"]
pub type RES_CAUSE2 = crate::Reg<res_cause2::RES_CAUSE2_SPEC>;
#[doc = "Reset Cause Observation Register 2"]
pub mod res_cause2;
#[doc = "PWR_TRIM_REF_CTL (rw) register accessor: Reference Trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_trim_ref_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_trim_ref_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_trim_ref_ctl`]
module"]
pub type PWR_TRIM_REF_CTL = crate::Reg<pwr_trim_ref_ctl::PWR_TRIM_REF_CTL_SPEC>;
#[doc = "Reference Trim Register"]
pub mod pwr_trim_ref_ctl;
#[doc = "PWR_TRIM_BODOVP_CTL (rw) register accessor: BOD/OVP Trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_trim_bodovp_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_trim_bodovp_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_trim_bodovp_ctl`]
module"]
pub type PWR_TRIM_BODOVP_CTL = crate::Reg<pwr_trim_bodovp_ctl::PWR_TRIM_BODOVP_CTL_SPEC>;
#[doc = "BOD/OVP Trim Register"]
pub mod pwr_trim_bodovp_ctl;
#[doc = "CLK_TRIM_CCO_CTL (rw) register accessor: CCO Trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_trim_cco_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_trim_cco_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_trim_cco_ctl`]
module"]
pub type CLK_TRIM_CCO_CTL = crate::Reg<clk_trim_cco_ctl::CLK_TRIM_CCO_CTL_SPEC>;
#[doc = "CCO Trim Register"]
pub mod clk_trim_cco_ctl;
#[doc = "CLK_TRIM_CCO_CTL2 (rw) register accessor: CCO Trim Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_trim_cco_ctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_trim_cco_ctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_trim_cco_ctl2`]
module"]
pub type CLK_TRIM_CCO_CTL2 = crate::Reg<clk_trim_cco_ctl2::CLK_TRIM_CCO_CTL2_SPEC>;
#[doc = "CCO Trim Register 2"]
pub mod clk_trim_cco_ctl2;
#[doc = "PWR_TRIM_WAKE_CTL (rw) register accessor: Wakeup Trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_trim_wake_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_trim_wake_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_trim_wake_ctl`]
module"]
pub type PWR_TRIM_WAKE_CTL = crate::Reg<pwr_trim_wake_ctl::PWR_TRIM_WAKE_CTL_SPEC>;
#[doc = "Wakeup Trim Register"]
pub mod pwr_trim_wake_ctl;
#[doc = "PWR_TRIM_LVD_CTL (rw) register accessor: LVD Trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_trim_lvd_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_trim_lvd_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_trim_lvd_ctl`]
module"]
pub type PWR_TRIM_LVD_CTL = crate::Reg<pwr_trim_lvd_ctl::PWR_TRIM_LVD_CTL_SPEC>;
#[doc = "LVD Trim Register"]
pub mod pwr_trim_lvd_ctl;
#[doc = "CLK_TRIM_ILO_CTL (rw) register accessor: ILO Trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_trim_ilo_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_trim_ilo_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_trim_ilo_ctl`]
module"]
pub type CLK_TRIM_ILO_CTL = crate::Reg<clk_trim_ilo_ctl::CLK_TRIM_ILO_CTL_SPEC>;
#[doc = "ILO Trim Register"]
pub mod clk_trim_ilo_ctl;
#[doc = "PWR_TRIM_PWRSYS_CTL (rw) register accessor: Power System Trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_trim_pwrsys_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_trim_pwrsys_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_trim_pwrsys_ctl`]
module"]
pub type PWR_TRIM_PWRSYS_CTL = crate::Reg<pwr_trim_pwrsys_ctl::PWR_TRIM_PWRSYS_CTL_SPEC>;
#[doc = "Power System Trim Register"]
pub mod pwr_trim_pwrsys_ctl;
#[doc = "CLK_TRIM_ECO_CTL (rw) register accessor: ECO Trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_trim_eco_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_trim_eco_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_trim_eco_ctl`]
module"]
pub type CLK_TRIM_ECO_CTL = crate::Reg<clk_trim_eco_ctl::CLK_TRIM_ECO_CTL_SPEC>;
#[doc = "ECO Trim Register"]
pub mod clk_trim_eco_ctl;
#[doc = "CLK_TRIM_PILO_CTL (rw) register accessor: PILO Trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_trim_pilo_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_trim_pilo_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_trim_pilo_ctl`]
module"]
pub type CLK_TRIM_PILO_CTL = crate::Reg<clk_trim_pilo_ctl::CLK_TRIM_PILO_CTL_SPEC>;
#[doc = "PILO Trim Register"]
pub mod clk_trim_pilo_ctl;
#[doc = "CLK_TRIM_PILO_CTL2 (rw) register accessor: PILO Trim Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_trim_pilo_ctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_trim_pilo_ctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_trim_pilo_ctl2`]
module"]
pub type CLK_TRIM_PILO_CTL2 = crate::Reg<clk_trim_pilo_ctl2::CLK_TRIM_PILO_CTL2_SPEC>;
#[doc = "PILO Trim Register 2"]
pub mod clk_trim_pilo_ctl2;
#[doc = "CLK_TRIM_PILO_CTL3 (rw) register accessor: PILO Trim Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_trim_pilo_ctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_trim_pilo_ctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_trim_pilo_ctl3`]
module"]
pub type CLK_TRIM_PILO_CTL3 = crate::Reg<clk_trim_pilo_ctl3::CLK_TRIM_PILO_CTL3_SPEC>;
#[doc = "PILO Trim Register 3"]
pub mod clk_trim_pilo_ctl3;
