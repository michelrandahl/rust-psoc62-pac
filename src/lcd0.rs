#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    id: ID,
    divider: DIVIDER,
    control: CONTROL,
    _reserved3: [u8; 0xf4],
    data0: [DATA0; 8],
    _reserved4: [u8; 0xe0],
    data1: [DATA1; 8],
    _reserved5: [u8; 0xe0],
    data2: [DATA2; 8],
    _reserved6: [u8; 0xe0],
    data3: [DATA3; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - ID &amp; Revision"]
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
    #[doc = "0x04 - LCD Divider Register"]
    #[inline(always)]
    pub const fn divider(&self) -> &DIVIDER {
        &self.divider
    }
    #[doc = "0x08 - LCD Configuration Register"]
    #[inline(always)]
    pub const fn control(&self) -> &CONTROL {
        &self.control
    }
    #[doc = "0x100..0x120 - LCD Pin Data Registers"]
    #[inline(always)]
    pub const fn data0(&self, n: usize) -> &DATA0 {
        &self.data0[n]
    }
    #[doc = "0x200..0x220 - LCD Pin Data Registers"]
    #[inline(always)]
    pub const fn data1(&self, n: usize) -> &DATA1 {
        &self.data1[n]
    }
    #[doc = "0x300..0x320 - LCD Pin Data Registers"]
    #[inline(always)]
    pub const fn data2(&self, n: usize) -> &DATA2 {
        &self.data2[n]
    }
    #[doc = "0x400..0x420 - LCD Pin Data Registers"]
    #[inline(always)]
    pub const fn data3(&self, n: usize) -> &DATA3 {
        &self.data3[n]
    }
}
#[doc = "ID (r) register accessor: ID &amp; Revision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "ID &amp; Revision"]
pub mod id;
#[doc = "DIVIDER (rw) register accessor: LCD Divider Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divider::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divider::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divider`]
module"]
pub type DIVIDER = crate::Reg<divider::DIVIDER_SPEC>;
#[doc = "LCD Divider Register"]
pub mod divider;
#[doc = "CONTROL (rw) register accessor: LCD Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "LCD Configuration Register"]
pub mod control;
#[doc = "DATA0 (rw) register accessor: LCD Pin Data Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0`]
module"]
pub type DATA0 = crate::Reg<data0::DATA0_SPEC>;
#[doc = "LCD Pin Data Registers"]
pub mod data0;
#[doc = "DATA1 (rw) register accessor: LCD Pin Data Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1`]
module"]
pub type DATA1 = crate::Reg<data1::DATA1_SPEC>;
#[doc = "LCD Pin Data Registers"]
pub mod data1;
#[doc = "DATA2 (rw) register accessor: LCD Pin Data Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2`]
module"]
pub type DATA2 = crate::Reg<data2::DATA2_SPEC>;
#[doc = "LCD Pin Data Registers"]
pub mod data2;
#[doc = "DATA3 (rw) register accessor: LCD Pin Data Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data3`]
module"]
pub type DATA3 = crate::Reg<data3::DATA3_SPEC>;
#[doc = "LCD Pin Data Registers"]
pub mod data3;
