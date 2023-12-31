#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctl: CTL,
    _reserved1: [u8; 0x0c],
    cmd: CMD,
    _reserved2: [u8; 0x0c],
    seq_default: SEQ_DEFAULT,
    _reserved3: [u8; 0x1c],
    seq_read_ctl_0: SEQ_READ_CTL_0,
    seq_read_ctl_1: SEQ_READ_CTL_1,
    seq_read_ctl_2: SEQ_READ_CTL_2,
    seq_read_ctl_3: SEQ_READ_CTL_3,
    seq_read_ctl_4: SEQ_READ_CTL_4,
    seq_read_ctl_5: SEQ_READ_CTL_5,
    _reserved9: [u8; 0x08],
    seq_program_ctl_0: SEQ_PROGRAM_CTL_0,
    seq_program_ctl_1: SEQ_PROGRAM_CTL_1,
    seq_program_ctl_2: SEQ_PROGRAM_CTL_2,
    seq_program_ctl_3: SEQ_PROGRAM_CTL_3,
    seq_program_ctl_4: SEQ_PROGRAM_CTL_4,
    seq_program_ctl_5: SEQ_PROGRAM_CTL_5,
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &CTL {
        &self.ctl
    }
    #[doc = "0x10 - Command"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x20 - Sequencer Default value"]
    #[inline(always)]
    pub const fn seq_default(&self) -> &SEQ_DEFAULT {
        &self.seq_default
    }
    #[doc = "0x40 - Sequencer read control 0"]
    #[inline(always)]
    pub const fn seq_read_ctl_0(&self) -> &SEQ_READ_CTL_0 {
        &self.seq_read_ctl_0
    }
    #[doc = "0x44 - Sequencer read control 1"]
    #[inline(always)]
    pub const fn seq_read_ctl_1(&self) -> &SEQ_READ_CTL_1 {
        &self.seq_read_ctl_1
    }
    #[doc = "0x48 - Sequencer read control 2"]
    #[inline(always)]
    pub const fn seq_read_ctl_2(&self) -> &SEQ_READ_CTL_2 {
        &self.seq_read_ctl_2
    }
    #[doc = "0x4c - Sequencer read control 3"]
    #[inline(always)]
    pub const fn seq_read_ctl_3(&self) -> &SEQ_READ_CTL_3 {
        &self.seq_read_ctl_3
    }
    #[doc = "0x50 - Sequencer read control 4"]
    #[inline(always)]
    pub const fn seq_read_ctl_4(&self) -> &SEQ_READ_CTL_4 {
        &self.seq_read_ctl_4
    }
    #[doc = "0x54 - Sequencer read control 5"]
    #[inline(always)]
    pub const fn seq_read_ctl_5(&self) -> &SEQ_READ_CTL_5 {
        &self.seq_read_ctl_5
    }
    #[doc = "0x60 - Sequencer program control 0"]
    #[inline(always)]
    pub const fn seq_program_ctl_0(&self) -> &SEQ_PROGRAM_CTL_0 {
        &self.seq_program_ctl_0
    }
    #[doc = "0x64 - Sequencer program control 1"]
    #[inline(always)]
    pub const fn seq_program_ctl_1(&self) -> &SEQ_PROGRAM_CTL_1 {
        &self.seq_program_ctl_1
    }
    #[doc = "0x68 - Sequencer program control 2"]
    #[inline(always)]
    pub const fn seq_program_ctl_2(&self) -> &SEQ_PROGRAM_CTL_2 {
        &self.seq_program_ctl_2
    }
    #[doc = "0x6c - Sequencer program control 3"]
    #[inline(always)]
    pub const fn seq_program_ctl_3(&self) -> &SEQ_PROGRAM_CTL_3 {
        &self.seq_program_ctl_3
    }
    #[doc = "0x70 - Sequencer program control 4"]
    #[inline(always)]
    pub const fn seq_program_ctl_4(&self) -> &SEQ_PROGRAM_CTL_4 {
        &self.seq_program_ctl_4
    }
    #[doc = "0x74 - Sequencer program control 5"]
    #[inline(always)]
    pub const fn seq_program_ctl_5(&self) -> &SEQ_PROGRAM_CTL_5 {
        &self.seq_program_ctl_5
    }
}
#[doc = "CTL (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "CMD (rw) register accessor: Command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command"]
pub mod cmd;
#[doc = "SEQ_DEFAULT (rw) register accessor: Sequencer Default value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_default::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_default::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_default`]
module"]
pub type SEQ_DEFAULT = crate::Reg<seq_default::SEQ_DEFAULT_SPEC>;
#[doc = "Sequencer Default value"]
pub mod seq_default;
#[doc = "SEQ_READ_CTL_0 (rw) register accessor: Sequencer read control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_read_ctl_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_read_ctl_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_read_ctl_0`]
module"]
pub type SEQ_READ_CTL_0 = crate::Reg<seq_read_ctl_0::SEQ_READ_CTL_0_SPEC>;
#[doc = "Sequencer read control 0"]
pub mod seq_read_ctl_0;
#[doc = "SEQ_READ_CTL_1 (rw) register accessor: Sequencer read control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_read_ctl_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_read_ctl_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_read_ctl_1`]
module"]
pub type SEQ_READ_CTL_1 = crate::Reg<seq_read_ctl_1::SEQ_READ_CTL_1_SPEC>;
#[doc = "Sequencer read control 1"]
pub mod seq_read_ctl_1;
#[doc = "SEQ_READ_CTL_2 (rw) register accessor: Sequencer read control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_read_ctl_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_read_ctl_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_read_ctl_2`]
module"]
pub type SEQ_READ_CTL_2 = crate::Reg<seq_read_ctl_2::SEQ_READ_CTL_2_SPEC>;
#[doc = "Sequencer read control 2"]
pub mod seq_read_ctl_2;
#[doc = "SEQ_READ_CTL_3 (rw) register accessor: Sequencer read control 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_read_ctl_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_read_ctl_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_read_ctl_3`]
module"]
pub type SEQ_READ_CTL_3 = crate::Reg<seq_read_ctl_3::SEQ_READ_CTL_3_SPEC>;
#[doc = "Sequencer read control 3"]
pub mod seq_read_ctl_3;
#[doc = "SEQ_READ_CTL_4 (rw) register accessor: Sequencer read control 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_read_ctl_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_read_ctl_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_read_ctl_4`]
module"]
pub type SEQ_READ_CTL_4 = crate::Reg<seq_read_ctl_4::SEQ_READ_CTL_4_SPEC>;
#[doc = "Sequencer read control 4"]
pub mod seq_read_ctl_4;
#[doc = "SEQ_READ_CTL_5 (rw) register accessor: Sequencer read control 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_read_ctl_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_read_ctl_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_read_ctl_5`]
module"]
pub type SEQ_READ_CTL_5 = crate::Reg<seq_read_ctl_5::SEQ_READ_CTL_5_SPEC>;
#[doc = "Sequencer read control 5"]
pub mod seq_read_ctl_5;
#[doc = "SEQ_PROGRAM_CTL_0 (rw) register accessor: Sequencer program control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_program_ctl_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_program_ctl_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_program_ctl_0`]
module"]
pub type SEQ_PROGRAM_CTL_0 = crate::Reg<seq_program_ctl_0::SEQ_PROGRAM_CTL_0_SPEC>;
#[doc = "Sequencer program control 0"]
pub mod seq_program_ctl_0;
#[doc = "SEQ_PROGRAM_CTL_1 (rw) register accessor: Sequencer program control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_program_ctl_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_program_ctl_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_program_ctl_1`]
module"]
pub type SEQ_PROGRAM_CTL_1 = crate::Reg<seq_program_ctl_1::SEQ_PROGRAM_CTL_1_SPEC>;
#[doc = "Sequencer program control 1"]
pub mod seq_program_ctl_1;
#[doc = "SEQ_PROGRAM_CTL_2 (rw) register accessor: Sequencer program control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_program_ctl_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_program_ctl_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_program_ctl_2`]
module"]
pub type SEQ_PROGRAM_CTL_2 = crate::Reg<seq_program_ctl_2::SEQ_PROGRAM_CTL_2_SPEC>;
#[doc = "Sequencer program control 2"]
pub mod seq_program_ctl_2;
#[doc = "SEQ_PROGRAM_CTL_3 (rw) register accessor: Sequencer program control 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_program_ctl_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_program_ctl_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_program_ctl_3`]
module"]
pub type SEQ_PROGRAM_CTL_3 = crate::Reg<seq_program_ctl_3::SEQ_PROGRAM_CTL_3_SPEC>;
#[doc = "Sequencer program control 3"]
pub mod seq_program_ctl_3;
#[doc = "SEQ_PROGRAM_CTL_4 (rw) register accessor: Sequencer program control 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_program_ctl_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_program_ctl_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_program_ctl_4`]
module"]
pub type SEQ_PROGRAM_CTL_4 = crate::Reg<seq_program_ctl_4::SEQ_PROGRAM_CTL_4_SPEC>;
#[doc = "Sequencer program control 4"]
pub mod seq_program_ctl_4;
#[doc = "SEQ_PROGRAM_CTL_5 (rw) register accessor: Sequencer program control 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_program_ctl_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_program_ctl_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_program_ctl_5`]
module"]
pub type SEQ_PROGRAM_CTL_5 = crate::Reg<seq_program_ctl_5::SEQ_PROGRAM_CTL_5_SPEC>;
#[doc = "Sequencer program control 5"]
pub mod seq_program_ctl_5;
