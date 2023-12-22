#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctl: CTL,
    _reserved1: [u8; 0x0c],
    clock_ctl: CLOCK_CTL,
    mode_ctl: MODE_CTL,
    data_ctl: DATA_CTL,
    _reserved4: [u8; 0x04],
    cmd: CMD,
    _reserved5: [u8; 0x1c],
    tr_ctl: TR_CTL,
    _reserved6: [u8; 0x02bc],
    rx_fifo_ctl: RX_FIFO_CTL,
    rx_fifo_status: RX_FIFO_STATUS,
    rx_fifo_rd: RX_FIFO_RD,
    rx_fifo_rd_silent: RX_FIFO_RD_SILENT,
    _reserved10: [u8; 0x0bf0],
    intr: INTR,
    intr_set: INTR_SET,
    intr_mask: INTR_MASK,
    intr_masked: INTR_MASKED,
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &CTL {
        &self.ctl
    }
    #[doc = "0x10 - Clock control"]
    #[inline(always)]
    pub const fn clock_ctl(&self) -> &CLOCK_CTL {
        &self.clock_ctl
    }
    #[doc = "0x14 - Mode control"]
    #[inline(always)]
    pub const fn mode_ctl(&self) -> &MODE_CTL {
        &self.mode_ctl
    }
    #[doc = "0x18 - Data control"]
    #[inline(always)]
    pub const fn data_ctl(&self) -> &DATA_CTL {
        &self.data_ctl
    }
    #[doc = "0x20 - Command"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x40 - Trigger control"]
    #[inline(always)]
    pub const fn tr_ctl(&self) -> &TR_CTL {
        &self.tr_ctl
    }
    #[doc = "0x300 - RX FIFO control"]
    #[inline(always)]
    pub const fn rx_fifo_ctl(&self) -> &RX_FIFO_CTL {
        &self.rx_fifo_ctl
    }
    #[doc = "0x304 - RX FIFO status"]
    #[inline(always)]
    pub const fn rx_fifo_status(&self) -> &RX_FIFO_STATUS {
        &self.rx_fifo_status
    }
    #[doc = "0x308 - RX FIFO read"]
    #[inline(always)]
    pub const fn rx_fifo_rd(&self) -> &RX_FIFO_RD {
        &self.rx_fifo_rd
    }
    #[doc = "0x30c - RX FIFO silent read"]
    #[inline(always)]
    pub const fn rx_fifo_rd_silent(&self) -> &RX_FIFO_RD_SILENT {
        &self.rx_fifo_rd_silent
    }
    #[doc = "0xf00 - Interrupt register"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0xf04 - Interrupt set register"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &INTR_SET {
        &self.intr_set
    }
    #[doc = "0xf08 - Interrupt mask register"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &INTR_MASK {
        &self.intr_mask
    }
    #[doc = "0xf0c - Interrupt masked register"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &INTR_MASKED {
        &self.intr_masked
    }
}
#[doc = "CTL (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "CLOCK_CTL (rw) register accessor: Clock control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_ctl`]
module"]
pub type CLOCK_CTL = crate::Reg<clock_ctl::CLOCK_CTL_SPEC>;
#[doc = "Clock control"]
pub mod clock_ctl;
#[doc = "MODE_CTL (rw) register accessor: Mode control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode_ctl`]
module"]
pub type MODE_CTL = crate::Reg<mode_ctl::MODE_CTL_SPEC>;
#[doc = "Mode control"]
pub mod mode_ctl;
#[doc = "DATA_CTL (rw) register accessor: Data control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_ctl`]
module"]
pub type DATA_CTL = crate::Reg<data_ctl::DATA_CTL_SPEC>;
#[doc = "Data control"]
pub mod data_ctl;
#[doc = "CMD (rw) register accessor: Command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command"]
pub mod cmd;
#[doc = "TR_CTL (rw) register accessor: Trigger control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_ctl`]
module"]
pub type TR_CTL = crate::Reg<tr_ctl::TR_CTL_SPEC>;
#[doc = "Trigger control"]
pub mod tr_ctl;
#[doc = "RX_FIFO_CTL (rw) register accessor: RX FIFO control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_fifo_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_fifo_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_fifo_ctl`]
module"]
pub type RX_FIFO_CTL = crate::Reg<rx_fifo_ctl::RX_FIFO_CTL_SPEC>;
#[doc = "RX FIFO control"]
pub mod rx_fifo_ctl;
#[doc = "RX_FIFO_STATUS (r) register accessor: RX FIFO status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_fifo_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_fifo_status`]
module"]
pub type RX_FIFO_STATUS = crate::Reg<rx_fifo_status::RX_FIFO_STATUS_SPEC>;
#[doc = "RX FIFO status"]
pub mod rx_fifo_status;
#[doc = "RX_FIFO_RD (r) register accessor: RX FIFO read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_fifo_rd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_fifo_rd`]
module"]
pub type RX_FIFO_RD = crate::Reg<rx_fifo_rd::RX_FIFO_RD_SPEC>;
#[doc = "RX FIFO read"]
pub mod rx_fifo_rd;
#[doc = "RX_FIFO_RD_SILENT (r) register accessor: RX FIFO silent read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_fifo_rd_silent::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_fifo_rd_silent`]
module"]
pub type RX_FIFO_RD_SILENT = crate::Reg<rx_fifo_rd_silent::RX_FIFO_RD_SILENT_SPEC>;
#[doc = "RX FIFO silent read"]
pub mod rx_fifo_rd_silent;
#[doc = "INTR (rw) register accessor: Interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt register"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: Interrupt set register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_set`]
module"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt set register"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mask`]
module"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt mask register"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: Interrupt masked register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_masked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_masked`]
module"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt masked register"]
pub mod intr_masked;
