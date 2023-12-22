#[doc = r"Register block"]
#[repr(C)]
pub struct SMPU {
    ms0_ctl: MS0_CTL,
    ms1_ctl: MS1_CTL,
    ms2_ctl: MS2_CTL,
    ms3_ctl: MS3_CTL,
    ms4_ctl: MS4_CTL,
    ms5_ctl: MS5_CTL,
    ms6_ctl: MS6_CTL,
    ms7_ctl: MS7_CTL,
    ms8_ctl: MS8_CTL,
    ms9_ctl: MS9_CTL,
    ms10_ctl: MS10_CTL,
    ms11_ctl: MS11_CTL,
    ms12_ctl: MS12_CTL,
    ms13_ctl: MS13_CTL,
    ms14_ctl: MS14_CTL,
    ms15_ctl: MS15_CTL,
    _reserved16: [u8; 0x1fc0],
    smpu_struct: (),
}
impl SMPU {
    #[doc = "0x00 - Master 0 protection context control"]
    #[inline(always)]
    pub const fn ms0_ctl(&self) -> &MS0_CTL {
        &self.ms0_ctl
    }
    #[doc = "0x04 - Master 1 protection context control"]
    #[inline(always)]
    pub const fn ms1_ctl(&self) -> &MS1_CTL {
        &self.ms1_ctl
    }
    #[doc = "0x08 - Master 2 protection context control"]
    #[inline(always)]
    pub const fn ms2_ctl(&self) -> &MS2_CTL {
        &self.ms2_ctl
    }
    #[doc = "0x0c - Master 3 protection context control"]
    #[inline(always)]
    pub const fn ms3_ctl(&self) -> &MS3_CTL {
        &self.ms3_ctl
    }
    #[doc = "0x10 - Master 4 protection context control"]
    #[inline(always)]
    pub const fn ms4_ctl(&self) -> &MS4_CTL {
        &self.ms4_ctl
    }
    #[doc = "0x14 - Master 5 protection context control"]
    #[inline(always)]
    pub const fn ms5_ctl(&self) -> &MS5_CTL {
        &self.ms5_ctl
    }
    #[doc = "0x18 - Master 6 protection context control"]
    #[inline(always)]
    pub const fn ms6_ctl(&self) -> &MS6_CTL {
        &self.ms6_ctl
    }
    #[doc = "0x1c - Master 7 protection context control"]
    #[inline(always)]
    pub const fn ms7_ctl(&self) -> &MS7_CTL {
        &self.ms7_ctl
    }
    #[doc = "0x20 - Master 8 protection context control"]
    #[inline(always)]
    pub const fn ms8_ctl(&self) -> &MS8_CTL {
        &self.ms8_ctl
    }
    #[doc = "0x24 - Master 9 protection context control"]
    #[inline(always)]
    pub const fn ms9_ctl(&self) -> &MS9_CTL {
        &self.ms9_ctl
    }
    #[doc = "0x28 - Master 10 protection context control"]
    #[inline(always)]
    pub const fn ms10_ctl(&self) -> &MS10_CTL {
        &self.ms10_ctl
    }
    #[doc = "0x2c - Master 11 protection context control"]
    #[inline(always)]
    pub const fn ms11_ctl(&self) -> &MS11_CTL {
        &self.ms11_ctl
    }
    #[doc = "0x30 - Master 12 protection context control"]
    #[inline(always)]
    pub const fn ms12_ctl(&self) -> &MS12_CTL {
        &self.ms12_ctl
    }
    #[doc = "0x34 - Master 13 protection context control"]
    #[inline(always)]
    pub const fn ms13_ctl(&self) -> &MS13_CTL {
        &self.ms13_ctl
    }
    #[doc = "0x38 - Master 14 protection context control"]
    #[inline(always)]
    pub const fn ms14_ctl(&self) -> &MS14_CTL {
        &self.ms14_ctl
    }
    #[doc = "0x3c - Master 15 protection context control"]
    #[inline(always)]
    pub const fn ms15_ctl(&self) -> &MS15_CTL {
        &self.ms15_ctl
    }
    #[doc = "0x2000..0x2280 - SMPU structure"]
    #[inline(always)]
    pub const fn smpu_struct(&self, n: usize) -> &SMPU_STRUCT {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(8192)
                .add(64 * n)
                .cast()
        }
    }
}
#[doc = "MS0_CTL (rw) register accessor: Master 0 protection context control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms0_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms0_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms0_ctl`]
module"]
pub type MS0_CTL = crate::Reg<ms0_ctl::MS0_CTL_SPEC>;
#[doc = "Master 0 protection context control"]
pub mod ms0_ctl;
#[doc = "MS1_CTL (rw) register accessor: Master 1 protection context control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms1_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms1_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms1_ctl`]
module"]
pub type MS1_CTL = crate::Reg<ms1_ctl::MS1_CTL_SPEC>;
#[doc = "Master 1 protection context control"]
pub mod ms1_ctl;
#[doc = "MS2_CTL (rw) register accessor: Master 2 protection context control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms2_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms2_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms2_ctl`]
module"]
pub type MS2_CTL = crate::Reg<ms2_ctl::MS2_CTL_SPEC>;
#[doc = "Master 2 protection context control"]
pub mod ms2_ctl;
#[doc = "MS3_CTL (rw) register accessor: Master 3 protection context control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms3_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms3_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms3_ctl`]
module"]
pub type MS3_CTL = crate::Reg<ms3_ctl::MS3_CTL_SPEC>;
#[doc = "Master 3 protection context control"]
pub mod ms3_ctl;
#[doc = "MS4_CTL (rw) register accessor: Master 4 protection context control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms4_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms4_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms4_ctl`]
module"]
pub type MS4_CTL = crate::Reg<ms4_ctl::MS4_CTL_SPEC>;
#[doc = "Master 4 protection context control"]
pub mod ms4_ctl;
#[doc = "MS5_CTL (rw) register accessor: Master 5 protection context control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms5_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms5_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms5_ctl`]
module"]
pub type MS5_CTL = crate::Reg<ms5_ctl::MS5_CTL_SPEC>;
#[doc = "Master 5 protection context control"]
pub mod ms5_ctl;
#[doc = "MS6_CTL (rw) register accessor: Master 6 protection context control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms6_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms6_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms6_ctl`]
module"]
pub type MS6_CTL = crate::Reg<ms6_ctl::MS6_CTL_SPEC>;
#[doc = "Master 6 protection context control"]
pub mod ms6_ctl;
#[doc = "MS7_CTL (rw) register accessor: Master 7 protection context control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms7_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms7_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms7_ctl`]
module"]
pub type MS7_CTL = crate::Reg<ms7_ctl::MS7_CTL_SPEC>;
#[doc = "Master 7 protection context control"]
pub mod ms7_ctl;
#[doc = "MS8_CTL (rw) register accessor: Master 8 protection context control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms8_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms8_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms8_ctl`]
module"]
pub type MS8_CTL = crate::Reg<ms8_ctl::MS8_CTL_SPEC>;
#[doc = "Master 8 protection context control"]
pub mod ms8_ctl;
#[doc = "MS9_CTL (rw) register accessor: Master 9 protection context control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms9_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms9_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms9_ctl`]
module"]
pub type MS9_CTL = crate::Reg<ms9_ctl::MS9_CTL_SPEC>;
#[doc = "Master 9 protection context control"]
pub mod ms9_ctl;
#[doc = "MS10_CTL (rw) register accessor: Master 10 protection context control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms10_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms10_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms10_ctl`]
module"]
pub type MS10_CTL = crate::Reg<ms10_ctl::MS10_CTL_SPEC>;
#[doc = "Master 10 protection context control"]
pub mod ms10_ctl;
#[doc = "MS11_CTL (rw) register accessor: Master 11 protection context control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms11_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms11_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms11_ctl`]
module"]
pub type MS11_CTL = crate::Reg<ms11_ctl::MS11_CTL_SPEC>;
#[doc = "Master 11 protection context control"]
pub mod ms11_ctl;
#[doc = "MS12_CTL (rw) register accessor: Master 12 protection context control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms12_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms12_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms12_ctl`]
module"]
pub type MS12_CTL = crate::Reg<ms12_ctl::MS12_CTL_SPEC>;
#[doc = "Master 12 protection context control"]
pub mod ms12_ctl;
#[doc = "MS13_CTL (rw) register accessor: Master 13 protection context control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms13_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms13_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms13_ctl`]
module"]
pub type MS13_CTL = crate::Reg<ms13_ctl::MS13_CTL_SPEC>;
#[doc = "Master 13 protection context control"]
pub mod ms13_ctl;
#[doc = "MS14_CTL (rw) register accessor: Master 14 protection context control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms14_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms14_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms14_ctl`]
module"]
pub type MS14_CTL = crate::Reg<ms14_ctl::MS14_CTL_SPEC>;
#[doc = "Master 14 protection context control"]
pub mod ms14_ctl;
#[doc = "MS15_CTL (rw) register accessor: Master 15 protection context control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms15_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms15_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms15_ctl`]
module"]
pub type MS15_CTL = crate::Reg<ms15_ctl::MS15_CTL_SPEC>;
#[doc = "Master 15 protection context control"]
pub mod ms15_ctl;
#[doc = "SMPU structure"]
pub use self::smpu_struct::SMPU_STRUCT;
#[doc = r"Cluster"]
#[doc = "SMPU structure"]
pub mod smpu_struct;
