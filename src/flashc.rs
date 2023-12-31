#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    flash_ctl: FLASH_CTL,
    flash_pwr_ctl: FLASH_PWR_CTL,
    flash_cmd: FLASH_CMD,
    _reserved3: [u8; 0x0294],
    ecc_ctl: ECC_CTL,
    _reserved4: [u8; 0x0c],
    fm_sram_ecc_ctl0: FM_SRAM_ECC_CTL0,
    fm_sram_ecc_ctl1: FM_SRAM_ECC_CTL1,
    fm_sram_ecc_ctl2: FM_SRAM_ECC_CTL2,
    fm_sram_ecc_ctl3: FM_SRAM_ECC_CTL3,
    _reserved8: [u8; 0x0140],
    cm0_ca_ctl0: CM0_CA_CTL0,
    cm0_ca_ctl1: CM0_CA_CTL1,
    cm0_ca_ctl2: CM0_CA_CTL2,
    _reserved11: [u8; 0x34],
    cm0_ca_status0: CM0_CA_STATUS0,
    cm0_ca_status1: CM0_CA_STATUS1,
    cm0_ca_status2: CM0_CA_STATUS2,
    _reserved14: [u8; 0x14],
    cm0_status: CM0_STATUS,
    _reserved15: [u8; 0x1c],
    cm4_ca_ctl0: CM4_CA_CTL0,
    cm4_ca_ctl1: CM4_CA_CTL1,
    cm4_ca_ctl2: CM4_CA_CTL2,
    _reserved18: [u8; 0x34],
    cm4_ca_status0: CM4_CA_STATUS0,
    cm4_ca_status1: CM4_CA_STATUS1,
    cm4_ca_status2: CM4_CA_STATUS2,
    _reserved21: [u8; 0x14],
    cm4_status: CM4_STATUS,
    _reserved22: [u8; 0x1c],
    crypto_buff_ctl: CRYPTO_BUFF_CTL,
    _reserved23: [u8; 0x7c],
    dw0_buff_ctl: DW0_BUFF_CTL,
    _reserved24: [u8; 0x7c],
    dw1_buff_ctl: DW1_BUFF_CTL,
    _reserved25: [u8; 0x7c],
    dmac_buff_ctl: DMAC_BUFF_CTL,
    _reserved26: [u8; 0x7c],
    ext_ms0_buff_ctl: EXT_MS0_BUFF_CTL,
    _reserved27: [u8; 0x7c],
    ext_ms1_buff_ctl: EXT_MS1_BUFF_CTL,
    _reserved28: [u8; 0xe87c],
    fm_ctl: FM_CTL,
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn flash_ctl(&self) -> &FLASH_CTL {
        &self.flash_ctl
    }
    #[doc = "0x04 - Flash power control"]
    #[inline(always)]
    pub const fn flash_pwr_ctl(&self) -> &FLASH_PWR_CTL {
        &self.flash_pwr_ctl
    }
    #[doc = "0x08 - Command"]
    #[inline(always)]
    pub const fn flash_cmd(&self) -> &FLASH_CMD {
        &self.flash_cmd
    }
    #[doc = "0x2a0 - ECC control"]
    #[inline(always)]
    pub const fn ecc_ctl(&self) -> &ECC_CTL {
        &self.ecc_ctl
    }
    #[doc = "0x2b0 - eCT Flash SRAM ECC control 0"]
    #[inline(always)]
    pub const fn fm_sram_ecc_ctl0(&self) -> &FM_SRAM_ECC_CTL0 {
        &self.fm_sram_ecc_ctl0
    }
    #[doc = "0x2b4 - eCT Flash SRAM ECC control 1"]
    #[inline(always)]
    pub const fn fm_sram_ecc_ctl1(&self) -> &FM_SRAM_ECC_CTL1 {
        &self.fm_sram_ecc_ctl1
    }
    #[doc = "0x2b8 - eCT Flash SRAM ECC control 2"]
    #[inline(always)]
    pub const fn fm_sram_ecc_ctl2(&self) -> &FM_SRAM_ECC_CTL2 {
        &self.fm_sram_ecc_ctl2
    }
    #[doc = "0x2bc - eCT Flash SRAM ECC control 3"]
    #[inline(always)]
    pub const fn fm_sram_ecc_ctl3(&self) -> &FM_SRAM_ECC_CTL3 {
        &self.fm_sram_ecc_ctl3
    }
    #[doc = "0x400 - CM0+ cache control"]
    #[inline(always)]
    pub const fn cm0_ca_ctl0(&self) -> &CM0_CA_CTL0 {
        &self.cm0_ca_ctl0
    }
    #[doc = "0x404 - CM0+ cache control"]
    #[inline(always)]
    pub const fn cm0_ca_ctl1(&self) -> &CM0_CA_CTL1 {
        &self.cm0_ca_ctl1
    }
    #[doc = "0x408 - CM0+ cache control"]
    #[inline(always)]
    pub const fn cm0_ca_ctl2(&self) -> &CM0_CA_CTL2 {
        &self.cm0_ca_ctl2
    }
    #[doc = "0x440 - CM0+ cache status 0"]
    #[inline(always)]
    pub const fn cm0_ca_status0(&self) -> &CM0_CA_STATUS0 {
        &self.cm0_ca_status0
    }
    #[doc = "0x444 - CM0+ cache status 1"]
    #[inline(always)]
    pub const fn cm0_ca_status1(&self) -> &CM0_CA_STATUS1 {
        &self.cm0_ca_status1
    }
    #[doc = "0x448 - CM0+ cache status 2"]
    #[inline(always)]
    pub const fn cm0_ca_status2(&self) -> &CM0_CA_STATUS2 {
        &self.cm0_ca_status2
    }
    #[doc = "0x460 - CM0+ interface status"]
    #[inline(always)]
    pub const fn cm0_status(&self) -> &CM0_STATUS {
        &self.cm0_status
    }
    #[doc = "0x480 - CM4 cache control"]
    #[inline(always)]
    pub const fn cm4_ca_ctl0(&self) -> &CM4_CA_CTL0 {
        &self.cm4_ca_ctl0
    }
    #[doc = "0x484 - CM4 cache control"]
    #[inline(always)]
    pub const fn cm4_ca_ctl1(&self) -> &CM4_CA_CTL1 {
        &self.cm4_ca_ctl1
    }
    #[doc = "0x488 - CM4 cache control"]
    #[inline(always)]
    pub const fn cm4_ca_ctl2(&self) -> &CM4_CA_CTL2 {
        &self.cm4_ca_ctl2
    }
    #[doc = "0x4c0 - CM4 cache status 0"]
    #[inline(always)]
    pub const fn cm4_ca_status0(&self) -> &CM4_CA_STATUS0 {
        &self.cm4_ca_status0
    }
    #[doc = "0x4c4 - CM4 cache status 1"]
    #[inline(always)]
    pub const fn cm4_ca_status1(&self) -> &CM4_CA_STATUS1 {
        &self.cm4_ca_status1
    }
    #[doc = "0x4c8 - CM4 cache status 2"]
    #[inline(always)]
    pub const fn cm4_ca_status2(&self) -> &CM4_CA_STATUS2 {
        &self.cm4_ca_status2
    }
    #[doc = "0x4e0 - CM4 interface status"]
    #[inline(always)]
    pub const fn cm4_status(&self) -> &CM4_STATUS {
        &self.cm4_status
    }
    #[doc = "0x500 - Cryptography buffer control"]
    #[inline(always)]
    pub const fn crypto_buff_ctl(&self) -> &CRYPTO_BUFF_CTL {
        &self.crypto_buff_ctl
    }
    #[doc = "0x580 - Datawire 0 buffer control"]
    #[inline(always)]
    pub const fn dw0_buff_ctl(&self) -> &DW0_BUFF_CTL {
        &self.dw0_buff_ctl
    }
    #[doc = "0x600 - Datawire 1 buffer control"]
    #[inline(always)]
    pub const fn dw1_buff_ctl(&self) -> &DW1_BUFF_CTL {
        &self.dw1_buff_ctl
    }
    #[doc = "0x680 - DMA controller buffer control"]
    #[inline(always)]
    pub const fn dmac_buff_ctl(&self) -> &DMAC_BUFF_CTL {
        &self.dmac_buff_ctl
    }
    #[doc = "0x700 - External master 0 buffer control"]
    #[inline(always)]
    pub const fn ext_ms0_buff_ctl(&self) -> &EXT_MS0_BUFF_CTL {
        &self.ext_ms0_buff_ctl
    }
    #[doc = "0x780 - External master 1 buffer control"]
    #[inline(always)]
    pub const fn ext_ms1_buff_ctl(&self) -> &EXT_MS1_BUFF_CTL {
        &self.ext_ms1_buff_ctl
    }
    #[doc = "0xf000..0x10000 - Flash Macro Registers"]
    #[inline(always)]
    pub const fn fm_ctl(&self) -> &FM_CTL {
        &self.fm_ctl
    }
}
#[doc = "FLASH_CTL (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_ctl`]
module"]
pub type FLASH_CTL = crate::Reg<flash_ctl::FLASH_CTL_SPEC>;
#[doc = "Control"]
pub mod flash_ctl;
#[doc = "FLASH_PWR_CTL (rw) register accessor: Flash power control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_pwr_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_pwr_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_pwr_ctl`]
module"]
pub type FLASH_PWR_CTL = crate::Reg<flash_pwr_ctl::FLASH_PWR_CTL_SPEC>;
#[doc = "Flash power control"]
pub mod flash_pwr_ctl;
#[doc = "FLASH_CMD (rw) register accessor: Command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_cmd`]
module"]
pub type FLASH_CMD = crate::Reg<flash_cmd::FLASH_CMD_SPEC>;
#[doc = "Command"]
pub mod flash_cmd;
#[doc = "ECC_CTL (rw) register accessor: ECC control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_ctl`]
module"]
pub type ECC_CTL = crate::Reg<ecc_ctl::ECC_CTL_SPEC>;
#[doc = "ECC control"]
pub mod ecc_ctl;
#[doc = "FM_SRAM_ECC_CTL0 (rw) register accessor: eCT Flash SRAM ECC control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fm_sram_ecc_ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fm_sram_ecc_ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fm_sram_ecc_ctl0`]
module"]
pub type FM_SRAM_ECC_CTL0 = crate::Reg<fm_sram_ecc_ctl0::FM_SRAM_ECC_CTL0_SPEC>;
#[doc = "eCT Flash SRAM ECC control 0"]
pub mod fm_sram_ecc_ctl0;
#[doc = "FM_SRAM_ECC_CTL1 (rw) register accessor: eCT Flash SRAM ECC control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fm_sram_ecc_ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fm_sram_ecc_ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fm_sram_ecc_ctl1`]
module"]
pub type FM_SRAM_ECC_CTL1 = crate::Reg<fm_sram_ecc_ctl1::FM_SRAM_ECC_CTL1_SPEC>;
#[doc = "eCT Flash SRAM ECC control 1"]
pub mod fm_sram_ecc_ctl1;
#[doc = "FM_SRAM_ECC_CTL2 (r) register accessor: eCT Flash SRAM ECC control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fm_sram_ecc_ctl2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fm_sram_ecc_ctl2`]
module"]
pub type FM_SRAM_ECC_CTL2 = crate::Reg<fm_sram_ecc_ctl2::FM_SRAM_ECC_CTL2_SPEC>;
#[doc = "eCT Flash SRAM ECC control 2"]
pub mod fm_sram_ecc_ctl2;
#[doc = "FM_SRAM_ECC_CTL3 (rw) register accessor: eCT Flash SRAM ECC control 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fm_sram_ecc_ctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fm_sram_ecc_ctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fm_sram_ecc_ctl3`]
module"]
pub type FM_SRAM_ECC_CTL3 = crate::Reg<fm_sram_ecc_ctl3::FM_SRAM_ECC_CTL3_SPEC>;
#[doc = "eCT Flash SRAM ECC control 3"]
pub mod fm_sram_ecc_ctl3;
#[doc = "CM0_CA_CTL0 (rw) register accessor: CM0+ cache control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_ca_ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm0_ca_ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_ca_ctl0`]
module"]
pub type CM0_CA_CTL0 = crate::Reg<cm0_ca_ctl0::CM0_CA_CTL0_SPEC>;
#[doc = "CM0+ cache control"]
pub mod cm0_ca_ctl0;
#[doc = "CM0_CA_CTL1 (rw) register accessor: CM0+ cache control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_ca_ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm0_ca_ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_ca_ctl1`]
module"]
pub type CM0_CA_CTL1 = crate::Reg<cm0_ca_ctl1::CM0_CA_CTL1_SPEC>;
#[doc = "CM0+ cache control"]
pub mod cm0_ca_ctl1;
#[doc = "CM0_CA_CTL2 (rw) register accessor: CM0+ cache control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_ca_ctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm0_ca_ctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_ca_ctl2`]
module"]
pub type CM0_CA_CTL2 = crate::Reg<cm0_ca_ctl2::CM0_CA_CTL2_SPEC>;
#[doc = "CM0+ cache control"]
pub mod cm0_ca_ctl2;
#[doc = "CM0_CA_STATUS0 (r) register accessor: CM0+ cache status 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_ca_status0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_ca_status0`]
module"]
pub type CM0_CA_STATUS0 = crate::Reg<cm0_ca_status0::CM0_CA_STATUS0_SPEC>;
#[doc = "CM0+ cache status 0"]
pub mod cm0_ca_status0;
#[doc = "CM0_CA_STATUS1 (r) register accessor: CM0+ cache status 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_ca_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_ca_status1`]
module"]
pub type CM0_CA_STATUS1 = crate::Reg<cm0_ca_status1::CM0_CA_STATUS1_SPEC>;
#[doc = "CM0+ cache status 1"]
pub mod cm0_ca_status1;
#[doc = "CM0_CA_STATUS2 (r) register accessor: CM0+ cache status 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_ca_status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_ca_status2`]
module"]
pub type CM0_CA_STATUS2 = crate::Reg<cm0_ca_status2::CM0_CA_STATUS2_SPEC>;
#[doc = "CM0+ cache status 2"]
pub mod cm0_ca_status2;
#[doc = "CM0_STATUS (rw) register accessor: CM0+ interface status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm0_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_status`]
module"]
pub type CM0_STATUS = crate::Reg<cm0_status::CM0_STATUS_SPEC>;
#[doc = "CM0+ interface status"]
pub mod cm0_status;
#[doc = "CM4_CA_CTL0 (rw) register accessor: CM4 cache control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_ca_ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm4_ca_ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_ca_ctl0`]
module"]
pub type CM4_CA_CTL0 = crate::Reg<cm4_ca_ctl0::CM4_CA_CTL0_SPEC>;
#[doc = "CM4 cache control"]
pub mod cm4_ca_ctl0;
#[doc = "CM4_CA_CTL1 (rw) register accessor: CM4 cache control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_ca_ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm4_ca_ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_ca_ctl1`]
module"]
pub type CM4_CA_CTL1 = crate::Reg<cm4_ca_ctl1::CM4_CA_CTL1_SPEC>;
#[doc = "CM4 cache control"]
pub mod cm4_ca_ctl1;
#[doc = "CM4_CA_CTL2 (rw) register accessor: CM4 cache control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_ca_ctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm4_ca_ctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_ca_ctl2`]
module"]
pub type CM4_CA_CTL2 = crate::Reg<cm4_ca_ctl2::CM4_CA_CTL2_SPEC>;
#[doc = "CM4 cache control"]
pub mod cm4_ca_ctl2;
#[doc = "CM4_CA_STATUS0 (r) register accessor: CM4 cache status 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_ca_status0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_ca_status0`]
module"]
pub type CM4_CA_STATUS0 = crate::Reg<cm4_ca_status0::CM4_CA_STATUS0_SPEC>;
#[doc = "CM4 cache status 0"]
pub mod cm4_ca_status0;
#[doc = "CM4_CA_STATUS1 (r) register accessor: CM4 cache status 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_ca_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_ca_status1`]
module"]
pub type CM4_CA_STATUS1 = crate::Reg<cm4_ca_status1::CM4_CA_STATUS1_SPEC>;
#[doc = "CM4 cache status 1"]
pub mod cm4_ca_status1;
#[doc = "CM4_CA_STATUS2 (r) register accessor: CM4 cache status 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_ca_status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_ca_status2`]
module"]
pub type CM4_CA_STATUS2 = crate::Reg<cm4_ca_status2::CM4_CA_STATUS2_SPEC>;
#[doc = "CM4 cache status 2"]
pub mod cm4_ca_status2;
#[doc = "CM4_STATUS (rw) register accessor: CM4 interface status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm4_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_status`]
module"]
pub type CM4_STATUS = crate::Reg<cm4_status::CM4_STATUS_SPEC>;
#[doc = "CM4 interface status"]
pub mod cm4_status;
#[doc = "CRYPTO_BUFF_CTL (rw) register accessor: Cryptography buffer control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crypto_buff_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_buff_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_buff_ctl`]
module"]
pub type CRYPTO_BUFF_CTL = crate::Reg<crypto_buff_ctl::CRYPTO_BUFF_CTL_SPEC>;
#[doc = "Cryptography buffer control"]
pub mod crypto_buff_ctl;
#[doc = "DW0_BUFF_CTL (rw) register accessor: Datawire 0 buffer control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dw0_buff_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dw0_buff_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dw0_buff_ctl`]
module"]
pub type DW0_BUFF_CTL = crate::Reg<dw0_buff_ctl::DW0_BUFF_CTL_SPEC>;
#[doc = "Datawire 0 buffer control"]
pub mod dw0_buff_ctl;
#[doc = "DW1_BUFF_CTL (rw) register accessor: Datawire 1 buffer control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dw1_buff_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dw1_buff_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dw1_buff_ctl`]
module"]
pub type DW1_BUFF_CTL = crate::Reg<dw1_buff_ctl::DW1_BUFF_CTL_SPEC>;
#[doc = "Datawire 1 buffer control"]
pub mod dw1_buff_ctl;
#[doc = "DMAC_BUFF_CTL (rw) register accessor: DMA controller buffer control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_buff_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_buff_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_buff_ctl`]
module"]
pub type DMAC_BUFF_CTL = crate::Reg<dmac_buff_ctl::DMAC_BUFF_CTL_SPEC>;
#[doc = "DMA controller buffer control"]
pub mod dmac_buff_ctl;
#[doc = "EXT_MS0_BUFF_CTL (rw) register accessor: External master 0 buffer control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_ms0_buff_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_ms0_buff_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ms0_buff_ctl`]
module"]
pub type EXT_MS0_BUFF_CTL = crate::Reg<ext_ms0_buff_ctl::EXT_MS0_BUFF_CTL_SPEC>;
#[doc = "External master 0 buffer control"]
pub mod ext_ms0_buff_ctl;
#[doc = "EXT_MS1_BUFF_CTL (rw) register accessor: External master 1 buffer control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_ms1_buff_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_ms1_buff_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ms1_buff_ctl`]
module"]
pub type EXT_MS1_BUFF_CTL = crate::Reg<ext_ms1_buff_ctl::EXT_MS1_BUFF_CTL_SPEC>;
#[doc = "External master 1 buffer control"]
pub mod ext_ms1_buff_ctl;
#[doc = "Flash Macro Registers"]
pub use self::fm_ctl::FM_CTL;
#[doc = r"Cluster"]
#[doc = "Flash Macro Registers"]
pub mod fm_ctl;
