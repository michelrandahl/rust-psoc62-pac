#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    intr_cause: INTR_CAUSE,
    _reserved1: [u8; 0x0dfc],
    aref: AREF,
    _reserved2: [u8; 0xfc],
    vref_trim0: VREF_TRIM0,
    vref_trim1: VREF_TRIM1,
    vref_trim2: VREF_TRIM2,
    vref_trim3: VREF_TRIM3,
    iztat_trim0: IZTAT_TRIM0,
    iztat_trim1: IZTAT_TRIM1,
    iptat_trim0: IPTAT_TRIM0,
    ictat_trim0: ICTAT_TRIM0,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt cause register"]
    #[inline(always)]
    pub const fn intr_cause(&self) -> &INTR_CAUSE {
        &self.intr_cause
    }
    #[doc = "0xe00 - AREF configuration"]
    #[inline(always)]
    pub const fn aref(&self) -> &AREF {
        &self.aref
    }
    #[doc = "0xf00 - VREF Trim bits"]
    #[inline(always)]
    pub const fn vref_trim0(&self) -> &VREF_TRIM0 {
        &self.vref_trim0
    }
    #[doc = "0xf04 - VREF Trim bits"]
    #[inline(always)]
    pub const fn vref_trim1(&self) -> &VREF_TRIM1 {
        &self.vref_trim1
    }
    #[doc = "0xf08 - VREF Trim bits"]
    #[inline(always)]
    pub const fn vref_trim2(&self) -> &VREF_TRIM2 {
        &self.vref_trim2
    }
    #[doc = "0xf0c - VREF Trim bits"]
    #[inline(always)]
    pub const fn vref_trim3(&self) -> &VREF_TRIM3 {
        &self.vref_trim3
    }
    #[doc = "0xf10 - IZTAT Trim bits"]
    #[inline(always)]
    pub const fn iztat_trim0(&self) -> &IZTAT_TRIM0 {
        &self.iztat_trim0
    }
    #[doc = "0xf14 - IZTAT Trim bits"]
    #[inline(always)]
    pub const fn iztat_trim1(&self) -> &IZTAT_TRIM1 {
        &self.iztat_trim1
    }
    #[doc = "0xf18 - IPTAT Trim bits"]
    #[inline(always)]
    pub const fn iptat_trim0(&self) -> &IPTAT_TRIM0 {
        &self.iptat_trim0
    }
    #[doc = "0xf1c - ICTAT Trim bits"]
    #[inline(always)]
    pub const fn ictat_trim0(&self) -> &ICTAT_TRIM0 {
        &self.ictat_trim0
    }
}
#[doc = "INTR_CAUSE (r) register accessor: Interrupt cause register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_cause::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cause`]
module"]
pub type INTR_CAUSE = crate::Reg<intr_cause::INTR_CAUSE_SPEC>;
#[doc = "Interrupt cause register"]
pub mod intr_cause;
#[doc = "AREF configuration"]
pub use self::aref::AREF;
#[doc = r"Cluster"]
#[doc = "AREF configuration"]
pub mod aref;
#[doc = "VREF_TRIM0 (rw) register accessor: VREF Trim bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vref_trim0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vref_trim0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vref_trim0`]
module"]
pub type VREF_TRIM0 = crate::Reg<vref_trim0::VREF_TRIM0_SPEC>;
#[doc = "VREF Trim bits"]
pub mod vref_trim0;
#[doc = "VREF_TRIM1 (rw) register accessor: VREF Trim bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vref_trim1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vref_trim1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vref_trim1`]
module"]
pub type VREF_TRIM1 = crate::Reg<vref_trim1::VREF_TRIM1_SPEC>;
#[doc = "VREF Trim bits"]
pub mod vref_trim1;
#[doc = "VREF_TRIM2 (rw) register accessor: VREF Trim bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vref_trim2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vref_trim2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vref_trim2`]
module"]
pub type VREF_TRIM2 = crate::Reg<vref_trim2::VREF_TRIM2_SPEC>;
#[doc = "VREF Trim bits"]
pub mod vref_trim2;
#[doc = "VREF_TRIM3 (rw) register accessor: VREF Trim bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vref_trim3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vref_trim3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vref_trim3`]
module"]
pub type VREF_TRIM3 = crate::Reg<vref_trim3::VREF_TRIM3_SPEC>;
#[doc = "VREF Trim bits"]
pub mod vref_trim3;
#[doc = "IZTAT_TRIM0 (rw) register accessor: IZTAT Trim bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iztat_trim0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iztat_trim0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iztat_trim0`]
module"]
pub type IZTAT_TRIM0 = crate::Reg<iztat_trim0::IZTAT_TRIM0_SPEC>;
#[doc = "IZTAT Trim bits"]
pub mod iztat_trim0;
#[doc = "IZTAT_TRIM1 (rw) register accessor: IZTAT Trim bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iztat_trim1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iztat_trim1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iztat_trim1`]
module"]
pub type IZTAT_TRIM1 = crate::Reg<iztat_trim1::IZTAT_TRIM1_SPEC>;
#[doc = "IZTAT Trim bits"]
pub mod iztat_trim1;
#[doc = "IPTAT_TRIM0 (rw) register accessor: IPTAT Trim bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iptat_trim0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iptat_trim0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iptat_trim0`]
module"]
pub type IPTAT_TRIM0 = crate::Reg<iptat_trim0::IPTAT_TRIM0_SPEC>;
#[doc = "IPTAT Trim bits"]
pub mod iptat_trim0;
#[doc = "ICTAT_TRIM0 (rw) register accessor: ICTAT Trim bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ictat_trim0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ictat_trim0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ictat_trim0`]
module"]
pub type ICTAT_TRIM0 = crate::Reg<ictat_trim0::ICTAT_TRIM0_SPEC>;
#[doc = "ICTAT Trim bits"]
pub mod ictat_trim0;
