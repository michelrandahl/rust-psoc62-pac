#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    prt: (),
    _reserved1: [u8; 0x4000],
    intr_cause0: INTR_CAUSE0,
    intr_cause1: INTR_CAUSE1,
    intr_cause2: INTR_CAUSE2,
    intr_cause3: INTR_CAUSE3,
    vdd_active: VDD_ACTIVE,
    vdd_intr: VDD_INTR,
    vdd_intr_mask: VDD_INTR_MASK,
    vdd_intr_masked: VDD_INTR_MASKED,
    vdd_intr_set: VDD_INTR_SET,
}
impl RegisterBlock {
    #[doc = "0x00..0x564 - GPIO port registers"]
    #[inline(always)]
    pub const fn prt(&self, n: usize) -> &PRT {
        #[allow(clippy::no_effect)]
        [(); 15][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(0)
                .add(128 * n)
                .cast()
        }
    }
    #[doc = "0x4000 - Interrupt port cause register 0"]
    #[inline(always)]
    pub const fn intr_cause0(&self) -> &INTR_CAUSE0 {
        &self.intr_cause0
    }
    #[doc = "0x4004 - Interrupt port cause register 1"]
    #[inline(always)]
    pub const fn intr_cause1(&self) -> &INTR_CAUSE1 {
        &self.intr_cause1
    }
    #[doc = "0x4008 - Interrupt port cause register 2"]
    #[inline(always)]
    pub const fn intr_cause2(&self) -> &INTR_CAUSE2 {
        &self.intr_cause2
    }
    #[doc = "0x400c - Interrupt port cause register 3"]
    #[inline(always)]
    pub const fn intr_cause3(&self) -> &INTR_CAUSE3 {
        &self.intr_cause3
    }
    #[doc = "0x4010 - Extern power supply detection register"]
    #[inline(always)]
    pub const fn vdd_active(&self) -> &VDD_ACTIVE {
        &self.vdd_active
    }
    #[doc = "0x4014 - Supply detection interrupt register"]
    #[inline(always)]
    pub const fn vdd_intr(&self) -> &VDD_INTR {
        &self.vdd_intr
    }
    #[doc = "0x4018 - Supply detection interrupt mask register"]
    #[inline(always)]
    pub const fn vdd_intr_mask(&self) -> &VDD_INTR_MASK {
        &self.vdd_intr_mask
    }
    #[doc = "0x401c - Supply detection interrupt masked register"]
    #[inline(always)]
    pub const fn vdd_intr_masked(&self) -> &VDD_INTR_MASKED {
        &self.vdd_intr_masked
    }
    #[doc = "0x4020 - Supply detection interrupt set register"]
    #[inline(always)]
    pub const fn vdd_intr_set(&self) -> &VDD_INTR_SET {
        &self.vdd_intr_set
    }
}
#[doc = "GPIO port registers"]
pub use self::prt::PRT;
#[doc = r"Cluster"]
#[doc = "GPIO port registers"]
pub mod prt;
#[doc = "INTR_CAUSE0 (r) register accessor: Interrupt port cause register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_cause0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cause0`]
module"]
pub type INTR_CAUSE0 = crate::Reg<intr_cause0::INTR_CAUSE0_SPEC>;
#[doc = "Interrupt port cause register 0"]
pub mod intr_cause0;
#[doc = "INTR_CAUSE1 (r) register accessor: Interrupt port cause register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_cause1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cause1`]
module"]
pub type INTR_CAUSE1 = crate::Reg<intr_cause1::INTR_CAUSE1_SPEC>;
#[doc = "Interrupt port cause register 1"]
pub mod intr_cause1;
#[doc = "INTR_CAUSE2 (r) register accessor: Interrupt port cause register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_cause2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cause2`]
module"]
pub type INTR_CAUSE2 = crate::Reg<intr_cause2::INTR_CAUSE2_SPEC>;
#[doc = "Interrupt port cause register 2"]
pub mod intr_cause2;
#[doc = "INTR_CAUSE3 (r) register accessor: Interrupt port cause register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_cause3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cause3`]
module"]
pub type INTR_CAUSE3 = crate::Reg<intr_cause3::INTR_CAUSE3_SPEC>;
#[doc = "Interrupt port cause register 3"]
pub mod intr_cause3;
#[doc = "VDD_ACTIVE (r) register accessor: Extern power supply detection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdd_active::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdd_active`]
module"]
pub type VDD_ACTIVE = crate::Reg<vdd_active::VDD_ACTIVE_SPEC>;
#[doc = "Extern power supply detection register"]
pub mod vdd_active;
#[doc = "VDD_INTR (rw) register accessor: Supply detection interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdd_intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vdd_intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdd_intr`]
module"]
pub type VDD_INTR = crate::Reg<vdd_intr::VDD_INTR_SPEC>;
#[doc = "Supply detection interrupt register"]
pub mod vdd_intr;
#[doc = "VDD_INTR_MASK (rw) register accessor: Supply detection interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdd_intr_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vdd_intr_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdd_intr_mask`]
module"]
pub type VDD_INTR_MASK = crate::Reg<vdd_intr_mask::VDD_INTR_MASK_SPEC>;
#[doc = "Supply detection interrupt mask register"]
pub mod vdd_intr_mask;
#[doc = "VDD_INTR_MASKED (r) register accessor: Supply detection interrupt masked register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdd_intr_masked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdd_intr_masked`]
module"]
pub type VDD_INTR_MASKED = crate::Reg<vdd_intr_masked::VDD_INTR_MASKED_SPEC>;
#[doc = "Supply detection interrupt masked register"]
pub mod vdd_intr_masked;
#[doc = "VDD_INTR_SET (rw) register accessor: Supply detection interrupt set register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdd_intr_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vdd_intr_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdd_intr_set`]
module"]
pub type VDD_INTR_SET = crate::Reg<vdd_intr_set::VDD_INTR_SET_SPEC>;
#[doc = "Supply detection interrupt set register"]
pub mod vdd_intr_set;
