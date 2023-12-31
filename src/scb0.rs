#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    status: STATUS,
    cmd_resp_ctrl: CMD_RESP_CTRL,
    cmd_resp_status: CMD_RESP_STATUS,
    _reserved4: [u8; 0x10],
    spi_ctrl: SPI_CTRL,
    spi_status: SPI_STATUS,
    _reserved6: [u8; 0x18],
    uart_ctrl: UART_CTRL,
    uart_tx_ctrl: UART_TX_CTRL,
    uart_rx_ctrl: UART_RX_CTRL,
    uart_rx_status: UART_RX_STATUS,
    uart_flow_ctrl: UART_FLOW_CTRL,
    _reserved11: [u8; 0x0c],
    i2c_ctrl: I2C_CTRL,
    i2c_status: I2C_STATUS,
    i2c_m_cmd: I2C_M_CMD,
    i2c_s_cmd: I2C_S_CMD,
    i2c_cfg: I2C_CFG,
    _reserved16: [u8; 0x018c],
    tx_ctrl: TX_CTRL,
    tx_fifo_ctrl: TX_FIFO_CTRL,
    tx_fifo_status: TX_FIFO_STATUS,
    _reserved19: [u8; 0x34],
    tx_fifo_wr: TX_FIFO_WR,
    _reserved20: [u8; 0xbc],
    rx_ctrl: RX_CTRL,
    rx_fifo_ctrl: RX_FIFO_CTRL,
    rx_fifo_status: RX_FIFO_STATUS,
    _reserved23: [u8; 0x04],
    rx_match: RX_MATCH,
    _reserved24: [u8; 0x2c],
    rx_fifo_rd: RX_FIFO_RD,
    rx_fifo_rd_silent: RX_FIFO_RD_SILENT,
    _reserved26: [u8; 0x0ab8],
    intr_cause: INTR_CAUSE,
    _reserved27: [u8; 0x7c],
    intr_i2c_ec: INTR_I2C_EC,
    _reserved28: [u8; 0x04],
    intr_i2c_ec_mask: INTR_I2C_EC_MASK,
    intr_i2c_ec_masked: INTR_I2C_EC_MASKED,
    _reserved30: [u8; 0x30],
    intr_spi_ec: INTR_SPI_EC,
    _reserved31: [u8; 0x04],
    intr_spi_ec_mask: INTR_SPI_EC_MASK,
    intr_spi_ec_masked: INTR_SPI_EC_MASKED,
    _reserved33: [u8; 0x30],
    intr_m: INTR_M,
    intr_m_set: INTR_M_SET,
    intr_m_mask: INTR_M_MASK,
    intr_m_masked: INTR_M_MASKED,
    _reserved37: [u8; 0x30],
    intr_s: INTR_S,
    intr_s_set: INTR_S_SET,
    intr_s_mask: INTR_S_MASK,
    intr_s_masked: INTR_S_MASKED,
    _reserved41: [u8; 0x30],
    intr_tx: INTR_TX,
    intr_tx_set: INTR_TX_SET,
    intr_tx_mask: INTR_TX_MASK,
    intr_tx_masked: INTR_TX_MASKED,
    _reserved45: [u8; 0x30],
    intr_rx: INTR_RX,
    intr_rx_set: INTR_RX_SET,
    intr_rx_mask: INTR_RX_MASK,
    intr_rx_masked: INTR_RX_MASKED,
}
impl RegisterBlock {
    #[doc = "0x00 - Generic control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Generic status"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x08 - Command/response control"]
    #[inline(always)]
    pub const fn cmd_resp_ctrl(&self) -> &CMD_RESP_CTRL {
        &self.cmd_resp_ctrl
    }
    #[doc = "0x0c - Command/response status"]
    #[inline(always)]
    pub const fn cmd_resp_status(&self) -> &CMD_RESP_STATUS {
        &self.cmd_resp_status
    }
    #[doc = "0x20 - SPI control"]
    #[inline(always)]
    pub const fn spi_ctrl(&self) -> &SPI_CTRL {
        &self.spi_ctrl
    }
    #[doc = "0x24 - SPI status"]
    #[inline(always)]
    pub const fn spi_status(&self) -> &SPI_STATUS {
        &self.spi_status
    }
    #[doc = "0x40 - UART control"]
    #[inline(always)]
    pub const fn uart_ctrl(&self) -> &UART_CTRL {
        &self.uart_ctrl
    }
    #[doc = "0x44 - UART transmitter control"]
    #[inline(always)]
    pub const fn uart_tx_ctrl(&self) -> &UART_TX_CTRL {
        &self.uart_tx_ctrl
    }
    #[doc = "0x48 - UART receiver control"]
    #[inline(always)]
    pub const fn uart_rx_ctrl(&self) -> &UART_RX_CTRL {
        &self.uart_rx_ctrl
    }
    #[doc = "0x4c - UART receiver status"]
    #[inline(always)]
    pub const fn uart_rx_status(&self) -> &UART_RX_STATUS {
        &self.uart_rx_status
    }
    #[doc = "0x50 - UART flow control"]
    #[inline(always)]
    pub const fn uart_flow_ctrl(&self) -> &UART_FLOW_CTRL {
        &self.uart_flow_ctrl
    }
    #[doc = "0x60 - I2C control"]
    #[inline(always)]
    pub const fn i2c_ctrl(&self) -> &I2C_CTRL {
        &self.i2c_ctrl
    }
    #[doc = "0x64 - I2C status"]
    #[inline(always)]
    pub const fn i2c_status(&self) -> &I2C_STATUS {
        &self.i2c_status
    }
    #[doc = "0x68 - I2C master command"]
    #[inline(always)]
    pub const fn i2c_m_cmd(&self) -> &I2C_M_CMD {
        &self.i2c_m_cmd
    }
    #[doc = "0x6c - I2C slave command"]
    #[inline(always)]
    pub const fn i2c_s_cmd(&self) -> &I2C_S_CMD {
        &self.i2c_s_cmd
    }
    #[doc = "0x70 - I2C configuration"]
    #[inline(always)]
    pub const fn i2c_cfg(&self) -> &I2C_CFG {
        &self.i2c_cfg
    }
    #[doc = "0x200 - Transmitter control"]
    #[inline(always)]
    pub const fn tx_ctrl(&self) -> &TX_CTRL {
        &self.tx_ctrl
    }
    #[doc = "0x204 - Transmitter FIFO control"]
    #[inline(always)]
    pub const fn tx_fifo_ctrl(&self) -> &TX_FIFO_CTRL {
        &self.tx_fifo_ctrl
    }
    #[doc = "0x208 - Transmitter FIFO status"]
    #[inline(always)]
    pub const fn tx_fifo_status(&self) -> &TX_FIFO_STATUS {
        &self.tx_fifo_status
    }
    #[doc = "0x240 - Transmitter FIFO write"]
    #[inline(always)]
    pub const fn tx_fifo_wr(&self) -> &TX_FIFO_WR {
        &self.tx_fifo_wr
    }
    #[doc = "0x300 - Receiver control"]
    #[inline(always)]
    pub const fn rx_ctrl(&self) -> &RX_CTRL {
        &self.rx_ctrl
    }
    #[doc = "0x304 - Receiver FIFO control"]
    #[inline(always)]
    pub const fn rx_fifo_ctrl(&self) -> &RX_FIFO_CTRL {
        &self.rx_fifo_ctrl
    }
    #[doc = "0x308 - Receiver FIFO status"]
    #[inline(always)]
    pub const fn rx_fifo_status(&self) -> &RX_FIFO_STATUS {
        &self.rx_fifo_status
    }
    #[doc = "0x310 - Slave address and mask"]
    #[inline(always)]
    pub const fn rx_match(&self) -> &RX_MATCH {
        &self.rx_match
    }
    #[doc = "0x340 - Receiver FIFO read"]
    #[inline(always)]
    pub const fn rx_fifo_rd(&self) -> &RX_FIFO_RD {
        &self.rx_fifo_rd
    }
    #[doc = "0x344 - Receiver FIFO read silent"]
    #[inline(always)]
    pub const fn rx_fifo_rd_silent(&self) -> &RX_FIFO_RD_SILENT {
        &self.rx_fifo_rd_silent
    }
    #[doc = "0xe00 - Active clocked interrupt signal"]
    #[inline(always)]
    pub const fn intr_cause(&self) -> &INTR_CAUSE {
        &self.intr_cause
    }
    #[doc = "0xe80 - Externally clocked I2C interrupt request"]
    #[inline(always)]
    pub const fn intr_i2c_ec(&self) -> &INTR_I2C_EC {
        &self.intr_i2c_ec
    }
    #[doc = "0xe88 - Externally clocked I2C interrupt mask"]
    #[inline(always)]
    pub const fn intr_i2c_ec_mask(&self) -> &INTR_I2C_EC_MASK {
        &self.intr_i2c_ec_mask
    }
    #[doc = "0xe8c - Externally clocked I2C interrupt masked"]
    #[inline(always)]
    pub const fn intr_i2c_ec_masked(&self) -> &INTR_I2C_EC_MASKED {
        &self.intr_i2c_ec_masked
    }
    #[doc = "0xec0 - Externally clocked SPI interrupt request"]
    #[inline(always)]
    pub const fn intr_spi_ec(&self) -> &INTR_SPI_EC {
        &self.intr_spi_ec
    }
    #[doc = "0xec8 - Externally clocked SPI interrupt mask"]
    #[inline(always)]
    pub const fn intr_spi_ec_mask(&self) -> &INTR_SPI_EC_MASK {
        &self.intr_spi_ec_mask
    }
    #[doc = "0xecc - Externally clocked SPI interrupt masked"]
    #[inline(always)]
    pub const fn intr_spi_ec_masked(&self) -> &INTR_SPI_EC_MASKED {
        &self.intr_spi_ec_masked
    }
    #[doc = "0xf00 - Master interrupt request"]
    #[inline(always)]
    pub const fn intr_m(&self) -> &INTR_M {
        &self.intr_m
    }
    #[doc = "0xf04 - Master interrupt set request"]
    #[inline(always)]
    pub const fn intr_m_set(&self) -> &INTR_M_SET {
        &self.intr_m_set
    }
    #[doc = "0xf08 - Master interrupt mask"]
    #[inline(always)]
    pub const fn intr_m_mask(&self) -> &INTR_M_MASK {
        &self.intr_m_mask
    }
    #[doc = "0xf0c - Master interrupt masked request"]
    #[inline(always)]
    pub const fn intr_m_masked(&self) -> &INTR_M_MASKED {
        &self.intr_m_masked
    }
    #[doc = "0xf40 - Slave interrupt request"]
    #[inline(always)]
    pub const fn intr_s(&self) -> &INTR_S {
        &self.intr_s
    }
    #[doc = "0xf44 - Slave interrupt set request"]
    #[inline(always)]
    pub const fn intr_s_set(&self) -> &INTR_S_SET {
        &self.intr_s_set
    }
    #[doc = "0xf48 - Slave interrupt mask"]
    #[inline(always)]
    pub const fn intr_s_mask(&self) -> &INTR_S_MASK {
        &self.intr_s_mask
    }
    #[doc = "0xf4c - Slave interrupt masked request"]
    #[inline(always)]
    pub const fn intr_s_masked(&self) -> &INTR_S_MASKED {
        &self.intr_s_masked
    }
    #[doc = "0xf80 - Transmitter interrupt request"]
    #[inline(always)]
    pub const fn intr_tx(&self) -> &INTR_TX {
        &self.intr_tx
    }
    #[doc = "0xf84 - Transmitter interrupt set request"]
    #[inline(always)]
    pub const fn intr_tx_set(&self) -> &INTR_TX_SET {
        &self.intr_tx_set
    }
    #[doc = "0xf88 - Transmitter interrupt mask"]
    #[inline(always)]
    pub const fn intr_tx_mask(&self) -> &INTR_TX_MASK {
        &self.intr_tx_mask
    }
    #[doc = "0xf8c - Transmitter interrupt masked request"]
    #[inline(always)]
    pub const fn intr_tx_masked(&self) -> &INTR_TX_MASKED {
        &self.intr_tx_masked
    }
    #[doc = "0xfc0 - Receiver interrupt request"]
    #[inline(always)]
    pub const fn intr_rx(&self) -> &INTR_RX {
        &self.intr_rx
    }
    #[doc = "0xfc4 - Receiver interrupt set request"]
    #[inline(always)]
    pub const fn intr_rx_set(&self) -> &INTR_RX_SET {
        &self.intr_rx_set
    }
    #[doc = "0xfc8 - Receiver interrupt mask"]
    #[inline(always)]
    pub const fn intr_rx_mask(&self) -> &INTR_RX_MASK {
        &self.intr_rx_mask
    }
    #[doc = "0xfcc - Receiver interrupt masked request"]
    #[inline(always)]
    pub const fn intr_rx_masked(&self) -> &INTR_RX_MASKED {
        &self.intr_rx_masked
    }
}
#[doc = "CTRL (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Generic control"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: Generic status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Generic status"]
pub mod status;
#[doc = "CMD_RESP_CTRL (rw) register accessor: Command/response control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_resp_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_resp_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_resp_ctrl`]
module"]
pub type CMD_RESP_CTRL = crate::Reg<cmd_resp_ctrl::CMD_RESP_CTRL_SPEC>;
#[doc = "Command/response control"]
pub mod cmd_resp_ctrl;
#[doc = "CMD_RESP_STATUS (r) register accessor: Command/response status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_resp_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_resp_status`]
module"]
pub type CMD_RESP_STATUS = crate::Reg<cmd_resp_status::CMD_RESP_STATUS_SPEC>;
#[doc = "Command/response status"]
pub mod cmd_resp_status;
#[doc = "SPI_CTRL (rw) register accessor: SPI control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ctrl`]
module"]
pub type SPI_CTRL = crate::Reg<spi_ctrl::SPI_CTRL_SPEC>;
#[doc = "SPI control"]
pub mod spi_ctrl;
#[doc = "SPI_STATUS (r) register accessor: SPI status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_status`]
module"]
pub type SPI_STATUS = crate::Reg<spi_status::SPI_STATUS_SPEC>;
#[doc = "SPI status"]
pub mod spi_status;
#[doc = "UART_CTRL (rw) register accessor: UART control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_ctrl`]
module"]
pub type UART_CTRL = crate::Reg<uart_ctrl::UART_CTRL_SPEC>;
#[doc = "UART control"]
pub mod uart_ctrl;
#[doc = "UART_TX_CTRL (rw) register accessor: UART transmitter control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_tx_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_tx_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_tx_ctrl`]
module"]
pub type UART_TX_CTRL = crate::Reg<uart_tx_ctrl::UART_TX_CTRL_SPEC>;
#[doc = "UART transmitter control"]
pub mod uart_tx_ctrl;
#[doc = "UART_RX_CTRL (rw) register accessor: UART receiver control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_rx_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_rx_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_rx_ctrl`]
module"]
pub type UART_RX_CTRL = crate::Reg<uart_rx_ctrl::UART_RX_CTRL_SPEC>;
#[doc = "UART receiver control"]
pub mod uart_rx_ctrl;
#[doc = "UART_RX_STATUS (r) register accessor: UART receiver status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_rx_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_rx_status`]
module"]
pub type UART_RX_STATUS = crate::Reg<uart_rx_status::UART_RX_STATUS_SPEC>;
#[doc = "UART receiver status"]
pub mod uart_rx_status;
#[doc = "UART_FLOW_CTRL (rw) register accessor: UART flow control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_flow_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_flow_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_flow_ctrl`]
module"]
pub type UART_FLOW_CTRL = crate::Reg<uart_flow_ctrl::UART_FLOW_CTRL_SPEC>;
#[doc = "UART flow control"]
pub mod uart_flow_ctrl;
#[doc = "I2C_CTRL (rw) register accessor: I2C control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_ctrl`]
module"]
pub type I2C_CTRL = crate::Reg<i2c_ctrl::I2C_CTRL_SPEC>;
#[doc = "I2C control"]
pub mod i2c_ctrl;
#[doc = "I2C_STATUS (r) register accessor: I2C status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_status`]
module"]
pub type I2C_STATUS = crate::Reg<i2c_status::I2C_STATUS_SPEC>;
#[doc = "I2C status"]
pub mod i2c_status;
#[doc = "I2C_M_CMD (rw) register accessor: I2C master command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_m_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_m_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_m_cmd`]
module"]
pub type I2C_M_CMD = crate::Reg<i2c_m_cmd::I2C_M_CMD_SPEC>;
#[doc = "I2C master command"]
pub mod i2c_m_cmd;
#[doc = "I2C_S_CMD (rw) register accessor: I2C slave command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_s_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_s_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_s_cmd`]
module"]
pub type I2C_S_CMD = crate::Reg<i2c_s_cmd::I2C_S_CMD_SPEC>;
#[doc = "I2C slave command"]
pub mod i2c_s_cmd;
#[doc = "I2C_CFG (rw) register accessor: I2C configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_cfg`]
module"]
pub type I2C_CFG = crate::Reg<i2c_cfg::I2C_CFG_SPEC>;
#[doc = "I2C configuration"]
pub mod i2c_cfg;
#[doc = "TX_CTRL (rw) register accessor: Transmitter control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_ctrl`]
module"]
pub type TX_CTRL = crate::Reg<tx_ctrl::TX_CTRL_SPEC>;
#[doc = "Transmitter control"]
pub mod tx_ctrl;
#[doc = "TX_FIFO_CTRL (rw) register accessor: Transmitter FIFO control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_fifo_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_fifo_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_fifo_ctrl`]
module"]
pub type TX_FIFO_CTRL = crate::Reg<tx_fifo_ctrl::TX_FIFO_CTRL_SPEC>;
#[doc = "Transmitter FIFO control"]
pub mod tx_fifo_ctrl;
#[doc = "TX_FIFO_STATUS (r) register accessor: Transmitter FIFO status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_fifo_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_fifo_status`]
module"]
pub type TX_FIFO_STATUS = crate::Reg<tx_fifo_status::TX_FIFO_STATUS_SPEC>;
#[doc = "Transmitter FIFO status"]
pub mod tx_fifo_status;
#[doc = "TX_FIFO_WR (w) register accessor: Transmitter FIFO write\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_fifo_wr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_fifo_wr`]
module"]
pub type TX_FIFO_WR = crate::Reg<tx_fifo_wr::TX_FIFO_WR_SPEC>;
#[doc = "Transmitter FIFO write"]
pub mod tx_fifo_wr;
#[doc = "RX_CTRL (rw) register accessor: Receiver control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ctrl`]
module"]
pub type RX_CTRL = crate::Reg<rx_ctrl::RX_CTRL_SPEC>;
#[doc = "Receiver control"]
pub mod rx_ctrl;
#[doc = "RX_FIFO_CTRL (rw) register accessor: Receiver FIFO control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_fifo_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_fifo_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_fifo_ctrl`]
module"]
pub type RX_FIFO_CTRL = crate::Reg<rx_fifo_ctrl::RX_FIFO_CTRL_SPEC>;
#[doc = "Receiver FIFO control"]
pub mod rx_fifo_ctrl;
#[doc = "RX_FIFO_STATUS (r) register accessor: Receiver FIFO status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_fifo_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_fifo_status`]
module"]
pub type RX_FIFO_STATUS = crate::Reg<rx_fifo_status::RX_FIFO_STATUS_SPEC>;
#[doc = "Receiver FIFO status"]
pub mod rx_fifo_status;
#[doc = "RX_MATCH (rw) register accessor: Slave address and mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_match::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_match::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_match`]
module"]
pub type RX_MATCH = crate::Reg<rx_match::RX_MATCH_SPEC>;
#[doc = "Slave address and mask"]
pub mod rx_match;
#[doc = "RX_FIFO_RD (r) register accessor: Receiver FIFO read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_fifo_rd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_fifo_rd`]
module"]
pub type RX_FIFO_RD = crate::Reg<rx_fifo_rd::RX_FIFO_RD_SPEC>;
#[doc = "Receiver FIFO read"]
pub mod rx_fifo_rd;
#[doc = "RX_FIFO_RD_SILENT (r) register accessor: Receiver FIFO read silent\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_fifo_rd_silent::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_fifo_rd_silent`]
module"]
pub type RX_FIFO_RD_SILENT = crate::Reg<rx_fifo_rd_silent::RX_FIFO_RD_SILENT_SPEC>;
#[doc = "Receiver FIFO read silent"]
pub mod rx_fifo_rd_silent;
#[doc = "INTR_CAUSE (r) register accessor: Active clocked interrupt signal\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_cause::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cause`]
module"]
pub type INTR_CAUSE = crate::Reg<intr_cause::INTR_CAUSE_SPEC>;
#[doc = "Active clocked interrupt signal"]
pub mod intr_cause;
#[doc = "INTR_I2C_EC (rw) register accessor: Externally clocked I2C interrupt request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_i2c_ec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_i2c_ec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_i2c_ec`]
module"]
pub type INTR_I2C_EC = crate::Reg<intr_i2c_ec::INTR_I2C_EC_SPEC>;
#[doc = "Externally clocked I2C interrupt request"]
pub mod intr_i2c_ec;
#[doc = "INTR_I2C_EC_MASK (rw) register accessor: Externally clocked I2C interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_i2c_ec_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_i2c_ec_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_i2c_ec_mask`]
module"]
pub type INTR_I2C_EC_MASK = crate::Reg<intr_i2c_ec_mask::INTR_I2C_EC_MASK_SPEC>;
#[doc = "Externally clocked I2C interrupt mask"]
pub mod intr_i2c_ec_mask;
#[doc = "INTR_I2C_EC_MASKED (r) register accessor: Externally clocked I2C interrupt masked\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_i2c_ec_masked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_i2c_ec_masked`]
module"]
pub type INTR_I2C_EC_MASKED = crate::Reg<intr_i2c_ec_masked::INTR_I2C_EC_MASKED_SPEC>;
#[doc = "Externally clocked I2C interrupt masked"]
pub mod intr_i2c_ec_masked;
#[doc = "INTR_SPI_EC (rw) register accessor: Externally clocked SPI interrupt request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_spi_ec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_spi_ec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_spi_ec`]
module"]
pub type INTR_SPI_EC = crate::Reg<intr_spi_ec::INTR_SPI_EC_SPEC>;
#[doc = "Externally clocked SPI interrupt request"]
pub mod intr_spi_ec;
#[doc = "INTR_SPI_EC_MASK (rw) register accessor: Externally clocked SPI interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_spi_ec_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_spi_ec_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_spi_ec_mask`]
module"]
pub type INTR_SPI_EC_MASK = crate::Reg<intr_spi_ec_mask::INTR_SPI_EC_MASK_SPEC>;
#[doc = "Externally clocked SPI interrupt mask"]
pub mod intr_spi_ec_mask;
#[doc = "INTR_SPI_EC_MASKED (r) register accessor: Externally clocked SPI interrupt masked\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_spi_ec_masked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_spi_ec_masked`]
module"]
pub type INTR_SPI_EC_MASKED = crate::Reg<intr_spi_ec_masked::INTR_SPI_EC_MASKED_SPEC>;
#[doc = "Externally clocked SPI interrupt masked"]
pub mod intr_spi_ec_masked;
#[doc = "INTR_M (rw) register accessor: Master interrupt request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_m::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_m::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_m`]
module"]
pub type INTR_M = crate::Reg<intr_m::INTR_M_SPEC>;
#[doc = "Master interrupt request"]
pub mod intr_m;
#[doc = "INTR_M_SET (rw) register accessor: Master interrupt set request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_m_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_m_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_m_set`]
module"]
pub type INTR_M_SET = crate::Reg<intr_m_set::INTR_M_SET_SPEC>;
#[doc = "Master interrupt set request"]
pub mod intr_m_set;
#[doc = "INTR_M_MASK (rw) register accessor: Master interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_m_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_m_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_m_mask`]
module"]
pub type INTR_M_MASK = crate::Reg<intr_m_mask::INTR_M_MASK_SPEC>;
#[doc = "Master interrupt mask"]
pub mod intr_m_mask;
#[doc = "INTR_M_MASKED (r) register accessor: Master interrupt masked request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_m_masked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_m_masked`]
module"]
pub type INTR_M_MASKED = crate::Reg<intr_m_masked::INTR_M_MASKED_SPEC>;
#[doc = "Master interrupt masked request"]
pub mod intr_m_masked;
#[doc = "INTR_S (rw) register accessor: Slave interrupt request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_s`]
module"]
pub type INTR_S = crate::Reg<intr_s::INTR_S_SPEC>;
#[doc = "Slave interrupt request"]
pub mod intr_s;
#[doc = "INTR_S_SET (rw) register accessor: Slave interrupt set request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_s_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_s_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_s_set`]
module"]
pub type INTR_S_SET = crate::Reg<intr_s_set::INTR_S_SET_SPEC>;
#[doc = "Slave interrupt set request"]
pub mod intr_s_set;
#[doc = "INTR_S_MASK (rw) register accessor: Slave interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_s_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_s_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_s_mask`]
module"]
pub type INTR_S_MASK = crate::Reg<intr_s_mask::INTR_S_MASK_SPEC>;
#[doc = "Slave interrupt mask"]
pub mod intr_s_mask;
#[doc = "INTR_S_MASKED (r) register accessor: Slave interrupt masked request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_s_masked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_s_masked`]
module"]
pub type INTR_S_MASKED = crate::Reg<intr_s_masked::INTR_S_MASKED_SPEC>;
#[doc = "Slave interrupt masked request"]
pub mod intr_s_masked;
#[doc = "INTR_TX (rw) register accessor: Transmitter interrupt request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_tx`]
module"]
pub type INTR_TX = crate::Reg<intr_tx::INTR_TX_SPEC>;
#[doc = "Transmitter interrupt request"]
pub mod intr_tx;
#[doc = "INTR_TX_SET (rw) register accessor: Transmitter interrupt set request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_tx_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_tx_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_tx_set`]
module"]
pub type INTR_TX_SET = crate::Reg<intr_tx_set::INTR_TX_SET_SPEC>;
#[doc = "Transmitter interrupt set request"]
pub mod intr_tx_set;
#[doc = "INTR_TX_MASK (rw) register accessor: Transmitter interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_tx_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_tx_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_tx_mask`]
module"]
pub type INTR_TX_MASK = crate::Reg<intr_tx_mask::INTR_TX_MASK_SPEC>;
#[doc = "Transmitter interrupt mask"]
pub mod intr_tx_mask;
#[doc = "INTR_TX_MASKED (r) register accessor: Transmitter interrupt masked request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_tx_masked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_tx_masked`]
module"]
pub type INTR_TX_MASKED = crate::Reg<intr_tx_masked::INTR_TX_MASKED_SPEC>;
#[doc = "Transmitter interrupt masked request"]
pub mod intr_tx_masked;
#[doc = "INTR_RX (rw) register accessor: Receiver interrupt request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_rx`]
module"]
pub type INTR_RX = crate::Reg<intr_rx::INTR_RX_SPEC>;
#[doc = "Receiver interrupt request"]
pub mod intr_rx;
#[doc = "INTR_RX_SET (rw) register accessor: Receiver interrupt set request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_rx_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_rx_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_rx_set`]
module"]
pub type INTR_RX_SET = crate::Reg<intr_rx_set::INTR_RX_SET_SPEC>;
#[doc = "Receiver interrupt set request"]
pub mod intr_rx_set;
#[doc = "INTR_RX_MASK (rw) register accessor: Receiver interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_rx_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_rx_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_rx_mask`]
module"]
pub type INTR_RX_MASK = crate::Reg<intr_rx_mask::INTR_RX_MASK_SPEC>;
#[doc = "Receiver interrupt mask"]
pub mod intr_rx_mask;
#[doc = "INTR_RX_MASKED (r) register accessor: Receiver interrupt masked request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_rx_masked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_rx_masked`]
module"]
pub type INTR_RX_MASKED = crate::Reg<intr_rx_masked::INTR_RX_MASKED_SPEC>;
#[doc = "Receiver interrupt masked request"]
pub mod intr_rx_masked;
