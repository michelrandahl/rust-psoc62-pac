#[doc = r"Register block"]
#[repr(C)]
pub struct FM_CTL {
    fm_ctl_reg: FM_CTL_REG,
    status: STATUS,
    fm_addr: FM_ADDR,
    bookmark: BOOKMARK,
    geometry: GEOMETRY,
    geometry_supervisory: GEOMETRY_SUPERVISORY,
    ana_ctl0: ANA_CTL0,
    ana_ctl1: ANA_CTL1,
    _reserved8: [u8; 0x08],
    wait_ctl: WAIT_CTL,
    _reserved9: [u8; 0x08],
    timer_clk_ctl: TIMER_CLK_CTL,
    timer_ctl: TIMER_CTL,
    aclk_ctl: ACLK_CTL,
    intr: INTR,
    intr_set: INTR_SET,
    intr_mask: INTR_MASK,
    intr_masked: INTR_MASKED,
    cal_ctl0: CAL_CTL0,
    cal_ctl1: CAL_CTL1,
    cal_ctl2: CAL_CTL2,
    cal_ctl3: CAL_CTL3,
    cal_ctl4: CAL_CTL4,
    cal_ctl5: CAL_CTL5,
    cal_ctl6: CAL_CTL6,
    cal_ctl7: CAL_CTL7,
    _reserved24: [u8; 0x10],
    red_ctl01: RED_CTL01,
    red_ctl23: RED_CTL23,
    red_ctl45: RED_CTL45,
    red_ctl67: RED_CTL67,
    red_ctl_sm01: RED_CTL_SM01,
    _reserved29: [u8; 0x04],
    rgrant_delay_prg: RGRANT_DELAY_PRG,
    _reserved30: [u8; 0x04],
    pw_seq12: PW_SEQ12,
    pw_seq23: PW_SEQ23,
    rgrant_scale_ers: RGRANT_SCALE_ERS,
    rgrant_delay_ers: RGRANT_DELAY_ERS,
    _reserved34: [u8; 0x074c],
    fm_pl_wrdata_all: FM_PL_WRDATA_ALL,
    fm_pl_data: [FM_PL_DATA; 256],
    fm_mem_data: [FM_MEM_DATA; 256],
}
impl FM_CTL {
    #[doc = "0x00 - Flash macro control"]
    #[inline(always)]
    pub const fn fm_ctl_reg(&self) -> &FM_CTL_REG {
        &self.fm_ctl_reg
    }
    #[doc = "0x04 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x08 - Flash macro address"]
    #[inline(always)]
    pub const fn fm_addr(&self) -> &FM_ADDR {
        &self.fm_addr
    }
    #[doc = "0x0c - Bookmark register - keeps the current FW HV seq"]
    #[inline(always)]
    pub const fn bookmark(&self) -> &BOOKMARK {
        &self.bookmark
    }
    #[doc = "0x10 - Regular flash geometry"]
    #[inline(always)]
    pub const fn geometry(&self) -> &GEOMETRY {
        &self.geometry
    }
    #[doc = "0x14 - Supervisory flash geometry"]
    #[inline(always)]
    pub const fn geometry_supervisory(&self) -> &GEOMETRY_SUPERVISORY {
        &self.geometry_supervisory
    }
    #[doc = "0x18 - Analog control 0"]
    #[inline(always)]
    pub const fn ana_ctl0(&self) -> &ANA_CTL0 {
        &self.ana_ctl0
    }
    #[doc = "0x1c - Analog control 1"]
    #[inline(always)]
    pub const fn ana_ctl1(&self) -> &ANA_CTL1 {
        &self.ana_ctl1
    }
    #[doc = "0x28 - Wait State control"]
    #[inline(always)]
    pub const fn wait_ctl(&self) -> &WAIT_CTL {
        &self.wait_ctl
    }
    #[doc = "0x34 - Timer prescaler (clk_t to timer clock frequency divider)"]
    #[inline(always)]
    pub const fn timer_clk_ctl(&self) -> &TIMER_CLK_CTL {
        &self.timer_clk_ctl
    }
    #[doc = "0x38 - Timer control"]
    #[inline(always)]
    pub const fn timer_ctl(&self) -> &TIMER_CTL {
        &self.timer_ctl
    }
    #[doc = "0x3c - MPCON clock"]
    #[inline(always)]
    pub const fn aclk_ctl(&self) -> &ACLK_CTL {
        &self.aclk_ctl
    }
    #[doc = "0x40 - Interrupt"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0x44 - Interrupt set"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &INTR_SET {
        &self.intr_set
    }
    #[doc = "0x48 - Interrupt mask"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &INTR_MASK {
        &self.intr_mask
    }
    #[doc = "0x4c - Interrupt masked"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &INTR_MASKED {
        &self.intr_masked
    }
    #[doc = "0x50 - Cal control BG LO trim bits"]
    #[inline(always)]
    pub const fn cal_ctl0(&self) -> &CAL_CTL0 {
        &self.cal_ctl0
    }
    #[doc = "0x54 - Cal control BG HI trim bits"]
    #[inline(always)]
    pub const fn cal_ctl1(&self) -> &CAL_CTL1 {
        &self.cal_ctl1
    }
    #[doc = "0x58 - Cal control BG LO&amp;HI trim bits"]
    #[inline(always)]
    pub const fn cal_ctl2(&self) -> &CAL_CTL2 {
        &self.cal_ctl2
    }
    #[doc = "0x5c - Cal control osc trim bits, idac, sdac, itim"]
    #[inline(always)]
    pub const fn cal_ctl3(&self) -> &CAL_CTL3 {
        &self.cal_ctl3
    }
    #[doc = "0x60 - Cal Control Vlim, SA, fdiv, reg_act"]
    #[inline(always)]
    pub const fn cal_ctl4(&self) -> &CAL_CTL4 {
        &self.cal_ctl4
    }
    #[doc = "0x64 - Cal control"]
    #[inline(always)]
    pub const fn cal_ctl5(&self) -> &CAL_CTL5 {
        &self.cal_ctl5
    }
    #[doc = "0x68 - SA trim LP/ULP"]
    #[inline(always)]
    pub const fn cal_ctl6(&self) -> &CAL_CTL6 {
        &self.cal_ctl6
    }
    #[doc = "0x6c - Cal control"]
    #[inline(always)]
    pub const fn cal_ctl7(&self) -> &CAL_CTL7 {
        &self.cal_ctl7
    }
    #[doc = "0x80 - Redundancy Control normal sectors 0,1"]
    #[inline(always)]
    pub const fn red_ctl01(&self) -> &RED_CTL01 {
        &self.red_ctl01
    }
    #[doc = "0x84 - Redundancy Control normal sectors 2,3"]
    #[inline(always)]
    pub const fn red_ctl23(&self) -> &RED_CTL23 {
        &self.red_ctl23
    }
    #[doc = "0x88 - Redundancy Control normal sectors 4,5"]
    #[inline(always)]
    pub const fn red_ctl45(&self) -> &RED_CTL45 {
        &self.red_ctl45
    }
    #[doc = "0x8c - Redundancy Control normal sectors 6,7"]
    #[inline(always)]
    pub const fn red_ctl67(&self) -> &RED_CTL67 {
        &self.red_ctl67
    }
    #[doc = "0x90 - Redundancy Control special sectors 0,1"]
    #[inline(always)]
    pub const fn red_ctl_sm01(&self) -> &RED_CTL_SM01 {
        &self.red_ctl_sm01
    }
    #[doc = "0x98 - R-grant delay for program"]
    #[inline(always)]
    pub const fn rgrant_delay_prg(&self) -> &RGRANT_DELAY_PRG {
        &self.rgrant_delay_prg
    }
    #[doc = "0xa0 - HV Pulse Delay for seq 1&amp;2 pre"]
    #[inline(always)]
    pub const fn pw_seq12(&self) -> &PW_SEQ12 {
        &self.pw_seq12
    }
    #[doc = "0xa4 - HV Pulse Delay for seq2 post &amp; seq3"]
    #[inline(always)]
    pub const fn pw_seq23(&self) -> &PW_SEQ23 {
        &self.pw_seq23
    }
    #[doc = "0xa8 - R-grant delay scale for erase"]
    #[inline(always)]
    pub const fn rgrant_scale_ers(&self) -> &RGRANT_SCALE_ERS {
        &self.rgrant_scale_ers
    }
    #[doc = "0xac - R-grant delay for erase"]
    #[inline(always)]
    pub const fn rgrant_delay_ers(&self) -> &RGRANT_DELAY_ERS {
        &self.rgrant_delay_ers
    }
    #[doc = "0x7fc - Flash macro write page latches all"]
    #[inline(always)]
    pub const fn fm_pl_wrdata_all(&self) -> &FM_PL_WRDATA_ALL {
        &self.fm_pl_wrdata_all
    }
    #[doc = "0x800..0xc00 - Flash macro Page Latches data"]
    #[inline(always)]
    pub const fn fm_pl_data(&self, n: usize) -> &FM_PL_DATA {
        &self.fm_pl_data[n]
    }
    #[doc = "0xc00..0x1000 - Flash macro memory sense amplifier and column decoder data"]
    #[inline(always)]
    pub const fn fm_mem_data(&self, n: usize) -> &FM_MEM_DATA {
        &self.fm_mem_data[n]
    }
}
#[doc = "FM_CTL_REG (rw) register accessor: Flash macro control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fm_ctl_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fm_ctl_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fm_ctl_reg`]
module"]
pub type FM_CTL_REG = crate::Reg<fm_ctl_reg::FM_CTL_REG_SPEC>;
#[doc = "Flash macro control"]
pub mod fm_ctl_reg;
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "FM_ADDR (rw) register accessor: Flash macro address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fm_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fm_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fm_addr`]
module"]
pub type FM_ADDR = crate::Reg<fm_addr::FM_ADDR_SPEC>;
#[doc = "Flash macro address"]
pub mod fm_addr;
#[doc = "BOOKMARK (rw) register accessor: Bookmark register - keeps the current FW HV seq\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bookmark::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bookmark::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bookmark`]
module"]
pub type BOOKMARK = crate::Reg<bookmark::BOOKMARK_SPEC>;
#[doc = "Bookmark register - keeps the current FW HV seq"]
pub mod bookmark;
#[doc = "GEOMETRY (r) register accessor: Regular flash geometry\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`geometry::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@geometry`]
module"]
pub type GEOMETRY = crate::Reg<geometry::GEOMETRY_SPEC>;
#[doc = "Regular flash geometry"]
pub mod geometry;
#[doc = "GEOMETRY_SUPERVISORY (r) register accessor: Supervisory flash geometry\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`geometry_supervisory::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@geometry_supervisory`]
module"]
pub type GEOMETRY_SUPERVISORY = crate::Reg<geometry_supervisory::GEOMETRY_SUPERVISORY_SPEC>;
#[doc = "Supervisory flash geometry"]
pub mod geometry_supervisory;
#[doc = "ANA_CTL0 (rw) register accessor: Analog control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_ctl0`]
module"]
pub type ANA_CTL0 = crate::Reg<ana_ctl0::ANA_CTL0_SPEC>;
#[doc = "Analog control 0"]
pub mod ana_ctl0;
#[doc = "ANA_CTL1 (rw) register accessor: Analog control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_ctl1`]
module"]
pub type ANA_CTL1 = crate::Reg<ana_ctl1::ANA_CTL1_SPEC>;
#[doc = "Analog control 1"]
pub mod ana_ctl1;
#[doc = "WAIT_CTL (rw) register accessor: Wait State control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wait_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wait_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wait_ctl`]
module"]
pub type WAIT_CTL = crate::Reg<wait_ctl::WAIT_CTL_SPEC>;
#[doc = "Wait State control"]
pub mod wait_ctl;
#[doc = "TIMER_CLK_CTL (rw) register accessor: Timer prescaler (clk_t to timer clock frequency divider)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_clk_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_clk_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_clk_ctl`]
module"]
pub type TIMER_CLK_CTL = crate::Reg<timer_clk_ctl::TIMER_CLK_CTL_SPEC>;
#[doc = "Timer prescaler (clk_t to timer clock frequency divider)"]
pub mod timer_clk_ctl;
#[doc = "TIMER_CTL (rw) register accessor: Timer control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_ctl`]
module"]
pub type TIMER_CTL = crate::Reg<timer_ctl::TIMER_CTL_SPEC>;
#[doc = "Timer control"]
pub mod timer_ctl;
#[doc = "ACLK_CTL (w) register accessor: MPCON clock\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aclk_ctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aclk_ctl`]
module"]
pub type ACLK_CTL = crate::Reg<aclk_ctl::ACLK_CTL_SPEC>;
#[doc = "MPCON clock"]
pub mod aclk_ctl;
#[doc = "INTR (rw) register accessor: Interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: Interrupt set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_set`]
module"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt set"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mask`]
module"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt mask"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: Interrupt masked\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_masked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_masked`]
module"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt masked"]
pub mod intr_masked;
#[doc = "CAL_CTL0 (rw) register accessor: Cal control BG LO trim bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal_ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal_ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal_ctl0`]
module"]
pub type CAL_CTL0 = crate::Reg<cal_ctl0::CAL_CTL0_SPEC>;
#[doc = "Cal control BG LO trim bits"]
pub mod cal_ctl0;
#[doc = "CAL_CTL1 (rw) register accessor: Cal control BG HI trim bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal_ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal_ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal_ctl1`]
module"]
pub type CAL_CTL1 = crate::Reg<cal_ctl1::CAL_CTL1_SPEC>;
#[doc = "Cal control BG HI trim bits"]
pub mod cal_ctl1;
#[doc = "CAL_CTL2 (rw) register accessor: Cal control BG LO&amp;HI trim bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal_ctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal_ctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal_ctl2`]
module"]
pub type CAL_CTL2 = crate::Reg<cal_ctl2::CAL_CTL2_SPEC>;
#[doc = "Cal control BG LO&amp;HI trim bits"]
pub mod cal_ctl2;
#[doc = "CAL_CTL3 (rw) register accessor: Cal control osc trim bits, idac, sdac, itim\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal_ctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal_ctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal_ctl3`]
module"]
pub type CAL_CTL3 = crate::Reg<cal_ctl3::CAL_CTL3_SPEC>;
#[doc = "Cal control osc trim bits, idac, sdac, itim"]
pub mod cal_ctl3;
#[doc = "CAL_CTL4 (rw) register accessor: Cal Control Vlim, SA, fdiv, reg_act\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal_ctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal_ctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal_ctl4`]
module"]
pub type CAL_CTL4 = crate::Reg<cal_ctl4::CAL_CTL4_SPEC>;
#[doc = "Cal Control Vlim, SA, fdiv, reg_act"]
pub mod cal_ctl4;
#[doc = "CAL_CTL5 (rw) register accessor: Cal control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal_ctl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal_ctl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal_ctl5`]
module"]
pub type CAL_CTL5 = crate::Reg<cal_ctl5::CAL_CTL5_SPEC>;
#[doc = "Cal control"]
pub mod cal_ctl5;
#[doc = "CAL_CTL6 (rw) register accessor: SA trim LP/ULP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal_ctl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal_ctl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal_ctl6`]
module"]
pub type CAL_CTL6 = crate::Reg<cal_ctl6::CAL_CTL6_SPEC>;
#[doc = "SA trim LP/ULP"]
pub mod cal_ctl6;
#[doc = "CAL_CTL7 (rw) register accessor: Cal control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal_ctl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal_ctl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal_ctl7`]
module"]
pub type CAL_CTL7 = crate::Reg<cal_ctl7::CAL_CTL7_SPEC>;
#[doc = "Cal control"]
pub mod cal_ctl7;
#[doc = "RED_CTL01 (rw) register accessor: Redundancy Control normal sectors 0,1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`red_ctl01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`red_ctl01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@red_ctl01`]
module"]
pub type RED_CTL01 = crate::Reg<red_ctl01::RED_CTL01_SPEC>;
#[doc = "Redundancy Control normal sectors 0,1"]
pub mod red_ctl01;
#[doc = "RED_CTL23 (rw) register accessor: Redundancy Control normal sectors 2,3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`red_ctl23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`red_ctl23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@red_ctl23`]
module"]
pub type RED_CTL23 = crate::Reg<red_ctl23::RED_CTL23_SPEC>;
#[doc = "Redundancy Control normal sectors 2,3"]
pub mod red_ctl23;
#[doc = "RED_CTL45 (rw) register accessor: Redundancy Control normal sectors 4,5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`red_ctl45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`red_ctl45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@red_ctl45`]
module"]
pub type RED_CTL45 = crate::Reg<red_ctl45::RED_CTL45_SPEC>;
#[doc = "Redundancy Control normal sectors 4,5"]
pub mod red_ctl45;
#[doc = "RED_CTL67 (rw) register accessor: Redundancy Control normal sectors 6,7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`red_ctl67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`red_ctl67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@red_ctl67`]
module"]
pub type RED_CTL67 = crate::Reg<red_ctl67::RED_CTL67_SPEC>;
#[doc = "Redundancy Control normal sectors 6,7"]
pub mod red_ctl67;
#[doc = "RED_CTL_SM01 (rw) register accessor: Redundancy Control special sectors 0,1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`red_ctl_sm01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`red_ctl_sm01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@red_ctl_sm01`]
module"]
pub type RED_CTL_SM01 = crate::Reg<red_ctl_sm01::RED_CTL_SM01_SPEC>;
#[doc = "Redundancy Control special sectors 0,1"]
pub mod red_ctl_sm01;
#[doc = "RGRANT_DELAY_PRG (rw) register accessor: R-grant delay for program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgrant_delay_prg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgrant_delay_prg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgrant_delay_prg`]
module"]
pub type RGRANT_DELAY_PRG = crate::Reg<rgrant_delay_prg::RGRANT_DELAY_PRG_SPEC>;
#[doc = "R-grant delay for program"]
pub mod rgrant_delay_prg;
#[doc = "PW_SEQ12 (rw) register accessor: HV Pulse Delay for seq 1&amp;2 pre\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pw_seq12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pw_seq12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pw_seq12`]
module"]
pub type PW_SEQ12 = crate::Reg<pw_seq12::PW_SEQ12_SPEC>;
#[doc = "HV Pulse Delay for seq 1&amp;2 pre"]
pub mod pw_seq12;
#[doc = "PW_SEQ23 (rw) register accessor: HV Pulse Delay for seq2 post &amp; seq3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pw_seq23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pw_seq23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pw_seq23`]
module"]
pub type PW_SEQ23 = crate::Reg<pw_seq23::PW_SEQ23_SPEC>;
#[doc = "HV Pulse Delay for seq2 post &amp; seq3"]
pub mod pw_seq23;
#[doc = "RGRANT_SCALE_ERS (rw) register accessor: R-grant delay scale for erase\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgrant_scale_ers::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgrant_scale_ers::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgrant_scale_ers`]
module"]
pub type RGRANT_SCALE_ERS = crate::Reg<rgrant_scale_ers::RGRANT_SCALE_ERS_SPEC>;
#[doc = "R-grant delay scale for erase"]
pub mod rgrant_scale_ers;
#[doc = "RGRANT_DELAY_ERS (rw) register accessor: R-grant delay for erase\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgrant_delay_ers::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgrant_delay_ers::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgrant_delay_ers`]
module"]
pub type RGRANT_DELAY_ERS = crate::Reg<rgrant_delay_ers::RGRANT_DELAY_ERS_SPEC>;
#[doc = "R-grant delay for erase"]
pub mod rgrant_delay_ers;
#[doc = "FM_PL_WRDATA_ALL (rw) register accessor: Flash macro write page latches all\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fm_pl_wrdata_all::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fm_pl_wrdata_all::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fm_pl_wrdata_all`]
module"]
pub type FM_PL_WRDATA_ALL = crate::Reg<fm_pl_wrdata_all::FM_PL_WRDATA_ALL_SPEC>;
#[doc = "Flash macro write page latches all"]
pub mod fm_pl_wrdata_all;
#[doc = "FM_PL_DATA (rw) register accessor: Flash macro Page Latches data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fm_pl_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fm_pl_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fm_pl_data`]
module"]
pub type FM_PL_DATA = crate::Reg<fm_pl_data::FM_PL_DATA_SPEC>;
#[doc = "Flash macro Page Latches data"]
pub mod fm_pl_data;
#[doc = "FM_MEM_DATA (r) register accessor: Flash macro memory sense amplifier and column decoder data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fm_mem_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fm_mem_data`]
module"]
pub type FM_MEM_DATA = crate::Reg<fm_mem_data::FM_MEM_DATA_SPEC>;
#[doc = "Flash macro memory sense amplifier and column decoder data"]
pub mod fm_mem_data;
