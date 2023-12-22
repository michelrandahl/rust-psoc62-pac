#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    prt: (),
    _reserved1: [u8; 0x2000],
    amux_split_ctl: [AMUX_SPLIT_CTL; 64],
    _reserved2: [u8; 0x0100],
    monitor_ctl_0: MONITOR_CTL_0,
    monitor_ctl_1: MONITOR_CTL_1,
    monitor_ctl_2: MONITOR_CTL_2,
    monitor_ctl_3: MONITOR_CTL_3,
    _reserved6: [u8; 0x30],
    alt_jtag_en: ALT_JTAG_EN,
}
impl RegisterBlock {
    #[doc = "0x00..0x78 - HSIOM port registers"]
    #[inline(always)]
    pub const fn prt(&self, n: usize) -> &PRT {
        #[allow(clippy::no_effect)]
        [(); 15][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(0).add(16 * n).cast() }
    }
    #[doc = "0x2000..0x2100 - AMUX splitter cell control"]
    #[inline(always)]
    pub const fn amux_split_ctl(&self, n: usize) -> &AMUX_SPLIT_CTL {
        &self.amux_split_ctl[n]
    }
    #[doc = "0x2200 - Power/Ground Monitor cell control 0"]
    #[inline(always)]
    pub const fn monitor_ctl_0(&self) -> &MONITOR_CTL_0 {
        &self.monitor_ctl_0
    }
    #[doc = "0x2204 - Power/Ground Monitor cell control 1"]
    #[inline(always)]
    pub const fn monitor_ctl_1(&self) -> &MONITOR_CTL_1 {
        &self.monitor_ctl_1
    }
    #[doc = "0x2208 - Power/Ground Monitor cell control 2"]
    #[inline(always)]
    pub const fn monitor_ctl_2(&self) -> &MONITOR_CTL_2 {
        &self.monitor_ctl_2
    }
    #[doc = "0x220c - Power/Ground Monitor cell control 3"]
    #[inline(always)]
    pub const fn monitor_ctl_3(&self) -> &MONITOR_CTL_3 {
        &self.monitor_ctl_3
    }
    #[doc = "0x2240 - Alternate JTAG IF selection register"]
    #[inline(always)]
    pub const fn alt_jtag_en(&self) -> &ALT_JTAG_EN {
        &self.alt_jtag_en
    }
}
#[doc = "HSIOM port registers"]
pub use self::prt::PRT;
#[doc = r"Cluster"]
#[doc = "HSIOM port registers"]
pub mod prt;
#[doc = "AMUX_SPLIT_CTL (rw) register accessor: AMUX splitter cell control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`amux_split_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`amux_split_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amux_split_ctl`]
module"]
pub type AMUX_SPLIT_CTL = crate::Reg<amux_split_ctl::AMUX_SPLIT_CTL_SPEC>;
#[doc = "AMUX splitter cell control"]
pub mod amux_split_ctl;
#[doc = "MONITOR_CTL_0 (rw) register accessor: Power/Ground Monitor cell control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`monitor_ctl_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`monitor_ctl_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monitor_ctl_0`]
module"]
pub type MONITOR_CTL_0 = crate::Reg<monitor_ctl_0::MONITOR_CTL_0_SPEC>;
#[doc = "Power/Ground Monitor cell control 0"]
pub mod monitor_ctl_0;
#[doc = "MONITOR_CTL_1 (rw) register accessor: Power/Ground Monitor cell control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`monitor_ctl_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`monitor_ctl_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monitor_ctl_1`]
module"]
pub type MONITOR_CTL_1 = crate::Reg<monitor_ctl_1::MONITOR_CTL_1_SPEC>;
#[doc = "Power/Ground Monitor cell control 1"]
pub mod monitor_ctl_1;
#[doc = "MONITOR_CTL_2 (rw) register accessor: Power/Ground Monitor cell control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`monitor_ctl_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`monitor_ctl_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monitor_ctl_2`]
module"]
pub type MONITOR_CTL_2 = crate::Reg<monitor_ctl_2::MONITOR_CTL_2_SPEC>;
#[doc = "Power/Ground Monitor cell control 2"]
pub mod monitor_ctl_2;
#[doc = "MONITOR_CTL_3 (rw) register accessor: Power/Ground Monitor cell control 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`monitor_ctl_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`monitor_ctl_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monitor_ctl_3`]
module"]
pub type MONITOR_CTL_3 = crate::Reg<monitor_ctl_3::MONITOR_CTL_3_SPEC>;
#[doc = "Power/Ground Monitor cell control 3"]
pub mod monitor_ctl_3;
#[doc = "ALT_JTAG_EN (rw) register accessor: Alternate JTAG IF selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alt_jtag_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alt_jtag_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alt_jtag_en`]
module"]
pub type ALT_JTAG_EN = crate::Reg<alt_jtag_en::ALT_JTAG_EN_SPEC>;
#[doc = "Alternate JTAG IF selection register"]
pub mod alt_jtag_en;
