#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctl: CTL,
    status: STATUS,
    _reserved2: [u8; 0x18],
    act_descr_ctl: ACT_DESCR_CTL,
    act_descr_src: ACT_DESCR_SRC,
    act_descr_dst: ACT_DESCR_DST,
    _reserved5: [u8; 0x04],
    act_descr_x_ctl: ACT_DESCR_X_CTL,
    act_descr_y_ctl: ACT_DESCR_Y_CTL,
    act_descr_next_ptr: ACT_DESCR_NEXT_PTR,
    _reserved8: [u8; 0x04],
    act_src: ACT_SRC,
    act_dst: ACT_DST,
    _reserved10: [u8; 0x38],
    ecc_ctl: ECC_CTL,
    _reserved11: [u8; 0x7c],
    crc_ctl: CRC_CTL,
    _reserved12: [u8; 0x0c],
    crc_data_ctl: CRC_DATA_CTL,
    _reserved13: [u8; 0x0c],
    crc_pol_ctl: CRC_POL_CTL,
    _reserved14: [u8; 0x0c],
    crc_lfsr_ctl: CRC_LFSR_CTL,
    _reserved15: [u8; 0x0c],
    crc_rem_ctl: CRC_REM_CTL,
    _reserved16: [u8; 0x04],
    crc_rem_result: CRC_REM_RESULT,
    _reserved17: [u8; 0x7eb4],
    ch_struct: (),
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &CTL {
        &self.ctl
    }
    #[doc = "0x04 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x20 - Active descriptor control"]
    #[inline(always)]
    pub const fn act_descr_ctl(&self) -> &ACT_DESCR_CTL {
        &self.act_descr_ctl
    }
    #[doc = "0x24 - Active descriptor source"]
    #[inline(always)]
    pub const fn act_descr_src(&self) -> &ACT_DESCR_SRC {
        &self.act_descr_src
    }
    #[doc = "0x28 - Active descriptor destination"]
    #[inline(always)]
    pub const fn act_descr_dst(&self) -> &ACT_DESCR_DST {
        &self.act_descr_dst
    }
    #[doc = "0x30 - Active descriptor X loop control"]
    #[inline(always)]
    pub const fn act_descr_x_ctl(&self) -> &ACT_DESCR_X_CTL {
        &self.act_descr_x_ctl
    }
    #[doc = "0x34 - Active descriptor Y loop control"]
    #[inline(always)]
    pub const fn act_descr_y_ctl(&self) -> &ACT_DESCR_Y_CTL {
        &self.act_descr_y_ctl
    }
    #[doc = "0x38 - Active descriptor next pointer"]
    #[inline(always)]
    pub const fn act_descr_next_ptr(&self) -> &ACT_DESCR_NEXT_PTR {
        &self.act_descr_next_ptr
    }
    #[doc = "0x40 - Active source"]
    #[inline(always)]
    pub const fn act_src(&self) -> &ACT_SRC {
        &self.act_src
    }
    #[doc = "0x44 - Active destination"]
    #[inline(always)]
    pub const fn act_dst(&self) -> &ACT_DST {
        &self.act_dst
    }
    #[doc = "0x80 - ECC control"]
    #[inline(always)]
    pub const fn ecc_ctl(&self) -> &ECC_CTL {
        &self.ecc_ctl
    }
    #[doc = "0x100 - CRC control"]
    #[inline(always)]
    pub const fn crc_ctl(&self) -> &CRC_CTL {
        &self.crc_ctl
    }
    #[doc = "0x110 - CRC data control"]
    #[inline(always)]
    pub const fn crc_data_ctl(&self) -> &CRC_DATA_CTL {
        &self.crc_data_ctl
    }
    #[doc = "0x120 - CRC polynomial control"]
    #[inline(always)]
    pub const fn crc_pol_ctl(&self) -> &CRC_POL_CTL {
        &self.crc_pol_ctl
    }
    #[doc = "0x130 - CRC LFSR control"]
    #[inline(always)]
    pub const fn crc_lfsr_ctl(&self) -> &CRC_LFSR_CTL {
        &self.crc_lfsr_ctl
    }
    #[doc = "0x140 - CRC remainder control"]
    #[inline(always)]
    pub const fn crc_rem_ctl(&self) -> &CRC_REM_CTL {
        &self.crc_rem_ctl
    }
    #[doc = "0x148 - CRC remainder result"]
    #[inline(always)]
    pub const fn crc_rem_result(&self) -> &CRC_REM_RESULT {
        &self.crc_rem_result
    }
    #[doc = "0x8000..0x84fc - DW channel structure"]
    #[inline(always)]
    pub const fn ch_struct(&self, n: usize) -> &CH_STRUCT {
        #[allow(clippy::no_effect)]
        [(); 29][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(32768)
                .add(64 * n)
                .cast()
        }
    }
}
#[doc = "CTL (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "ACT_DESCR_CTL (r) register accessor: Active descriptor control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`act_descr_ctl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@act_descr_ctl`]
module"]
pub type ACT_DESCR_CTL = crate::Reg<act_descr_ctl::ACT_DESCR_CTL_SPEC>;
#[doc = "Active descriptor control"]
pub mod act_descr_ctl;
#[doc = "ACT_DESCR_SRC (r) register accessor: Active descriptor source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`act_descr_src::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@act_descr_src`]
module"]
pub type ACT_DESCR_SRC = crate::Reg<act_descr_src::ACT_DESCR_SRC_SPEC>;
#[doc = "Active descriptor source"]
pub mod act_descr_src;
#[doc = "ACT_DESCR_DST (r) register accessor: Active descriptor destination\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`act_descr_dst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@act_descr_dst`]
module"]
pub type ACT_DESCR_DST = crate::Reg<act_descr_dst::ACT_DESCR_DST_SPEC>;
#[doc = "Active descriptor destination"]
pub mod act_descr_dst;
#[doc = "ACT_DESCR_X_CTL (r) register accessor: Active descriptor X loop control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`act_descr_x_ctl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@act_descr_x_ctl`]
module"]
pub type ACT_DESCR_X_CTL = crate::Reg<act_descr_x_ctl::ACT_DESCR_X_CTL_SPEC>;
#[doc = "Active descriptor X loop control"]
pub mod act_descr_x_ctl;
#[doc = "ACT_DESCR_Y_CTL (r) register accessor: Active descriptor Y loop control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`act_descr_y_ctl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@act_descr_y_ctl`]
module"]
pub type ACT_DESCR_Y_CTL = crate::Reg<act_descr_y_ctl::ACT_DESCR_Y_CTL_SPEC>;
#[doc = "Active descriptor Y loop control"]
pub mod act_descr_y_ctl;
#[doc = "ACT_DESCR_NEXT_PTR (r) register accessor: Active descriptor next pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`act_descr_next_ptr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@act_descr_next_ptr`]
module"]
pub type ACT_DESCR_NEXT_PTR = crate::Reg<act_descr_next_ptr::ACT_DESCR_NEXT_PTR_SPEC>;
#[doc = "Active descriptor next pointer"]
pub mod act_descr_next_ptr;
#[doc = "ACT_SRC (r) register accessor: Active source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`act_src::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@act_src`]
module"]
pub type ACT_SRC = crate::Reg<act_src::ACT_SRC_SPEC>;
#[doc = "Active source"]
pub mod act_src;
#[doc = "ACT_DST (r) register accessor: Active destination\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`act_dst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@act_dst`]
module"]
pub type ACT_DST = crate::Reg<act_dst::ACT_DST_SPEC>;
#[doc = "Active destination"]
pub mod act_dst;
#[doc = "ECC_CTL (rw) register accessor: ECC control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_ctl`]
module"]
pub type ECC_CTL = crate::Reg<ecc_ctl::ECC_CTL_SPEC>;
#[doc = "ECC control"]
pub mod ecc_ctl;
#[doc = "CRC_CTL (rw) register accessor: CRC control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_ctl`]
module"]
pub type CRC_CTL = crate::Reg<crc_ctl::CRC_CTL_SPEC>;
#[doc = "CRC control"]
pub mod crc_ctl;
#[doc = "CRC_DATA_CTL (rw) register accessor: CRC data control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_data_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_data_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_data_ctl`]
module"]
pub type CRC_DATA_CTL = crate::Reg<crc_data_ctl::CRC_DATA_CTL_SPEC>;
#[doc = "CRC data control"]
pub mod crc_data_ctl;
#[doc = "CRC_POL_CTL (rw) register accessor: CRC polynomial control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_pol_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_pol_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_pol_ctl`]
module"]
pub type CRC_POL_CTL = crate::Reg<crc_pol_ctl::CRC_POL_CTL_SPEC>;
#[doc = "CRC polynomial control"]
pub mod crc_pol_ctl;
#[doc = "CRC_LFSR_CTL (rw) register accessor: CRC LFSR control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_lfsr_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_lfsr_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_lfsr_ctl`]
module"]
pub type CRC_LFSR_CTL = crate::Reg<crc_lfsr_ctl::CRC_LFSR_CTL_SPEC>;
#[doc = "CRC LFSR control"]
pub mod crc_lfsr_ctl;
#[doc = "CRC_REM_CTL (rw) register accessor: CRC remainder control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_rem_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_rem_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_rem_ctl`]
module"]
pub type CRC_REM_CTL = crate::Reg<crc_rem_ctl::CRC_REM_CTL_SPEC>;
#[doc = "CRC remainder control"]
pub mod crc_rem_ctl;
#[doc = "CRC_REM_RESULT (r) register accessor: CRC remainder result\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_rem_result::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_rem_result`]
module"]
pub type CRC_REM_RESULT = crate::Reg<crc_rem_result::CRC_REM_RESULT_SPEC>;
#[doc = "CRC remainder result"]
pub mod crc_rem_result;
#[doc = "DW channel structure"]
pub use self::ch_struct::CH_STRUCT;
#[doc = r"Cluster"]
#[doc = "DW channel structure"]
pub mod ch_struct;
