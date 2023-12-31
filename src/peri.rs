#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0200],
    timeout_ctl: TIMEOUT_CTL,
    _reserved1: [u8; 0x1c],
    tr_cmd: TR_CMD,
    _reserved2: [u8; 0x01dc],
    div_cmd: DIV_CMD,
    _reserved3: [u8; 0x07fc],
    clock_ctl: [CLOCK_CTL; 256],
    div_8_ctl: [DIV_8_CTL; 256],
    div_16_ctl: [DIV_16_CTL; 256],
    div_16_5_ctl: [DIV_16_5_CTL; 256],
    div_24_5_ctl: [DIV_24_5_CTL; 255],
    _reserved8: [u8; 0x04],
    ecc_ctl: ECC_CTL,
    _reserved9: [u8; 0x1ffc],
    gr: (),
    _reserved10: [u8; 0x4000],
    tr_gr: [TR_GR; 10],
    _reserved11: [u8; 0x1800],
    tr_1to1_gr: [TR_1TO1_GR; 7],
}
impl RegisterBlock {
    #[doc = "0x200 - Timeout control"]
    #[inline(always)]
    pub const fn timeout_ctl(&self) -> &TIMEOUT_CTL {
        &self.timeout_ctl
    }
    #[doc = "0x220 - Trigger command"]
    #[inline(always)]
    pub const fn tr_cmd(&self) -> &TR_CMD {
        &self.tr_cmd
    }
    #[doc = "0x400 - Divider command"]
    #[inline(always)]
    pub const fn div_cmd(&self) -> &DIV_CMD {
        &self.div_cmd
    }
    #[doc = "0xc00..0x1000 - Clock control"]
    #[inline(always)]
    pub const fn clock_ctl(&self, n: usize) -> &CLOCK_CTL {
        &self.clock_ctl[n]
    }
    #[doc = "0x1000..0x1400 - Divider control (for 8.0 divider)"]
    #[inline(always)]
    pub const fn div_8_ctl(&self, n: usize) -> &DIV_8_CTL {
        &self.div_8_ctl[n]
    }
    #[doc = "0x1400..0x1800 - Divider control (for 16.0 divider)"]
    #[inline(always)]
    pub const fn div_16_ctl(&self, n: usize) -> &DIV_16_CTL {
        &self.div_16_ctl[n]
    }
    #[doc = "0x1800..0x1c00 - Divider control (for 16.5 divider)"]
    #[inline(always)]
    pub const fn div_16_5_ctl(&self, n: usize) -> &DIV_16_5_CTL {
        &self.div_16_5_ctl[n]
    }
    #[doc = "0x1c00..0x1ffc - Divider control (for 24.5 divider)"]
    #[inline(always)]
    pub const fn div_24_5_ctl(&self, n: usize) -> &DIV_24_5_CTL {
        &self.div_24_5_ctl[n]
    }
    #[doc = "0x2000 - ECC control"]
    #[inline(always)]
    pub const fn ecc_ctl(&self) -> &ECC_CTL {
        &self.ecc_ctl
    }
    #[doc = "0x4000..0x40dc - Peripheral group structure"]
    #[inline(always)]
    pub const fn gr(&self, n: usize) -> &GR {
        #[allow(clippy::no_effect)]
        [(); 11][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(16384)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "0x8000..0xa800 - Trigger group"]
    #[inline(always)]
    pub const fn tr_gr(&self, n: usize) -> &TR_GR {
        &self.tr_gr[n]
    }
    #[doc = "0xc000..0xdc00 - Trigger 1-to-1 group"]
    #[inline(always)]
    pub const fn tr_1to1_gr(&self, n: usize) -> &TR_1TO1_GR {
        &self.tr_1to1_gr[n]
    }
}
#[doc = "TIMEOUT_CTL (rw) register accessor: Timeout control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout_ctl`]
module"]
pub type TIMEOUT_CTL = crate::Reg<timeout_ctl::TIMEOUT_CTL_SPEC>;
#[doc = "Timeout control"]
pub mod timeout_ctl;
#[doc = "TR_CMD (rw) register accessor: Trigger command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_cmd`]
module"]
pub type TR_CMD = crate::Reg<tr_cmd::TR_CMD_SPEC>;
#[doc = "Trigger command"]
pub mod tr_cmd;
#[doc = "DIV_CMD (rw) register accessor: Divider command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div_cmd`]
module"]
pub type DIV_CMD = crate::Reg<div_cmd::DIV_CMD_SPEC>;
#[doc = "Divider command"]
pub mod div_cmd;
#[doc = "CLOCK_CTL (rw) register accessor: Clock control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_ctl`]
module"]
pub type CLOCK_CTL = crate::Reg<clock_ctl::CLOCK_CTL_SPEC>;
#[doc = "Clock control"]
pub mod clock_ctl;
#[doc = "DIV_8_CTL (rw) register accessor: Divider control (for 8.0 divider)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div_8_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div_8_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div_8_ctl`]
module"]
pub type DIV_8_CTL = crate::Reg<div_8_ctl::DIV_8_CTL_SPEC>;
#[doc = "Divider control (for 8.0 divider)"]
pub mod div_8_ctl;
#[doc = "DIV_16_CTL (rw) register accessor: Divider control (for 16.0 divider)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div_16_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div_16_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div_16_ctl`]
module"]
pub type DIV_16_CTL = crate::Reg<div_16_ctl::DIV_16_CTL_SPEC>;
#[doc = "Divider control (for 16.0 divider)"]
pub mod div_16_ctl;
#[doc = "DIV_16_5_CTL (rw) register accessor: Divider control (for 16.5 divider)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div_16_5_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div_16_5_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div_16_5_ctl`]
module"]
pub type DIV_16_5_CTL = crate::Reg<div_16_5_ctl::DIV_16_5_CTL_SPEC>;
#[doc = "Divider control (for 16.5 divider)"]
pub mod div_16_5_ctl;
#[doc = "DIV_24_5_CTL (rw) register accessor: Divider control (for 24.5 divider)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div_24_5_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div_24_5_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div_24_5_ctl`]
module"]
pub type DIV_24_5_CTL = crate::Reg<div_24_5_ctl::DIV_24_5_CTL_SPEC>;
#[doc = "Divider control (for 24.5 divider)"]
pub mod div_24_5_ctl;
#[doc = "ECC_CTL (rw) register accessor: ECC control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_ctl`]
module"]
pub type ECC_CTL = crate::Reg<ecc_ctl::ECC_CTL_SPEC>;
#[doc = "ECC control"]
pub mod ecc_ctl;
#[doc = "Peripheral group structure"]
pub use self::gr::GR;
#[doc = r"Cluster"]
#[doc = "Peripheral group structure"]
pub mod gr;
#[doc = "Trigger group"]
pub use self::tr_gr::TR_GR;
#[doc = r"Cluster"]
#[doc = "Trigger group"]
pub mod tr_gr;
#[doc = "Trigger 1-to-1 group"]
pub use self::tr_1to1_gr::TR_1TO1_GR;
#[doc = r"Cluster"]
#[doc = "Trigger 1-to-1 group"]
pub mod tr_1to1_gr;
