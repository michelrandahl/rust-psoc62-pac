#[doc = r"Register block"]
#[repr(C)]
pub struct DEVICE {
    ctl: CTL,
    _reserved1: [u8; 0x04],
    addr: ADDR,
    mask: MASK,
    _reserved3: [u8; 0x10],
    addr_ctl: ADDR_CTL,
    _reserved4: [u8; 0x1c],
    rd_cmd_ctl: RD_CMD_CTL,
    rd_addr_ctl: RD_ADDR_CTL,
    rd_mode_ctl: RD_MODE_CTL,
    rd_dummy_ctl: RD_DUMMY_CTL,
    rd_data_ctl: RD_DATA_CTL,
    _reserved9: [u8; 0x0c],
    wr_cmd_ctl: WR_CMD_CTL,
    wr_addr_ctl: WR_ADDR_CTL,
    wr_mode_ctl: WR_MODE_CTL,
    wr_dummy_ctl: WR_DUMMY_CTL,
    wr_data_ctl: WR_DATA_CTL,
}
impl DEVICE {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &CTL {
        &self.ctl
    }
    #[doc = "0x08 - Device region base address"]
    #[inline(always)]
    pub const fn addr(&self) -> &ADDR {
        &self.addr
    }
    #[doc = "0x0c - Device region mask"]
    #[inline(always)]
    pub const fn mask(&self) -> &MASK {
        &self.mask
    }
    #[doc = "0x20 - Address control"]
    #[inline(always)]
    pub const fn addr_ctl(&self) -> &ADDR_CTL {
        &self.addr_ctl
    }
    #[doc = "0x40 - Read command control"]
    #[inline(always)]
    pub const fn rd_cmd_ctl(&self) -> &RD_CMD_CTL {
        &self.rd_cmd_ctl
    }
    #[doc = "0x44 - Read address control"]
    #[inline(always)]
    pub const fn rd_addr_ctl(&self) -> &RD_ADDR_CTL {
        &self.rd_addr_ctl
    }
    #[doc = "0x48 - Read mode control"]
    #[inline(always)]
    pub const fn rd_mode_ctl(&self) -> &RD_MODE_CTL {
        &self.rd_mode_ctl
    }
    #[doc = "0x4c - Read dummy control"]
    #[inline(always)]
    pub const fn rd_dummy_ctl(&self) -> &RD_DUMMY_CTL {
        &self.rd_dummy_ctl
    }
    #[doc = "0x50 - Read data control"]
    #[inline(always)]
    pub const fn rd_data_ctl(&self) -> &RD_DATA_CTL {
        &self.rd_data_ctl
    }
    #[doc = "0x60 - Write command control"]
    #[inline(always)]
    pub const fn wr_cmd_ctl(&self) -> &WR_CMD_CTL {
        &self.wr_cmd_ctl
    }
    #[doc = "0x64 - Write address control"]
    #[inline(always)]
    pub const fn wr_addr_ctl(&self) -> &WR_ADDR_CTL {
        &self.wr_addr_ctl
    }
    #[doc = "0x68 - Write mode control"]
    #[inline(always)]
    pub const fn wr_mode_ctl(&self) -> &WR_MODE_CTL {
        &self.wr_mode_ctl
    }
    #[doc = "0x6c - Write dummy control"]
    #[inline(always)]
    pub const fn wr_dummy_ctl(&self) -> &WR_DUMMY_CTL {
        &self.wr_dummy_ctl
    }
    #[doc = "0x70 - Write data control"]
    #[inline(always)]
    pub const fn wr_data_ctl(&self) -> &WR_DATA_CTL {
        &self.wr_data_ctl
    }
}
#[doc = "CTL (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "ADDR (rw) register accessor: Device region base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Device region base address"]
pub mod addr;
#[doc = "MASK (rw) register accessor: Device region mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask`]
module"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "Device region mask"]
pub mod mask;
#[doc = "ADDR_CTL (rw) register accessor: Address control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr_ctl`]
module"]
pub type ADDR_CTL = crate::Reg<addr_ctl::ADDR_CTL_SPEC>;
#[doc = "Address control"]
pub mod addr_ctl;
#[doc = "RD_CMD_CTL (rw) register accessor: Read command control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_cmd_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_cmd_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_cmd_ctl`]
module"]
pub type RD_CMD_CTL = crate::Reg<rd_cmd_ctl::RD_CMD_CTL_SPEC>;
#[doc = "Read command control"]
pub mod rd_cmd_ctl;
#[doc = "RD_ADDR_CTL (rw) register accessor: Read address control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_addr_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_addr_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_addr_ctl`]
module"]
pub type RD_ADDR_CTL = crate::Reg<rd_addr_ctl::RD_ADDR_CTL_SPEC>;
#[doc = "Read address control"]
pub mod rd_addr_ctl;
#[doc = "RD_MODE_CTL (rw) register accessor: Read mode control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_mode_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_mode_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_mode_ctl`]
module"]
pub type RD_MODE_CTL = crate::Reg<rd_mode_ctl::RD_MODE_CTL_SPEC>;
#[doc = "Read mode control"]
pub mod rd_mode_ctl;
#[doc = "RD_DUMMY_CTL (rw) register accessor: Read dummy control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_dummy_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_dummy_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_dummy_ctl`]
module"]
pub type RD_DUMMY_CTL = crate::Reg<rd_dummy_ctl::RD_DUMMY_CTL_SPEC>;
#[doc = "Read dummy control"]
pub mod rd_dummy_ctl;
#[doc = "RD_DATA_CTL (rw) register accessor: Read data control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_data_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_data_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_data_ctl`]
module"]
pub type RD_DATA_CTL = crate::Reg<rd_data_ctl::RD_DATA_CTL_SPEC>;
#[doc = "Read data control"]
pub mod rd_data_ctl;
#[doc = "WR_CMD_CTL (rw) register accessor: Write command control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_cmd_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_cmd_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_cmd_ctl`]
module"]
pub type WR_CMD_CTL = crate::Reg<wr_cmd_ctl::WR_CMD_CTL_SPEC>;
#[doc = "Write command control"]
pub mod wr_cmd_ctl;
#[doc = "WR_ADDR_CTL (rw) register accessor: Write address control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_addr_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_addr_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_addr_ctl`]
module"]
pub type WR_ADDR_CTL = crate::Reg<wr_addr_ctl::WR_ADDR_CTL_SPEC>;
#[doc = "Write address control"]
pub mod wr_addr_ctl;
#[doc = "WR_MODE_CTL (rw) register accessor: Write mode control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_mode_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_mode_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_mode_ctl`]
module"]
pub type WR_MODE_CTL = crate::Reg<wr_mode_ctl::WR_MODE_CTL_SPEC>;
#[doc = "Write mode control"]
pub mod wr_mode_ctl;
#[doc = "WR_DUMMY_CTL (rw) register accessor: Write dummy control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_dummy_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_dummy_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_dummy_ctl`]
module"]
pub type WR_DUMMY_CTL = crate::Reg<wr_dummy_ctl::WR_DUMMY_CTL_SPEC>;
#[doc = "Write dummy control"]
pub mod wr_dummy_ctl;
#[doc = "WR_DATA_CTL (rw) register accessor: Write data control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_data_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_data_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_data_ctl`]
module"]
pub type WR_DATA_CTL = crate::Reg<wr_data_ctl::WR_DATA_CTL_SPEC>;
#[doc = "Write data control"]
pub mod wr_data_ctl;
