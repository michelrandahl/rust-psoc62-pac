#[doc = r"Register block"]
#[repr(C)]
pub struct CORE {
    sdmasa_r: SDMASA_R,
    blocksize_r: BLOCKSIZE_R,
    blockcount_r: BLOCKCOUNT_R,
    argument_r: ARGUMENT_R,
    xfer_mode_r: XFER_MODE_R,
    cmd_r: CMD_R,
    resp01_r: RESP01_R,
    resp23_r: RESP23_R,
    resp45_r: RESP45_R,
    resp67_r: RESP67_R,
    buf_data_r: BUF_DATA_R,
    pstate_reg: PSTATE_REG,
    host_ctrl1_r: HOST_CTRL1_R,
    pwr_ctrl_r: PWR_CTRL_R,
    bgap_ctrl_r: BGAP_CTRL_R,
    wup_ctrl_r: WUP_CTRL_R,
    clk_ctrl_r: CLK_CTRL_R,
    tout_ctrl_r: TOUT_CTRL_R,
    sw_rst_r: SW_RST_R,
    normal_int_stat_r: NORMAL_INT_STAT_R,
    error_int_stat_r: ERROR_INT_STAT_R,
    normal_int_stat_en_r: NORMAL_INT_STAT_EN_R,
    error_int_stat_en_r: ERROR_INT_STAT_EN_R,
    normal_int_signal_en_r: NORMAL_INT_SIGNAL_EN_R,
    error_int_signal_en_r: ERROR_INT_SIGNAL_EN_R,
    auto_cmd_stat_r: AUTO_CMD_STAT_R,
    host_ctrl2_r: HOST_CTRL2_R,
    capabilities1_r: CAPABILITIES1_R,
    capabilities2_r: CAPABILITIES2_R,
    curr_capabilities1_r: CURR_CAPABILITIES1_R,
    curr_capabilities2_r: CURR_CAPABILITIES2_R,
    force_auto_cmd_stat_r: FORCE_AUTO_CMD_STAT_R,
    force_error_int_stat_r: FORCE_ERROR_INT_STAT_R,
    adma_err_stat_r: ADMA_ERR_STAT_R,
    _reserved34: [u8; 0x03],
    adma_sa_low_r: ADMA_SA_LOW_R,
    _reserved35: [u8; 0x1c],
    adma_id_low_r: ADMA_ID_LOW_R,
    _reserved36: [u8; 0x82],
    host_cntrl_vers_r: HOST_CNTRL_VERS_R,
    _reserved37: [u8; 0x80],
    cqver: CQVER,
    cqcap: CQCAP,
    cqcfg: CQCFG,
    cqctl: CQCTL,
    cqis: CQIS,
    cqise: CQISE,
    cqisge: CQISGE,
    cqic: CQIC,
    cqtdlba: CQTDLBA,
    _reserved46: [u8; 0x04],
    cqtdbr: CQTDBR,
    cqtcn: CQTCN,
    cqdqs: CQDQS,
    cqdpt: CQDPT,
    cqtclr: CQTCLR,
    _reserved51: [u8; 0x04],
    cqssc1: CQSSC1,
    cqssc2: CQSSC2,
    cqcrdct: CQCRDCT,
    _reserved54: [u8; 0x04],
    cqrmem: CQRMEM,
    cqterri: CQTERRI,
    cqcri: CQCRI,
    cqcra: CQCRA,
    _reserved58: [u8; 0x0320],
    mshc_ver_id_r: MSHC_VER_ID_R,
    mshc_ver_type_r: MSHC_VER_TYPE_R,
    mshc_ctrl_r: MSHC_CTRL_R,
    _reserved61: [u8; 0x07],
    mbiu_ctrl_r: MBIU_CTRL_R,
    _reserved62: [u8; 0x1b],
    emmc_ctrl_r: EMMC_CTRL_R,
    boot_ctrl_r: BOOT_CTRL_R,
    gp_in_r: GP_IN_R,
    gp_out_r: GP_OUT_R,
}
impl CORE {
    #[doc = "0x00 - SDMA System Address register"]
    #[inline(always)]
    pub const fn sdmasa_r(&self) -> &SDMASA_R {
        &self.sdmasa_r
    }
    #[doc = "0x04 - Block Size register"]
    #[inline(always)]
    pub const fn blocksize_r(&self) -> &BLOCKSIZE_R {
        &self.blocksize_r
    }
    #[doc = "0x06 - 16-bit Block Count register"]
    #[inline(always)]
    pub const fn blockcount_r(&self) -> &BLOCKCOUNT_R {
        &self.blockcount_r
    }
    #[doc = "0x08 - Argument register"]
    #[inline(always)]
    pub const fn argument_r(&self) -> &ARGUMENT_R {
        &self.argument_r
    }
    #[doc = "0x0c - Transfer Mode register"]
    #[inline(always)]
    pub const fn xfer_mode_r(&self) -> &XFER_MODE_R {
        &self.xfer_mode_r
    }
    #[doc = "0x0e - Command register"]
    #[inline(always)]
    pub const fn cmd_r(&self) -> &CMD_R {
        &self.cmd_r
    }
    #[doc = "0x10 - Response Register 0/1"]
    #[inline(always)]
    pub const fn resp01_r(&self) -> &RESP01_R {
        &self.resp01_r
    }
    #[doc = "0x14 - Response Register 2/3"]
    #[inline(always)]
    pub const fn resp23_r(&self) -> &RESP23_R {
        &self.resp23_r
    }
    #[doc = "0x18 - Response Register 4/5"]
    #[inline(always)]
    pub const fn resp45_r(&self) -> &RESP45_R {
        &self.resp45_r
    }
    #[doc = "0x1c - Response Register 6/7"]
    #[inline(always)]
    pub const fn resp67_r(&self) -> &RESP67_R {
        &self.resp67_r
    }
    #[doc = "0x20 - Buffer Data Port Register"]
    #[inline(always)]
    pub const fn buf_data_r(&self) -> &BUF_DATA_R {
        &self.buf_data_r
    }
    #[doc = "0x24 - Present State Register"]
    #[inline(always)]
    pub const fn pstate_reg(&self) -> &PSTATE_REG {
        &self.pstate_reg
    }
    #[doc = "0x28 - Host Control 1 Register"]
    #[inline(always)]
    pub const fn host_ctrl1_r(&self) -> &HOST_CTRL1_R {
        &self.host_ctrl1_r
    }
    #[doc = "0x29 - Power Control Register"]
    #[inline(always)]
    pub const fn pwr_ctrl_r(&self) -> &PWR_CTRL_R {
        &self.pwr_ctrl_r
    }
    #[doc = "0x2a - Block Gap Control Register"]
    #[inline(always)]
    pub const fn bgap_ctrl_r(&self) -> &BGAP_CTRL_R {
        &self.bgap_ctrl_r
    }
    #[doc = "0x2b - Wakeup Control Register"]
    #[inline(always)]
    pub const fn wup_ctrl_r(&self) -> &WUP_CTRL_R {
        &self.wup_ctrl_r
    }
    #[doc = "0x2c - Clock Control Register"]
    #[inline(always)]
    pub const fn clk_ctrl_r(&self) -> &CLK_CTRL_R {
        &self.clk_ctrl_r
    }
    #[doc = "0x2e - Timeout Control Register"]
    #[inline(always)]
    pub const fn tout_ctrl_r(&self) -> &TOUT_CTRL_R {
        &self.tout_ctrl_r
    }
    #[doc = "0x2f - Software Reset Register"]
    #[inline(always)]
    pub const fn sw_rst_r(&self) -> &SW_RST_R {
        &self.sw_rst_r
    }
    #[doc = "0x30 - Normal Interrupt Status Register"]
    #[inline(always)]
    pub const fn normal_int_stat_r(&self) -> &NORMAL_INT_STAT_R {
        &self.normal_int_stat_r
    }
    #[doc = "0x32 - Error Interrupt Status Register"]
    #[inline(always)]
    pub const fn error_int_stat_r(&self) -> &ERROR_INT_STAT_R {
        &self.error_int_stat_r
    }
    #[doc = "0x34 - Normal Interrupt Status Enable Register"]
    #[inline(always)]
    pub const fn normal_int_stat_en_r(&self) -> &NORMAL_INT_STAT_EN_R {
        &self.normal_int_stat_en_r
    }
    #[doc = "0x36 - Error Interrupt Status Enable Register"]
    #[inline(always)]
    pub const fn error_int_stat_en_r(&self) -> &ERROR_INT_STAT_EN_R {
        &self.error_int_stat_en_r
    }
    #[doc = "0x38 - Normal Interrupt Signal Enable Register"]
    #[inline(always)]
    pub const fn normal_int_signal_en_r(&self) -> &NORMAL_INT_SIGNAL_EN_R {
        &self.normal_int_signal_en_r
    }
    #[doc = "0x3a - Error Interrupt Signal Enable Register"]
    #[inline(always)]
    pub const fn error_int_signal_en_r(&self) -> &ERROR_INT_SIGNAL_EN_R {
        &self.error_int_signal_en_r
    }
    #[doc = "0x3c - Auto CMD Status Register"]
    #[inline(always)]
    pub const fn auto_cmd_stat_r(&self) -> &AUTO_CMD_STAT_R {
        &self.auto_cmd_stat_r
    }
    #[doc = "0x3e - Host Control 2 Register"]
    #[inline(always)]
    pub const fn host_ctrl2_r(&self) -> &HOST_CTRL2_R {
        &self.host_ctrl2_r
    }
    #[doc = "0x40 - Capabilities 1 Register - 0 to 31"]
    #[inline(always)]
    pub const fn capabilities1_r(&self) -> &CAPABILITIES1_R {
        &self.capabilities1_r
    }
    #[doc = "0x44 - Capabilities Register - 32 to 63"]
    #[inline(always)]
    pub const fn capabilities2_r(&self) -> &CAPABILITIES2_R {
        &self.capabilities2_r
    }
    #[doc = "0x48 - Current Capabilities Register - 0 to 31"]
    #[inline(always)]
    pub const fn curr_capabilities1_r(&self) -> &CURR_CAPABILITIES1_R {
        &self.curr_capabilities1_r
    }
    #[doc = "0x4c - Maximum Current Capabilities Register - 32 to 63"]
    #[inline(always)]
    pub const fn curr_capabilities2_r(&self) -> &CURR_CAPABILITIES2_R {
        &self.curr_capabilities2_r
    }
    #[doc = "0x50 - Force Event Register for Auto CMD Error Status register"]
    #[inline(always)]
    pub const fn force_auto_cmd_stat_r(&self) -> &FORCE_AUTO_CMD_STAT_R {
        &self.force_auto_cmd_stat_r
    }
    #[doc = "0x52 - Force Event Register for Error Interrupt Status"]
    #[inline(always)]
    pub const fn force_error_int_stat_r(&self) -> &FORCE_ERROR_INT_STAT_R {
        &self.force_error_int_stat_r
    }
    #[doc = "0x54 - ADMA Error Status Register"]
    #[inline(always)]
    pub const fn adma_err_stat_r(&self) -> &ADMA_ERR_STAT_R {
        &self.adma_err_stat_r
    }
    #[doc = "0x58 - ADMA System Address Register - Low"]
    #[inline(always)]
    pub const fn adma_sa_low_r(&self) -> &ADMA_SA_LOW_R {
        &self.adma_sa_low_r
    }
    #[doc = "0x78 - ADMA3 Integrated Descriptor Address Register - Low"]
    #[inline(always)]
    pub const fn adma_id_low_r(&self) -> &ADMA_ID_LOW_R {
        &self.adma_id_low_r
    }
    #[doc = "0xfe - Host Controller Version"]
    #[inline(always)]
    pub const fn host_cntrl_vers_r(&self) -> &HOST_CNTRL_VERS_R {
        &self.host_cntrl_vers_r
    }
    #[doc = "0x180 - Command Queuing Version register"]
    #[inline(always)]
    pub const fn cqver(&self) -> &CQVER {
        &self.cqver
    }
    #[doc = "0x184 - Command Queuing Capabilities register"]
    #[inline(always)]
    pub const fn cqcap(&self) -> &CQCAP {
        &self.cqcap
    }
    #[doc = "0x188 - Command Queuing Configuration register"]
    #[inline(always)]
    pub const fn cqcfg(&self) -> &CQCFG {
        &self.cqcfg
    }
    #[doc = "0x18c - Command Queuing Control register"]
    #[inline(always)]
    pub const fn cqctl(&self) -> &CQCTL {
        &self.cqctl
    }
    #[doc = "0x190 - Command Queuing Interrupt Status register"]
    #[inline(always)]
    pub const fn cqis(&self) -> &CQIS {
        &self.cqis
    }
    #[doc = "0x194 - Command Queuing Interrupt Status Enable register"]
    #[inline(always)]
    pub const fn cqise(&self) -> &CQISE {
        &self.cqise
    }
    #[doc = "0x198 - Command Queuing Interrupt signal enable register"]
    #[inline(always)]
    pub const fn cqisge(&self) -> &CQISGE {
        &self.cqisge
    }
    #[doc = "0x19c - Command Queuing Interrupt Coalescing register"]
    #[inline(always)]
    pub const fn cqic(&self) -> &CQIC {
        &self.cqic
    }
    #[doc = "0x1a0 - Command Queuing Task Descriptor List Base Address register"]
    #[inline(always)]
    pub const fn cqtdlba(&self) -> &CQTDLBA {
        &self.cqtdlba
    }
    #[doc = "0x1a8 - Command Queuing DoorBell register"]
    #[inline(always)]
    pub const fn cqtdbr(&self) -> &CQTDBR {
        &self.cqtdbr
    }
    #[doc = "0x1ac - Command Queuing TaskClear Notification register"]
    #[inline(always)]
    pub const fn cqtcn(&self) -> &CQTCN {
        &self.cqtcn
    }
    #[doc = "0x1b0 - Device queue status register"]
    #[inline(always)]
    pub const fn cqdqs(&self) -> &CQDQS {
        &self.cqdqs
    }
    #[doc = "0x1b4 - Device pending tasks register"]
    #[inline(always)]
    pub const fn cqdpt(&self) -> &CQDPT {
        &self.cqdpt
    }
    #[doc = "0x1b8 - Command Queuing DoorBell register"]
    #[inline(always)]
    pub const fn cqtclr(&self) -> &CQTCLR {
        &self.cqtclr
    }
    #[doc = "0x1c0 - CQ Send Status Configuration 1 register"]
    #[inline(always)]
    pub const fn cqssc1(&self) -> &CQSSC1 {
        &self.cqssc1
    }
    #[doc = "0x1c4 - CQ Send Status Configuration 2 register"]
    #[inline(always)]
    pub const fn cqssc2(&self) -> &CQSSC2 {
        &self.cqssc2
    }
    #[doc = "0x1c8 - Command response for direct command register"]
    #[inline(always)]
    pub const fn cqcrdct(&self) -> &CQCRDCT {
        &self.cqcrdct
    }
    #[doc = "0x1d0 - Command response mode error mask register"]
    #[inline(always)]
    pub const fn cqrmem(&self) -> &CQRMEM {
        &self.cqrmem
    }
    #[doc = "0x1d4 - CQ Task Error Information register"]
    #[inline(always)]
    pub const fn cqterri(&self) -> &CQTERRI {
        &self.cqterri
    }
    #[doc = "0x1d8 - CQ Command response index"]
    #[inline(always)]
    pub const fn cqcri(&self) -> &CQCRI {
        &self.cqcri
    }
    #[doc = "0x1dc - CQ Command response argument register"]
    #[inline(always)]
    pub const fn cqcra(&self) -> &CQCRA {
        &self.cqcra
    }
    #[doc = "0x500 - MSHC version"]
    #[inline(always)]
    pub const fn mshc_ver_id_r(&self) -> &MSHC_VER_ID_R {
        &self.mshc_ver_id_r
    }
    #[doc = "0x504 - MSHC version type"]
    #[inline(always)]
    pub const fn mshc_ver_type_r(&self) -> &MSHC_VER_TYPE_R {
        &self.mshc_ver_type_r
    }
    #[doc = "0x508 - MSHC Control register"]
    #[inline(always)]
    pub const fn mshc_ctrl_r(&self) -> &MSHC_CTRL_R {
        &self.mshc_ctrl_r
    }
    #[doc = "0x510 - MBIU Control register"]
    #[inline(always)]
    pub const fn mbiu_ctrl_r(&self) -> &MBIU_CTRL_R {
        &self.mbiu_ctrl_r
    }
    #[doc = "0x52c - eMMC Control register"]
    #[inline(always)]
    pub const fn emmc_ctrl_r(&self) -> &EMMC_CTRL_R {
        &self.emmc_ctrl_r
    }
    #[doc = "0x52e - eMMC Boot Control register"]
    #[inline(always)]
    pub const fn boot_ctrl_r(&self) -> &BOOT_CTRL_R {
        &self.boot_ctrl_r
    }
    #[doc = "0x530 - General Purpose Input register"]
    #[inline(always)]
    pub const fn gp_in_r(&self) -> &GP_IN_R {
        &self.gp_in_r
    }
    #[doc = "0x534 - General Purpose Output register"]
    #[inline(always)]
    pub const fn gp_out_r(&self) -> &GP_OUT_R {
        &self.gp_out_r
    }
}
#[doc = "SDMASA_R (rw) register accessor: SDMA System Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmasa_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmasa_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmasa_r`]
module"]
pub type SDMASA_R = crate::Reg<sdmasa_r::SDMASA_R_SPEC>;
#[doc = "SDMA System Address register"]
pub mod sdmasa_r;
#[doc = "BLOCKSIZE_R (rw) register accessor: Block Size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blocksize_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blocksize_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blocksize_r`]
module"]
pub type BLOCKSIZE_R = crate::Reg<blocksize_r::BLOCKSIZE_R_SPEC>;
#[doc = "Block Size register"]
pub mod blocksize_r;
#[doc = "BLOCKCOUNT_R (rw) register accessor: 16-bit Block Count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blockcount_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blockcount_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blockcount_r`]
module"]
pub type BLOCKCOUNT_R = crate::Reg<blockcount_r::BLOCKCOUNT_R_SPEC>;
#[doc = "16-bit Block Count register"]
pub mod blockcount_r;
#[doc = "ARGUMENT_R (rw) register accessor: Argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`argument_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`argument_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@argument_r`]
module"]
pub type ARGUMENT_R = crate::Reg<argument_r::ARGUMENT_R_SPEC>;
#[doc = "Argument register"]
pub mod argument_r;
#[doc = "XFER_MODE_R (rw) register accessor: Transfer Mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xfer_mode_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xfer_mode_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xfer_mode_r`]
module"]
pub type XFER_MODE_R = crate::Reg<xfer_mode_r::XFER_MODE_R_SPEC>;
#[doc = "Transfer Mode register"]
pub mod xfer_mode_r;
#[doc = "CMD_R (rw) register accessor: Command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_r`]
module"]
pub type CMD_R = crate::Reg<cmd_r::CMD_R_SPEC>;
#[doc = "Command register"]
pub mod cmd_r;
#[doc = "RESP01_R (r) register accessor: Response Register 0/1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp01_r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp01_r`]
module"]
pub type RESP01_R = crate::Reg<resp01_r::RESP01_R_SPEC>;
#[doc = "Response Register 0/1"]
pub mod resp01_r;
#[doc = "RESP23_R (r) register accessor: Response Register 2/3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp23_r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp23_r`]
module"]
pub type RESP23_R = crate::Reg<resp23_r::RESP23_R_SPEC>;
#[doc = "Response Register 2/3"]
pub mod resp23_r;
#[doc = "RESP45_R (r) register accessor: Response Register 4/5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp45_r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp45_r`]
module"]
pub type RESP45_R = crate::Reg<resp45_r::RESP45_R_SPEC>;
#[doc = "Response Register 4/5"]
pub mod resp45_r;
#[doc = "RESP67_R (r) register accessor: Response Register 6/7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp67_r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp67_r`]
module"]
pub type RESP67_R = crate::Reg<resp67_r::RESP67_R_SPEC>;
#[doc = "Response Register 6/7"]
pub mod resp67_r;
#[doc = "BUF_DATA_R (rw) register accessor: Buffer Data Port Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_data_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_data_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_data_r`]
module"]
pub type BUF_DATA_R = crate::Reg<buf_data_r::BUF_DATA_R_SPEC>;
#[doc = "Buffer Data Port Register"]
pub mod buf_data_r;
#[doc = "PSTATE_REG (r) register accessor: Present State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pstate_reg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pstate_reg`]
module"]
pub type PSTATE_REG = crate::Reg<pstate_reg::PSTATE_REG_SPEC>;
#[doc = "Present State Register"]
pub mod pstate_reg;
#[doc = "HOST_CTRL1_R (rw) register accessor: Host Control 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ctrl1_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ctrl1_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ctrl1_r`]
module"]
pub type HOST_CTRL1_R = crate::Reg<host_ctrl1_r::HOST_CTRL1_R_SPEC>;
#[doc = "Host Control 1 Register"]
pub mod host_ctrl1_r;
#[doc = "PWR_CTRL_R (rw) register accessor: Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_ctrl_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_ctrl_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_ctrl_r`]
module"]
pub type PWR_CTRL_R = crate::Reg<pwr_ctrl_r::PWR_CTRL_R_SPEC>;
#[doc = "Power Control Register"]
pub mod pwr_ctrl_r;
#[doc = "BGAP_CTRL_R (rw) register accessor: Block Gap Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgap_ctrl_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgap_ctrl_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgap_ctrl_r`]
module"]
pub type BGAP_CTRL_R = crate::Reg<bgap_ctrl_r::BGAP_CTRL_R_SPEC>;
#[doc = "Block Gap Control Register"]
pub mod bgap_ctrl_r;
#[doc = "WUP_CTRL_R (rw) register accessor: Wakeup Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wup_ctrl_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wup_ctrl_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wup_ctrl_r`]
module"]
pub type WUP_CTRL_R = crate::Reg<wup_ctrl_r::WUP_CTRL_R_SPEC>;
#[doc = "Wakeup Control Register"]
pub mod wup_ctrl_r;
#[doc = "CLK_CTRL_R (rw) register accessor: Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_ctrl_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ctrl_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ctrl_r`]
module"]
pub type CLK_CTRL_R = crate::Reg<clk_ctrl_r::CLK_CTRL_R_SPEC>;
#[doc = "Clock Control Register"]
pub mod clk_ctrl_r;
#[doc = "TOUT_CTRL_R (rw) register accessor: Timeout Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tout_ctrl_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tout_ctrl_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tout_ctrl_r`]
module"]
pub type TOUT_CTRL_R = crate::Reg<tout_ctrl_r::TOUT_CTRL_R_SPEC>;
#[doc = "Timeout Control Register"]
pub mod tout_ctrl_r;
#[doc = "SW_RST_R (rw) register accessor: Software Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_rst_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_rst_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_rst_r`]
module"]
pub type SW_RST_R = crate::Reg<sw_rst_r::SW_RST_R_SPEC>;
#[doc = "Software Reset Register"]
pub mod sw_rst_r;
#[doc = "NORMAL_INT_STAT_R (rw) register accessor: Normal Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`normal_int_stat_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`normal_int_stat_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@normal_int_stat_r`]
module"]
pub type NORMAL_INT_STAT_R = crate::Reg<normal_int_stat_r::NORMAL_INT_STAT_R_SPEC>;
#[doc = "Normal Interrupt Status Register"]
pub mod normal_int_stat_r;
#[doc = "ERROR_INT_STAT_R (rw) register accessor: Error Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`error_int_stat_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`error_int_stat_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_int_stat_r`]
module"]
pub type ERROR_INT_STAT_R = crate::Reg<error_int_stat_r::ERROR_INT_STAT_R_SPEC>;
#[doc = "Error Interrupt Status Register"]
pub mod error_int_stat_r;
#[doc = "NORMAL_INT_STAT_EN_R (rw) register accessor: Normal Interrupt Status Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`normal_int_stat_en_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`normal_int_stat_en_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@normal_int_stat_en_r`]
module"]
pub type NORMAL_INT_STAT_EN_R = crate::Reg<normal_int_stat_en_r::NORMAL_INT_STAT_EN_R_SPEC>;
#[doc = "Normal Interrupt Status Enable Register"]
pub mod normal_int_stat_en_r;
#[doc = "ERROR_INT_STAT_EN_R (rw) register accessor: Error Interrupt Status Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`error_int_stat_en_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`error_int_stat_en_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_int_stat_en_r`]
module"]
pub type ERROR_INT_STAT_EN_R = crate::Reg<error_int_stat_en_r::ERROR_INT_STAT_EN_R_SPEC>;
#[doc = "Error Interrupt Status Enable Register"]
pub mod error_int_stat_en_r;
#[doc = "NORMAL_INT_SIGNAL_EN_R (rw) register accessor: Normal Interrupt Signal Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`normal_int_signal_en_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`normal_int_signal_en_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@normal_int_signal_en_r`]
module"]
pub type NORMAL_INT_SIGNAL_EN_R = crate::Reg<normal_int_signal_en_r::NORMAL_INT_SIGNAL_EN_R_SPEC>;
#[doc = "Normal Interrupt Signal Enable Register"]
pub mod normal_int_signal_en_r;
#[doc = "ERROR_INT_SIGNAL_EN_R (rw) register accessor: Error Interrupt Signal Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`error_int_signal_en_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`error_int_signal_en_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_int_signal_en_r`]
module"]
pub type ERROR_INT_SIGNAL_EN_R = crate::Reg<error_int_signal_en_r::ERROR_INT_SIGNAL_EN_R_SPEC>;
#[doc = "Error Interrupt Signal Enable Register"]
pub mod error_int_signal_en_r;
#[doc = "AUTO_CMD_STAT_R (r) register accessor: Auto CMD Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auto_cmd_stat_r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auto_cmd_stat_r`]
module"]
pub type AUTO_CMD_STAT_R = crate::Reg<auto_cmd_stat_r::AUTO_CMD_STAT_R_SPEC>;
#[doc = "Auto CMD Status Register"]
pub mod auto_cmd_stat_r;
#[doc = "HOST_CTRL2_R (rw) register accessor: Host Control 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ctrl2_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ctrl2_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ctrl2_r`]
module"]
pub type HOST_CTRL2_R = crate::Reg<host_ctrl2_r::HOST_CTRL2_R_SPEC>;
#[doc = "Host Control 2 Register"]
pub mod host_ctrl2_r;
#[doc = "CAPABILITIES1_R (r) register accessor: Capabilities 1 Register - 0 to 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capabilities1_r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capabilities1_r`]
module"]
pub type CAPABILITIES1_R = crate::Reg<capabilities1_r::CAPABILITIES1_R_SPEC>;
#[doc = "Capabilities 1 Register - 0 to 31"]
pub mod capabilities1_r;
#[doc = "CAPABILITIES2_R (r) register accessor: Capabilities Register - 32 to 63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capabilities2_r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capabilities2_r`]
module"]
pub type CAPABILITIES2_R = crate::Reg<capabilities2_r::CAPABILITIES2_R_SPEC>;
#[doc = "Capabilities Register - 32 to 63"]
pub mod capabilities2_r;
#[doc = "CURR_CAPABILITIES1_R (r) register accessor: Current Capabilities Register - 0 to 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`curr_capabilities1_r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@curr_capabilities1_r`]
module"]
pub type CURR_CAPABILITIES1_R = crate::Reg<curr_capabilities1_r::CURR_CAPABILITIES1_R_SPEC>;
#[doc = "Current Capabilities Register - 0 to 31"]
pub mod curr_capabilities1_r;
#[doc = "CURR_CAPABILITIES2_R (r) register accessor: Maximum Current Capabilities Register - 32 to 63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`curr_capabilities2_r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@curr_capabilities2_r`]
module"]
pub type CURR_CAPABILITIES2_R = crate::Reg<curr_capabilities2_r::CURR_CAPABILITIES2_R_SPEC>;
#[doc = "Maximum Current Capabilities Register - 32 to 63"]
pub mod curr_capabilities2_r;
#[doc = "FORCE_AUTO_CMD_STAT_R (w) register accessor: Force Event Register for Auto CMD Error Status register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`force_auto_cmd_stat_r::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@force_auto_cmd_stat_r`]
module"]
pub type FORCE_AUTO_CMD_STAT_R = crate::Reg<force_auto_cmd_stat_r::FORCE_AUTO_CMD_STAT_R_SPEC>;
#[doc = "Force Event Register for Auto CMD Error Status register"]
pub mod force_auto_cmd_stat_r;
#[doc = "FORCE_ERROR_INT_STAT_R (rw) register accessor: Force Event Register for Error Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`force_error_int_stat_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`force_error_int_stat_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@force_error_int_stat_r`]
module"]
pub type FORCE_ERROR_INT_STAT_R = crate::Reg<force_error_int_stat_r::FORCE_ERROR_INT_STAT_R_SPEC>;
#[doc = "Force Event Register for Error Interrupt Status"]
pub mod force_error_int_stat_r;
#[doc = "ADMA_ERR_STAT_R (r) register accessor: ADMA Error Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adma_err_stat_r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adma_err_stat_r`]
module"]
pub type ADMA_ERR_STAT_R = crate::Reg<adma_err_stat_r::ADMA_ERR_STAT_R_SPEC>;
#[doc = "ADMA Error Status Register"]
pub mod adma_err_stat_r;
#[doc = "ADMA_SA_LOW_R (rw) register accessor: ADMA System Address Register - Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adma_sa_low_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adma_sa_low_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adma_sa_low_r`]
module"]
pub type ADMA_SA_LOW_R = crate::Reg<adma_sa_low_r::ADMA_SA_LOW_R_SPEC>;
#[doc = "ADMA System Address Register - Low"]
pub mod adma_sa_low_r;
#[doc = "ADMA_ID_LOW_R (rw) register accessor: ADMA3 Integrated Descriptor Address Register - Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adma_id_low_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adma_id_low_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adma_id_low_r`]
module"]
pub type ADMA_ID_LOW_R = crate::Reg<adma_id_low_r::ADMA_ID_LOW_R_SPEC>;
#[doc = "ADMA3 Integrated Descriptor Address Register - Low"]
pub mod adma_id_low_r;
#[doc = "HOST_CNTRL_VERS_R (r) register accessor: Host Controller Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_cntrl_vers_r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_cntrl_vers_r`]
module"]
pub type HOST_CNTRL_VERS_R = crate::Reg<host_cntrl_vers_r::HOST_CNTRL_VERS_R_SPEC>;
#[doc = "Host Controller Version"]
pub mod host_cntrl_vers_r;
#[doc = "CQVER (r) register accessor: Command Queuing Version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqver::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqver`]
module"]
pub type CQVER = crate::Reg<cqver::CQVER_SPEC>;
#[doc = "Command Queuing Version register"]
pub mod cqver;
#[doc = "CQCAP (r) register accessor: Command Queuing Capabilities register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqcap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqcap`]
module"]
pub type CQCAP = crate::Reg<cqcap::CQCAP_SPEC>;
#[doc = "Command Queuing Capabilities register"]
pub mod cqcap;
#[doc = "CQCFG (rw) register accessor: Command Queuing Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqcfg`]
module"]
pub type CQCFG = crate::Reg<cqcfg::CQCFG_SPEC>;
#[doc = "Command Queuing Configuration register"]
pub mod cqcfg;
#[doc = "CQCTL (rw) register accessor: Command Queuing Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqctl`]
module"]
pub type CQCTL = crate::Reg<cqctl::CQCTL_SPEC>;
#[doc = "Command Queuing Control register"]
pub mod cqctl;
#[doc = "CQIS (rw) register accessor: Command Queuing Interrupt Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqis`]
module"]
pub type CQIS = crate::Reg<cqis::CQIS_SPEC>;
#[doc = "Command Queuing Interrupt Status register"]
pub mod cqis;
#[doc = "CQISE (rw) register accessor: Command Queuing Interrupt Status Enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqise::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqise::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqise`]
module"]
pub type CQISE = crate::Reg<cqise::CQISE_SPEC>;
#[doc = "Command Queuing Interrupt Status Enable register"]
pub mod cqise;
#[doc = "CQISGE (rw) register accessor: Command Queuing Interrupt signal enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqisge::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqisge::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqisge`]
module"]
pub type CQISGE = crate::Reg<cqisge::CQISGE_SPEC>;
#[doc = "Command Queuing Interrupt signal enable register"]
pub mod cqisge;
#[doc = "CQIC (rw) register accessor: Command Queuing Interrupt Coalescing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqic::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqic::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqic`]
module"]
pub type CQIC = crate::Reg<cqic::CQIC_SPEC>;
#[doc = "Command Queuing Interrupt Coalescing register"]
pub mod cqic;
#[doc = "CQTDLBA (rw) register accessor: Command Queuing Task Descriptor List Base Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqtdlba::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqtdlba::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqtdlba`]
module"]
pub type CQTDLBA = crate::Reg<cqtdlba::CQTDLBA_SPEC>;
#[doc = "Command Queuing Task Descriptor List Base Address register"]
pub mod cqtdlba;
#[doc = "CQTDBR (rw) register accessor: Command Queuing DoorBell register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqtdbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqtdbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqtdbr`]
module"]
pub type CQTDBR = crate::Reg<cqtdbr::CQTDBR_SPEC>;
#[doc = "Command Queuing DoorBell register"]
pub mod cqtdbr;
#[doc = "CQTCN (rw) register accessor: Command Queuing TaskClear Notification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqtcn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqtcn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqtcn`]
module"]
pub type CQTCN = crate::Reg<cqtcn::CQTCN_SPEC>;
#[doc = "Command Queuing TaskClear Notification register"]
pub mod cqtcn;
#[doc = "CQDQS (r) register accessor: Device queue status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqdqs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqdqs`]
module"]
pub type CQDQS = crate::Reg<cqdqs::CQDQS_SPEC>;
#[doc = "Device queue status register"]
pub mod cqdqs;
#[doc = "CQDPT (r) register accessor: Device pending tasks register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqdpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqdpt`]
module"]
pub type CQDPT = crate::Reg<cqdpt::CQDPT_SPEC>;
#[doc = "Device pending tasks register"]
pub mod cqdpt;
#[doc = "CQTCLR (rw) register accessor: Command Queuing DoorBell register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqtclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqtclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqtclr`]
module"]
pub type CQTCLR = crate::Reg<cqtclr::CQTCLR_SPEC>;
#[doc = "Command Queuing DoorBell register"]
pub mod cqtclr;
#[doc = "CQSSC1 (rw) register accessor: CQ Send Status Configuration 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqssc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqssc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqssc1`]
module"]
pub type CQSSC1 = crate::Reg<cqssc1::CQSSC1_SPEC>;
#[doc = "CQ Send Status Configuration 1 register"]
pub mod cqssc1;
#[doc = "CQSSC2 (rw) register accessor: CQ Send Status Configuration 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqssc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqssc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqssc2`]
module"]
pub type CQSSC2 = crate::Reg<cqssc2::CQSSC2_SPEC>;
#[doc = "CQ Send Status Configuration 2 register"]
pub mod cqssc2;
#[doc = "CQCRDCT (r) register accessor: Command response for direct command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqcrdct::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqcrdct`]
module"]
pub type CQCRDCT = crate::Reg<cqcrdct::CQCRDCT_SPEC>;
#[doc = "Command response for direct command register"]
pub mod cqcrdct;
#[doc = "CQRMEM (rw) register accessor: Command response mode error mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqrmem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cqrmem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqrmem`]
module"]
pub type CQRMEM = crate::Reg<cqrmem::CQRMEM_SPEC>;
#[doc = "Command response mode error mask register"]
pub mod cqrmem;
#[doc = "CQTERRI (r) register accessor: CQ Task Error Information register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqterri::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqterri`]
module"]
pub type CQTERRI = crate::Reg<cqterri::CQTERRI_SPEC>;
#[doc = "CQ Task Error Information register"]
pub mod cqterri;
#[doc = "CQCRI (r) register accessor: CQ Command response index\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqcri::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqcri`]
module"]
pub type CQCRI = crate::Reg<cqcri::CQCRI_SPEC>;
#[doc = "CQ Command response index"]
pub mod cqcri;
#[doc = "CQCRA (r) register accessor: CQ Command response argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqcra::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqcra`]
module"]
pub type CQCRA = crate::Reg<cqcra::CQCRA_SPEC>;
#[doc = "CQ Command response argument register"]
pub mod cqcra;
#[doc = "MSHC_VER_ID_R (r) register accessor: MSHC version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mshc_ver_id_r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mshc_ver_id_r`]
module"]
pub type MSHC_VER_ID_R = crate::Reg<mshc_ver_id_r::MSHC_VER_ID_R_SPEC>;
#[doc = "MSHC version"]
pub mod mshc_ver_id_r;
#[doc = "MSHC_VER_TYPE_R (r) register accessor: MSHC version type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mshc_ver_type_r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mshc_ver_type_r`]
module"]
pub type MSHC_VER_TYPE_R = crate::Reg<mshc_ver_type_r::MSHC_VER_TYPE_R_SPEC>;
#[doc = "MSHC version type"]
pub mod mshc_ver_type_r;
#[doc = "MSHC_CTRL_R (rw) register accessor: MSHC Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mshc_ctrl_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mshc_ctrl_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mshc_ctrl_r`]
module"]
pub type MSHC_CTRL_R = crate::Reg<mshc_ctrl_r::MSHC_CTRL_R_SPEC>;
#[doc = "MSHC Control register"]
pub mod mshc_ctrl_r;
#[doc = "MBIU_CTRL_R (rw) register accessor: MBIU Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mbiu_ctrl_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mbiu_ctrl_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mbiu_ctrl_r`]
module"]
pub type MBIU_CTRL_R = crate::Reg<mbiu_ctrl_r::MBIU_CTRL_R_SPEC>;
#[doc = "MBIU Control register"]
pub mod mbiu_ctrl_r;
#[doc = "EMMC_CTRL_R (rw) register accessor: eMMC Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmc_ctrl_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmc_ctrl_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmc_ctrl_r`]
module"]
pub type EMMC_CTRL_R = crate::Reg<emmc_ctrl_r::EMMC_CTRL_R_SPEC>;
#[doc = "eMMC Control register"]
pub mod emmc_ctrl_r;
#[doc = "BOOT_CTRL_R (rw) register accessor: eMMC Boot Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot_ctrl_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot_ctrl_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot_ctrl_r`]
module"]
pub type BOOT_CTRL_R = crate::Reg<boot_ctrl_r::BOOT_CTRL_R_SPEC>;
#[doc = "eMMC Boot Control register"]
pub mod boot_ctrl_r;
#[doc = "GP_IN_R (r) register accessor: General Purpose Input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_in_r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_in_r`]
module"]
pub type GP_IN_R = crate::Reg<gp_in_r::GP_IN_R_SPEC>;
#[doc = "General Purpose Input register"]
pub mod gp_in_r;
#[doc = "GP_OUT_R (rw) register accessor: General Purpose Output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_out_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_out_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_out_r`]
module"]
pub type GP_OUT_R = crate::Reg<gp_out_r::GP_OUT_R_SPEC>;
#[doc = "General Purpose Output register"]
pub mod gp_out_r;
