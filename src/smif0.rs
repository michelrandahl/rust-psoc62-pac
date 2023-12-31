#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctl: CTL,
    status: STATUS,
    _reserved2: [u8; 0x3c],
    tx_cmd_fifo_status: TX_CMD_FIFO_STATUS,
    _reserved3: [u8; 0x08],
    tx_cmd_fifo_wr: TX_CMD_FIFO_WR,
    _reserved4: [u8; 0x2c],
    tx_data_fifo_ctl: TX_DATA_FIFO_CTL,
    tx_data_fifo_status: TX_DATA_FIFO_STATUS,
    _reserved6: [u8; 0x08],
    tx_data_fifo_wr1: TX_DATA_FIFO_WR1,
    tx_data_fifo_wr2: TX_DATA_FIFO_WR2,
    tx_data_fifo_wr4: TX_DATA_FIFO_WR4,
    _reserved9: [u8; 0x24],
    rx_data_fifo_ctl: RX_DATA_FIFO_CTL,
    rx_data_fifo_status: RX_DATA_FIFO_STATUS,
    _reserved11: [u8; 0x08],
    rx_data_fifo_rd1: RX_DATA_FIFO_RD1,
    rx_data_fifo_rd2: RX_DATA_FIFO_RD2,
    rx_data_fifo_rd4: RX_DATA_FIFO_RD4,
    _reserved14: [u8; 0x04],
    rx_data_fifo_rd1_silent: RX_DATA_FIFO_RD1_SILENT,
    _reserved15: [u8; 0x1c],
    slow_ca_ctl: SLOW_CA_CTL,
    _reserved16: [u8; 0x04],
    slow_ca_cmd: SLOW_CA_CMD,
    _reserved17: [u8; 0x74],
    fast_ca_ctl: FAST_CA_CTL,
    _reserved18: [u8; 0x04],
    fast_ca_cmd: FAST_CA_CMD,
    _reserved19: [u8; 0x74],
    crypto_cmd: CRYPTO_CMD,
    _reserved20: [u8; 0x1c],
    crypto_input0: CRYPTO_INPUT0,
    crypto_input1: CRYPTO_INPUT1,
    crypto_input2: CRYPTO_INPUT2,
    crypto_input3: CRYPTO_INPUT3,
    _reserved24: [u8; 0x10],
    crypto_key0: CRYPTO_KEY0,
    crypto_key1: CRYPTO_KEY1,
    crypto_key2: CRYPTO_KEY2,
    crypto_key3: CRYPTO_KEY3,
    _reserved28: [u8; 0x10],
    crypto_output0: CRYPTO_OUTPUT0,
    crypto_output1: CRYPTO_OUTPUT1,
    crypto_output2: CRYPTO_OUTPUT2,
    crypto_output3: CRYPTO_OUTPUT3,
    _reserved32: [u8; 0x0550],
    intr: INTR,
    intr_set: INTR_SET,
    intr_mask: INTR_MASK,
    intr_masked: INTR_MASKED,
    _reserved36: [u8; 0x30],
    device: (),
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
    #[doc = "0x44 - Transmitter command FIFO status"]
    #[inline(always)]
    pub const fn tx_cmd_fifo_status(&self) -> &TX_CMD_FIFO_STATUS {
        &self.tx_cmd_fifo_status
    }
    #[doc = "0x50 - Transmitter command FIFO write"]
    #[inline(always)]
    pub const fn tx_cmd_fifo_wr(&self) -> &TX_CMD_FIFO_WR {
        &self.tx_cmd_fifo_wr
    }
    #[doc = "0x80 - Transmitter data FIFO control"]
    #[inline(always)]
    pub const fn tx_data_fifo_ctl(&self) -> &TX_DATA_FIFO_CTL {
        &self.tx_data_fifo_ctl
    }
    #[doc = "0x84 - Transmitter data FIFO status"]
    #[inline(always)]
    pub const fn tx_data_fifo_status(&self) -> &TX_DATA_FIFO_STATUS {
        &self.tx_data_fifo_status
    }
    #[doc = "0x90 - Transmitter data FIFO write"]
    #[inline(always)]
    pub const fn tx_data_fifo_wr1(&self) -> &TX_DATA_FIFO_WR1 {
        &self.tx_data_fifo_wr1
    }
    #[doc = "0x94 - Transmitter data FIFO write"]
    #[inline(always)]
    pub const fn tx_data_fifo_wr2(&self) -> &TX_DATA_FIFO_WR2 {
        &self.tx_data_fifo_wr2
    }
    #[doc = "0x98 - Transmitter data FIFO write"]
    #[inline(always)]
    pub const fn tx_data_fifo_wr4(&self) -> &TX_DATA_FIFO_WR4 {
        &self.tx_data_fifo_wr4
    }
    #[doc = "0xc0 - Receiver data FIFO control"]
    #[inline(always)]
    pub const fn rx_data_fifo_ctl(&self) -> &RX_DATA_FIFO_CTL {
        &self.rx_data_fifo_ctl
    }
    #[doc = "0xc4 - Receiver data FIFO status"]
    #[inline(always)]
    pub const fn rx_data_fifo_status(&self) -> &RX_DATA_FIFO_STATUS {
        &self.rx_data_fifo_status
    }
    #[doc = "0xd0 - Receiver data FIFO read"]
    #[inline(always)]
    pub const fn rx_data_fifo_rd1(&self) -> &RX_DATA_FIFO_RD1 {
        &self.rx_data_fifo_rd1
    }
    #[doc = "0xd4 - Receiver data FIFO read"]
    #[inline(always)]
    pub const fn rx_data_fifo_rd2(&self) -> &RX_DATA_FIFO_RD2 {
        &self.rx_data_fifo_rd2
    }
    #[doc = "0xd8 - Receiver data FIFO read"]
    #[inline(always)]
    pub const fn rx_data_fifo_rd4(&self) -> &RX_DATA_FIFO_RD4 {
        &self.rx_data_fifo_rd4
    }
    #[doc = "0xe0 - Receiver data FIFO silent read"]
    #[inline(always)]
    pub const fn rx_data_fifo_rd1_silent(&self) -> &RX_DATA_FIFO_RD1_SILENT {
        &self.rx_data_fifo_rd1_silent
    }
    #[doc = "0x100 - Slow cache control"]
    #[inline(always)]
    pub const fn slow_ca_ctl(&self) -> &SLOW_CA_CTL {
        &self.slow_ca_ctl
    }
    #[doc = "0x108 - Slow cache command"]
    #[inline(always)]
    pub const fn slow_ca_cmd(&self) -> &SLOW_CA_CMD {
        &self.slow_ca_cmd
    }
    #[doc = "0x180 - Fast cache control"]
    #[inline(always)]
    pub const fn fast_ca_ctl(&self) -> &FAST_CA_CTL {
        &self.fast_ca_ctl
    }
    #[doc = "0x188 - Fast cache command"]
    #[inline(always)]
    pub const fn fast_ca_cmd(&self) -> &FAST_CA_CMD {
        &self.fast_ca_cmd
    }
    #[doc = "0x200 - Cryptography Command"]
    #[inline(always)]
    pub const fn crypto_cmd(&self) -> &CRYPTO_CMD {
        &self.crypto_cmd
    }
    #[doc = "0x220 - Cryptography input 0"]
    #[inline(always)]
    pub const fn crypto_input0(&self) -> &CRYPTO_INPUT0 {
        &self.crypto_input0
    }
    #[doc = "0x224 - Cryptography input 1"]
    #[inline(always)]
    pub const fn crypto_input1(&self) -> &CRYPTO_INPUT1 {
        &self.crypto_input1
    }
    #[doc = "0x228 - Cryptography input 2"]
    #[inline(always)]
    pub const fn crypto_input2(&self) -> &CRYPTO_INPUT2 {
        &self.crypto_input2
    }
    #[doc = "0x22c - Cryptography input 3"]
    #[inline(always)]
    pub const fn crypto_input3(&self) -> &CRYPTO_INPUT3 {
        &self.crypto_input3
    }
    #[doc = "0x240 - Cryptography key 0"]
    #[inline(always)]
    pub const fn crypto_key0(&self) -> &CRYPTO_KEY0 {
        &self.crypto_key0
    }
    #[doc = "0x244 - Cryptography key 1"]
    #[inline(always)]
    pub const fn crypto_key1(&self) -> &CRYPTO_KEY1 {
        &self.crypto_key1
    }
    #[doc = "0x248 - Cryptography key 2"]
    #[inline(always)]
    pub const fn crypto_key2(&self) -> &CRYPTO_KEY2 {
        &self.crypto_key2
    }
    #[doc = "0x24c - Cryptography key 3"]
    #[inline(always)]
    pub const fn crypto_key3(&self) -> &CRYPTO_KEY3 {
        &self.crypto_key3
    }
    #[doc = "0x260 - Cryptography output 0"]
    #[inline(always)]
    pub const fn crypto_output0(&self) -> &CRYPTO_OUTPUT0 {
        &self.crypto_output0
    }
    #[doc = "0x264 - Cryptography output 1"]
    #[inline(always)]
    pub const fn crypto_output1(&self) -> &CRYPTO_OUTPUT1 {
        &self.crypto_output1
    }
    #[doc = "0x268 - Cryptography output 2"]
    #[inline(always)]
    pub const fn crypto_output2(&self) -> &CRYPTO_OUTPUT2 {
        &self.crypto_output2
    }
    #[doc = "0x26c - Cryptography output 3"]
    #[inline(always)]
    pub const fn crypto_output3(&self) -> &CRYPTO_OUTPUT3 {
        &self.crypto_output3
    }
    #[doc = "0x7c0 - Interrupt register"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0x7c4 - Interrupt set register"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &INTR_SET {
        &self.intr_set
    }
    #[doc = "0x7c8 - Interrupt mask register"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &INTR_MASK {
        &self.intr_mask
    }
    #[doc = "0x7cc - Interrupt masked register"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &INTR_MASKED {
        &self.intr_masked
    }
    #[doc = "0x800..0x9d0 - Device (only used in XIP mode)"]
    #[inline(always)]
    pub const fn device(&self, n: usize) -> &DEVICE {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(2048)
                .add(128 * n)
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
#[doc = "TX_CMD_FIFO_STATUS (r) register accessor: Transmitter command FIFO status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_cmd_fifo_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_cmd_fifo_status`]
module"]
pub type TX_CMD_FIFO_STATUS = crate::Reg<tx_cmd_fifo_status::TX_CMD_FIFO_STATUS_SPEC>;
#[doc = "Transmitter command FIFO status"]
pub mod tx_cmd_fifo_status;
#[doc = "TX_CMD_FIFO_WR (w) register accessor: Transmitter command FIFO write\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_cmd_fifo_wr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_cmd_fifo_wr`]
module"]
pub type TX_CMD_FIFO_WR = crate::Reg<tx_cmd_fifo_wr::TX_CMD_FIFO_WR_SPEC>;
#[doc = "Transmitter command FIFO write"]
pub mod tx_cmd_fifo_wr;
#[doc = "TX_DATA_FIFO_CTL (rw) register accessor: Transmitter data FIFO control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_data_fifo_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_data_fifo_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_data_fifo_ctl`]
module"]
pub type TX_DATA_FIFO_CTL = crate::Reg<tx_data_fifo_ctl::TX_DATA_FIFO_CTL_SPEC>;
#[doc = "Transmitter data FIFO control"]
pub mod tx_data_fifo_ctl;
#[doc = "TX_DATA_FIFO_STATUS (r) register accessor: Transmitter data FIFO status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_data_fifo_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_data_fifo_status`]
module"]
pub type TX_DATA_FIFO_STATUS = crate::Reg<tx_data_fifo_status::TX_DATA_FIFO_STATUS_SPEC>;
#[doc = "Transmitter data FIFO status"]
pub mod tx_data_fifo_status;
#[doc = "TX_DATA_FIFO_WR1 (w) register accessor: Transmitter data FIFO write\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_data_fifo_wr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_data_fifo_wr1`]
module"]
pub type TX_DATA_FIFO_WR1 = crate::Reg<tx_data_fifo_wr1::TX_DATA_FIFO_WR1_SPEC>;
#[doc = "Transmitter data FIFO write"]
pub mod tx_data_fifo_wr1;
#[doc = "TX_DATA_FIFO_WR2 (w) register accessor: Transmitter data FIFO write\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_data_fifo_wr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_data_fifo_wr2`]
module"]
pub type TX_DATA_FIFO_WR2 = crate::Reg<tx_data_fifo_wr2::TX_DATA_FIFO_WR2_SPEC>;
#[doc = "Transmitter data FIFO write"]
pub mod tx_data_fifo_wr2;
#[doc = "TX_DATA_FIFO_WR4 (w) register accessor: Transmitter data FIFO write\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_data_fifo_wr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_data_fifo_wr4`]
module"]
pub type TX_DATA_FIFO_WR4 = crate::Reg<tx_data_fifo_wr4::TX_DATA_FIFO_WR4_SPEC>;
#[doc = "Transmitter data FIFO write"]
pub mod tx_data_fifo_wr4;
#[doc = "RX_DATA_FIFO_CTL (rw) register accessor: Receiver data FIFO control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_data_fifo_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_data_fifo_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_data_fifo_ctl`]
module"]
pub type RX_DATA_FIFO_CTL = crate::Reg<rx_data_fifo_ctl::RX_DATA_FIFO_CTL_SPEC>;
#[doc = "Receiver data FIFO control"]
pub mod rx_data_fifo_ctl;
#[doc = "RX_DATA_FIFO_STATUS (r) register accessor: Receiver data FIFO status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_data_fifo_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_data_fifo_status`]
module"]
pub type RX_DATA_FIFO_STATUS = crate::Reg<rx_data_fifo_status::RX_DATA_FIFO_STATUS_SPEC>;
#[doc = "Receiver data FIFO status"]
pub mod rx_data_fifo_status;
#[doc = "RX_DATA_FIFO_RD1 (r) register accessor: Receiver data FIFO read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_data_fifo_rd1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_data_fifo_rd1`]
module"]
pub type RX_DATA_FIFO_RD1 = crate::Reg<rx_data_fifo_rd1::RX_DATA_FIFO_RD1_SPEC>;
#[doc = "Receiver data FIFO read"]
pub mod rx_data_fifo_rd1;
#[doc = "RX_DATA_FIFO_RD2 (r) register accessor: Receiver data FIFO read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_data_fifo_rd2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_data_fifo_rd2`]
module"]
pub type RX_DATA_FIFO_RD2 = crate::Reg<rx_data_fifo_rd2::RX_DATA_FIFO_RD2_SPEC>;
#[doc = "Receiver data FIFO read"]
pub mod rx_data_fifo_rd2;
#[doc = "RX_DATA_FIFO_RD4 (r) register accessor: Receiver data FIFO read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_data_fifo_rd4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_data_fifo_rd4`]
module"]
pub type RX_DATA_FIFO_RD4 = crate::Reg<rx_data_fifo_rd4::RX_DATA_FIFO_RD4_SPEC>;
#[doc = "Receiver data FIFO read"]
pub mod rx_data_fifo_rd4;
#[doc = "RX_DATA_FIFO_RD1_SILENT (r) register accessor: Receiver data FIFO silent read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_data_fifo_rd1_silent::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_data_fifo_rd1_silent`]
module"]
pub type RX_DATA_FIFO_RD1_SILENT =
    crate::Reg<rx_data_fifo_rd1_silent::RX_DATA_FIFO_RD1_SILENT_SPEC>;
#[doc = "Receiver data FIFO silent read"]
pub mod rx_data_fifo_rd1_silent;
#[doc = "SLOW_CA_CTL (rw) register accessor: Slow cache control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slow_ca_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slow_ca_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slow_ca_ctl`]
module"]
pub type SLOW_CA_CTL = crate::Reg<slow_ca_ctl::SLOW_CA_CTL_SPEC>;
#[doc = "Slow cache control"]
pub mod slow_ca_ctl;
#[doc = "SLOW_CA_CMD (rw) register accessor: Slow cache command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slow_ca_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slow_ca_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slow_ca_cmd`]
module"]
pub type SLOW_CA_CMD = crate::Reg<slow_ca_cmd::SLOW_CA_CMD_SPEC>;
#[doc = "Slow cache command"]
pub mod slow_ca_cmd;
#[doc = "FAST_CA_CTL (rw) register accessor: Fast cache control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fast_ca_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fast_ca_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fast_ca_ctl`]
module"]
pub type FAST_CA_CTL = crate::Reg<fast_ca_ctl::FAST_CA_CTL_SPEC>;
#[doc = "Fast cache control"]
pub mod fast_ca_ctl;
#[doc = "FAST_CA_CMD (rw) register accessor: Fast cache command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fast_ca_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fast_ca_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fast_ca_cmd`]
module"]
pub type FAST_CA_CMD = crate::Reg<fast_ca_cmd::FAST_CA_CMD_SPEC>;
#[doc = "Fast cache command"]
pub mod fast_ca_cmd;
#[doc = "CRYPTO_CMD (rw) register accessor: Cryptography Command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crypto_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_cmd`]
module"]
pub type CRYPTO_CMD = crate::Reg<crypto_cmd::CRYPTO_CMD_SPEC>;
#[doc = "Cryptography Command"]
pub mod crypto_cmd;
#[doc = "CRYPTO_INPUT0 (rw) register accessor: Cryptography input 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crypto_input0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_input0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_input0`]
module"]
pub type CRYPTO_INPUT0 = crate::Reg<crypto_input0::CRYPTO_INPUT0_SPEC>;
#[doc = "Cryptography input 0"]
pub mod crypto_input0;
#[doc = "CRYPTO_INPUT1 (rw) register accessor: Cryptography input 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crypto_input1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_input1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_input1`]
module"]
pub type CRYPTO_INPUT1 = crate::Reg<crypto_input1::CRYPTO_INPUT1_SPEC>;
#[doc = "Cryptography input 1"]
pub mod crypto_input1;
#[doc = "CRYPTO_INPUT2 (rw) register accessor: Cryptography input 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crypto_input2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_input2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_input2`]
module"]
pub type CRYPTO_INPUT2 = crate::Reg<crypto_input2::CRYPTO_INPUT2_SPEC>;
#[doc = "Cryptography input 2"]
pub mod crypto_input2;
#[doc = "CRYPTO_INPUT3 (rw) register accessor: Cryptography input 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crypto_input3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_input3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_input3`]
module"]
pub type CRYPTO_INPUT3 = crate::Reg<crypto_input3::CRYPTO_INPUT3_SPEC>;
#[doc = "Cryptography input 3"]
pub mod crypto_input3;
#[doc = "CRYPTO_KEY0 (w) register accessor: Cryptography key 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_key0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_key0`]
module"]
pub type CRYPTO_KEY0 = crate::Reg<crypto_key0::CRYPTO_KEY0_SPEC>;
#[doc = "Cryptography key 0"]
pub mod crypto_key0;
#[doc = "CRYPTO_KEY1 (w) register accessor: Cryptography key 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_key1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_key1`]
module"]
pub type CRYPTO_KEY1 = crate::Reg<crypto_key1::CRYPTO_KEY1_SPEC>;
#[doc = "Cryptography key 1"]
pub mod crypto_key1;
#[doc = "CRYPTO_KEY2 (w) register accessor: Cryptography key 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_key2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_key2`]
module"]
pub type CRYPTO_KEY2 = crate::Reg<crypto_key2::CRYPTO_KEY2_SPEC>;
#[doc = "Cryptography key 2"]
pub mod crypto_key2;
#[doc = "CRYPTO_KEY3 (w) register accessor: Cryptography key 3\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_key3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_key3`]
module"]
pub type CRYPTO_KEY3 = crate::Reg<crypto_key3::CRYPTO_KEY3_SPEC>;
#[doc = "Cryptography key 3"]
pub mod crypto_key3;
#[doc = "CRYPTO_OUTPUT0 (rw) register accessor: Cryptography output 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crypto_output0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_output0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_output0`]
module"]
pub type CRYPTO_OUTPUT0 = crate::Reg<crypto_output0::CRYPTO_OUTPUT0_SPEC>;
#[doc = "Cryptography output 0"]
pub mod crypto_output0;
#[doc = "CRYPTO_OUTPUT1 (rw) register accessor: Cryptography output 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crypto_output1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_output1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_output1`]
module"]
pub type CRYPTO_OUTPUT1 = crate::Reg<crypto_output1::CRYPTO_OUTPUT1_SPEC>;
#[doc = "Cryptography output 1"]
pub mod crypto_output1;
#[doc = "CRYPTO_OUTPUT2 (rw) register accessor: Cryptography output 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crypto_output2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_output2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_output2`]
module"]
pub type CRYPTO_OUTPUT2 = crate::Reg<crypto_output2::CRYPTO_OUTPUT2_SPEC>;
#[doc = "Cryptography output 2"]
pub mod crypto_output2;
#[doc = "CRYPTO_OUTPUT3 (rw) register accessor: Cryptography output 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crypto_output3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_output3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_output3`]
module"]
pub type CRYPTO_OUTPUT3 = crate::Reg<crypto_output3::CRYPTO_OUTPUT3_SPEC>;
#[doc = "Cryptography output 3"]
pub mod crypto_output3;
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
#[doc = "Device (only used in XIP mode)"]
pub use self::device::DEVICE;
#[doc = r"Cluster"]
#[doc = "Device (only used in XIP mode)"]
pub mod device;
