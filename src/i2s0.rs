#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctl: CTL,
    _reserved1: [u8; 0x0c],
    clock_ctl: CLOCK_CTL,
    _reserved2: [u8; 0x0c],
    cmd: CMD,
    _reserved3: [u8; 0x1c],
    tr_ctl: TR_CTL,
    _reserved4: [u8; 0x3c],
    tx_ctl: TX_CTL,
    tx_watchdog: TX_WATCHDOG,
    _reserved6: [u8; 0x18],
    rx_ctl: RX_CTL,
    rx_watchdog: RX_WATCHDOG,
    _reserved8: [u8; 0x0158],
    tx_fifo_ctl: TX_FIFO_CTL,
    tx_fifo_status: TX_FIFO_STATUS,
    tx_fifo_wr: TX_FIFO_WR,
    _reserved11: [u8; 0xf4],
    rx_fifo_ctl: RX_FIFO_CTL,
    rx_fifo_status: RX_FIFO_STATUS,
    rx_fifo_rd: RX_FIFO_RD,
    rx_fifo_rd_silent: RX_FIFO_RD_SILENT,
    _reserved15: [u8; 0x0bf0],
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
    #[doc = "0x80 - Transmitter control"]
    #[inline(always)]
    pub const fn tx_ctl(&self) -> &TX_CTL {
        &self.tx_ctl
    }
    #[doc = "0x84 - Transmitter watchdog"]
    #[inline(always)]
    pub const fn tx_watchdog(&self) -> &TX_WATCHDOG {
        &self.tx_watchdog
    }
    #[doc = "0xa0 - Receiver control"]
    #[inline(always)]
    pub const fn rx_ctl(&self) -> &RX_CTL {
        &self.rx_ctl
    }
    #[doc = "0xa4 - Receiver watchdog"]
    #[inline(always)]
    pub const fn rx_watchdog(&self) -> &RX_WATCHDOG {
        &self.rx_watchdog
    }
    #[doc = "0x200 - TX FIFO control"]
    #[inline(always)]
    pub const fn tx_fifo_ctl(&self) -> &TX_FIFO_CTL {
        &self.tx_fifo_ctl
    }
    #[doc = "0x204 - TX FIFO status"]
    #[inline(always)]
    pub const fn tx_fifo_status(&self) -> &TX_FIFO_STATUS {
        &self.tx_fifo_status
    }
    #[doc = "0x208 - TX FIFO write"]
    #[inline(always)]
    pub const fn tx_fifo_wr(&self) -> &TX_FIFO_WR {
        &self.tx_fifo_wr
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
#[doc = "TX_CTL (rw) register accessor: Transmitter control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_ctl`]
module"]
pub type TX_CTL = crate::Reg<tx_ctl::TX_CTL_SPEC>;
#[doc = "Transmitter control"]
pub mod tx_ctl;
#[doc = "TX_WATCHDOG (rw) register accessor: Transmitter watchdog\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_watchdog::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_watchdog::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_watchdog`]
module"]
pub type TX_WATCHDOG = crate::Reg<tx_watchdog::TX_WATCHDOG_SPEC>;
#[doc = "Transmitter watchdog"]
pub mod tx_watchdog;
#[doc = "RX_CTL (rw) register accessor: Receiver control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ctl`]
module"]
pub type RX_CTL = crate::Reg<rx_ctl::RX_CTL_SPEC>;
#[doc = "Receiver control"]
pub mod rx_ctl;
#[doc = "RX_WATCHDOG (rw) register accessor: Receiver watchdog\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_watchdog::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_watchdog::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_watchdog`]
module"]
pub type RX_WATCHDOG = crate::Reg<rx_watchdog::RX_WATCHDOG_SPEC>;
#[doc = "Receiver watchdog"]
pub mod rx_watchdog;
#[doc = "TX_FIFO_CTL (rw) register accessor: TX FIFO control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_fifo_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_fifo_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_fifo_ctl`]
module"]
pub type TX_FIFO_CTL = crate::Reg<tx_fifo_ctl::TX_FIFO_CTL_SPEC>;
#[doc = "TX FIFO control"]
pub mod tx_fifo_ctl;
#[doc = "TX_FIFO_STATUS (r) register accessor: TX FIFO status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_fifo_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_fifo_status`]
module"]
pub type TX_FIFO_STATUS = crate::Reg<tx_fifo_status::TX_FIFO_STATUS_SPEC>;
#[doc = "TX FIFO status"]
pub mod tx_fifo_status;
#[doc = "TX_FIFO_WR (w) register accessor: TX FIFO write\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_fifo_wr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_fifo_wr`]
module"]
pub type TX_FIFO_WR = crate::Reg<tx_fifo_wr::TX_FIFO_WR_SPEC>;
#[doc = "TX FIFO write"]
pub mod tx_fifo_wr;
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
